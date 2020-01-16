#!/usr/bin/env python3

import click
import collections
import datetime
import helpers
import local_runner
import os
import os.path
import queue
import subprocess
import threading
import time
import traceback


@click.command()
@click.option('--opponent_type', default='Quickstart', type=click.Choice((
    'Tcp',
    'Quickstart',
    'Empty',
)))
@click.option('--game_type', default='2x1', type=click.Choice((
    '2x1',
    '2x2',
    '2x2+',
)))
@click.option('--local_runner_bin_path', required=True, type=str)
@click.option('--local_runner_config_template_path', required=True, type=str)
@click.option('--player_strategy_bin_path', required=True, type=str)
@click.option('--player_base_port', default=31010, type=int)
@click.option('--player_strategy_config_path', type=str)
@click.option('--opponent_strategy_bin_path', type=str)
@click.option('--opponent_base_port', default=32010, type=int)
@click.option('--opponent_strategy_config_path', type=str)
@click.option('--workers', default=1, type=int)
@click.option('--max_runs', default=2**64 - 1, type=int)
@click.option('--prefix', default='default')
@click.option('--output_path', default=os.path.join(os.getcwd(), 'results'), type=str)
@click.option('--verbose', is_flag=True)
def main(**kwargs):
    run(**kwargs)


def run(opponent_type, game_type, local_runner_bin_path, local_runner_config_template_path,
        player_strategy_bin_path, player_base_port, player_strategy_config_path,
        opponent_strategy_bin_path, opponent_base_port, opponent_strategy_config_path,
        workers, max_runs, prefix, output_path, verbose):
    local_runner_config_template = helpers.read_json(local_runner_config_template_path)
    session = '%s.%s.%s.%s' % (prefix, game_type, player_base_port, datetime.datetime.now().strftime('%Y-%m-%d_%H-%M-%S'))
    games_path = os.path.join(output_path, opponent_type, game_type, session)
    scheduler = Scheduler(workers_number=workers, verbose=verbose)
    for number in range(max_runs):
        scheduler.put_task(Task(
            local_runner=LocalRunner(
                bin_path=local_runner_bin_path,
                config_template=local_runner_config_template,
                opponent_type=opponent_type,
                game_type=game_type,
                swap=number % 2 == 1,
                seed=None,
                output_path=os.path.join(games_path, '%s.%s' % (number, int(time.time() * 1e6))),
            ),
            player=TcpPlayer(
                bin_path=player_strategy_bin_path,
                base_port=player_base_port,
                config_path=player_strategy_config_path,
            ),
            opponent=TcpPlayer(
                bin_path=opponent_strategy_bin_path,
                base_port=opponent_base_port,
                config_path=None,
            ),
        ))
    scheduler.start()
    scheduler.join()


Task = collections.namedtuple('Task', (
    'local_runner',
    'player',
    'opponent',
))


LocalRunner = collections.namedtuple('LocalRunner', (
    'bin_path',
    'config_template',
    'opponent_type',
    'game_type',
    'swap',
    'seed',
    'output_path',
))


TcpPlayer = collections.namedtuple('TcpPlayer', (
    'bin_path',
    'base_port',
    'config_path',
))


class Scheduler:
    def __init__(self, workers_number, verbose):
        self.__should_stop = threading.Event()
        self.__task_queue = queue.Queue()
        self.__workers = [
            threading.Thread(
                target=run_worker,
                kwargs=dict(
                    task_queue=self.__task_queue,
                    port_shift=n,
                    should_stop=self.__should_stop,
                    verbose=verbose,
                ),
            ) for n in range(workers_number)
        ]

    def start(self):
        for worker in self.__workers:
            worker.start()

    def put_task(self, task: Task):
        self.__task_queue.put(task)

    def join(self):
        try:
            self.__task_queue.join()
            for worker in self.__workers:
                worker.join()
        except:
            traceback.print_exc()
            self.__should_stop.set()
            for worker in self.__workers:
                worker.join()
            raise


def run_worker(task_queue, port_shift, should_stop, verbose):
    try:
        while not should_stop.is_set():
            task = task_queue.get_nowait()
            if task is None:
                break
            handle_task(task=task, port_shift=port_shift, verbose=verbose, should_stop=should_stop)
            task_queue.task_done()
    except queue.Empty:
        pass


def handle_task(task, port_shift, verbose, should_stop):
    player_port = task.player.base_port + port_shift
    opponent_port = task.opponent.base_port + port_shift
    local_runner_process = local_runner.run_game(
        opponent_type=task.local_runner.opponent_type,
        game_type=task.local_runner.game_type,
        player_port=player_port,
        opponent_port=opponent_port,
        seed=task.local_runner.seed,
        swap=task.local_runner.swap,
        bin_path=task.local_runner.bin_path,
        config_template=task.local_runner.config_template,
        verbose=verbose,
        output_path=task.local_runner.output_path,
    )
    player_thread = threading.Thread(
        target=run_player,
        kwargs=dict(
            bin_path=task.player.bin_path,
            port=player_port,
            config_path=task.player.config_path,
            verbose=verbose,
            should_stop=should_stop,
        ),
    )
    opponent_thread = threading.Thread(
        target=run_player,
        kwargs=dict(
            bin_path=task.opponent.bin_path,
            port=opponent_port,
            config_path=task.opponent.config_path,
            verbose=verbose,
            should_stop=should_stop,
        ),
    )
    time.sleep(0.2)
    player_thread.start()
    opponent_thread.start()
    wait_process(process=local_runner_process, should_stop=should_stop)


def run_player(bin_path, port, config_path, verbose, should_stop):
    env = dict()
    if config_path is not None:
        env['CONFIG'] = config_path
    args=[bin_path, '127.0.0.1', str(port)]
    if verbose:
        print('run', env, *args)
    fails = 0
    while not should_stop.is_set():
        try:
            process = subprocess.Popen(
                args=args,
                env=env,
                stdout=None if verbose else subprocess.DEVNULL,
                stderr=None if verbose else subprocess.DEVNULL,
            )
            if not wait_process(process=process, should_stop=should_stop):
                break
            if process.returncode == 0:
                break
            fails += 1
            time.sleep(min(1, fails * 0.1))
        except subprocess.TimeoutExpired:
            break


def wait_process(process, should_stop):
    start = time.time()
    while time.time() - start < 120 and not should_stop.is_set():
        try:
            process.wait(timeout=0.1)
            return True
        except subprocess.TimeoutExpired:
            pass
    process.terminate()
    return False


if __name__ == "__main__":
    main()
