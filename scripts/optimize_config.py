#!/usr/bin/env python3

import click
import copy
import functools
import games
import helpers
import itertools
import json
import numpy
import operator
import os
import os.path
import results_stats
import scipy.optimize
import time


def to_int(v):
    return int(round(v))


def to_unsigned_int(v):
    return max(int(round(v)), 0)


def to_int_list(v):
    return [to_int(w) for w in v]


OPTIONS_TYPES = dict(
    unsigned_int=to_unsigned_int,
    int=to_int,
    float=float,
)


@click.command()
@click.option('--config_path', required=True, type=str)
@click.option('--options_index_path', required=True, type=str)
@click.option('--seeds_path', required=True, type=str)
@click.option('--opponent_type', default='Quickstart', type=click.Choice((
    'Tcp',
    'Quickstart',
    'Empty',
)))
@click.option('--local_runner_bin_path', required=True, type=str)
@click.option('--local_runner_config_template_path', required=True, type=str)
@click.option('--player_strategy_bin_path', required=True, type=str)
@click.option('--player_base_port', default=31010, type=int)
@click.option('--opponent_strategy_bin_path', type=str)
@click.option('--opponent_base_port', default=32010, type=int)
@click.option('--workers', default=1, type=int)
@click.option('--output_path', default=os.path.join(os.getcwd(), 'results'), type=str)
@click.option('--verbose', is_flag=True)
def main(**kwargs):
    run(**kwargs)


def run(config_path, options_index_path, seeds_path, opponent_type, local_runner_bin_path, local_runner_config_template_path,
        player_strategy_bin_path, player_base_port, opponent_strategy_bin_path, opponent_base_port, workers,
        output_path, verbose):
    config = helpers.read_json(config_path)
    print(config)
    options_index = helpers.read_json(options_index_path)
    seeds = [int(v) for v in helpers.read_lines(seeds_path) if v]
    local_runner_config_template = helpers.read_json(local_runner_config_template_path)
    print(options_index)
    initial = make_initial(config, options_index)
    iteration = [0]

    def function(args):
        current_config = copy.deepcopy(config)
        for k, v in options_index.items():
            current_config[k] = OPTIONS_TYPES[v['type']](args[v['index']])
        current_iteration = iteration[0]
        iteration[0] += 1
        score, wins, draws, losses = run_games(
            opponent_type=opponent_type,
            local_runner_bin_path=local_runner_bin_path,
            local_runner_config_template=local_runner_config_template,
            player_strategy_bin_path=player_strategy_bin_path,
            player_base_port=player_base_port,
            opponent_strategy_bin_path=opponent_strategy_bin_path,
            opponent_base_port=opponent_base_port,
            workers_number=workers,
            output_path=output_path,
            verbose=verbose,
            seeds=seeds,
            config=current_config,
            iteration=current_iteration,
        )
        result = score + wins * 3000
        print('iteration', current_iteration, 'score=%s' % score, 'wins=%s' % wins, 'draws=%s' % draws,
              'losses=%s' % losses, 'result=%s' % result, json.dumps(current_config))
        return -result

    result = minimize_ga(
        function=function,
        initial=numpy.array(initial),
    )
    for k, v in options_index.items():
        config[k] = OPTIONS_TYPES[v['type']](result[1][v['index']])
    print(json.dumps(config))


def minimize_powell(function, initial):
    return scipy.optimize.minimize(
        function,
        initial,
        method='Powell',
        options=dict(disp=True),
    )


def minimize_ga(function, initial, max_iterations=1000):
    minimum = (function(initial), initial)
    generator = generate_minimums(
        function=function,
        generation=[initial for _ in range(max(len(initial), 4))],
        minimum=minimum,
        max_generation_size=len(initial),
    )
    for _ in range(max_iterations):
        minimum, generation = next(generator)
        print(json.dumps(dict(
            minimum=dict(value=minimum[0], args=list(minimum[1])),
            generation_len=len(generation),
        )))
    return minimum


def generate_minimums(function, generation, minimum, max_generation_size):
    while True:
        mutations = [mutate(v) for v in generation]
        results = sorted(((function(v), v) for v in mutations), key=lambda v: v[0])
        if minimum[0] > results[0][0]:
            minimum = results[0]
        selected = [v for _, v in results[:get_size_of_selection(max_generation_size)]]
        generation = [crossover(pair[0], pair[1]) for pair in itertools.combinations(selected, 2)]
        yield minimum, generation


def mutate(args):
    return args + numpy.random.normal(numpy.zeros(len(args)), numpy.maximum(numpy.abs(args), numpy.ones(len(args))) / 2.0)


def crossover(left, right):
    result = numpy.copy(left)
    for n in range(len(right)):
        if numpy.random.randint(0, 1) == 1:
            result[n] = right[n]
    return result


def get_size_of_selection(max_combinations_number):
    result = 2
    combinations = 0
    while combinations < max_combinations_number:
        combinations = number_of_combinations(n=result, r=2)
        result += 1
    return result


def number_of_combinations(n, r):
    r = min(r, n - r)
    numer = functools.reduce(operator.mul, range(n, n - r, -1), 1)
    denom = functools.reduce(operator.mul, range(1, r + 1), 1)
    return numer // denom


def run_games(opponent_type, local_runner_bin_path, local_runner_config_template,
              player_strategy_bin_path, player_base_port, opponent_strategy_bin_path, opponent_base_port,
              workers_number, output_path, verbose, seeds, config, iteration):
    etc_path = os.path.join(output_path, 'etc')
    os.makedirs(etc_path, exist_ok=True)
    games_path = os.path.join(output_path, 'games', str(iteration))
    os.makedirs(games_path, exist_ok=True)
    strategy_config_path = os.path.abspath(os.path.join(etc_path, 'config.%s.json' % iteration))
    helpers.write_json(config, strategy_config_path)
    scheduler = games.Scheduler(workers_number=workers_number, verbose=verbose)
    number = 0
    for game_type in ('2x2+', '2x2', '2x1'):
        for seed in seeds:
            for swap in (False, True):
                scheduler.put_task(games.Task(
                    local_runner=games.LocalRunner(
                        bin_path=local_runner_bin_path,
                        config_template=local_runner_config_template,
                        opponent_type=opponent_type,
                        game_type=game_type,
                        swap=swap,
                        seed=seed,
                        output_path=os.path.join(games_path, '%s.%s' % (number, int(time.time() * 1e6))),
                    ),
                    player=games.TcpPlayer(
                        bin_path=player_strategy_bin_path,
                        base_port=player_base_port,
                        config_path=strategy_config_path,
                    ),
                    opponent=games.TcpPlayer(
                        bin_path=opponent_strategy_bin_path,
                        base_port=opponent_base_port,
                        config_path=None,
                    ),
                ))
                number += 1
    scheduler.start()
    scheduler.join()
    return get_games_result(games_path, player_base_port)


def get_games_result(games_path, player_base_port):
    score_diff = 0
    wins = 0
    draws = 0
    losses = 0
    for game in collect_games(games_path):
        player_key = 'Tcp_%s' % str(player_base_port)[:3]
        player_score = game['results'][player_key]['score']
        player_score = game['results'][player_key]['score']
        opponent_score = next(v for k, v in game['results'].items() if k != player_key)['score']
        score_diff += player_score - opponent_score
        wins += (player_score > opponent_score)
        draws += (player_score == opponent_score)
        losses += (player_score < opponent_score)
    return score_diff, wins, draws, losses


def collect_games(path):
    for dir_name in os.listdir(path):
        dir_path = os.path.join(path, dir_name)
        if os.path.exists(os.path.join(dir_path, 'config.json')):
            yield from results_stats.collect_data([os.path.join(path, v) for v in os.listdir(path)])
            return
        else:
            yield from collect_games(os.path.exists(os.path.join(path, dir_name)))


def make_initial(config, options_index):
    initial = [0] * len(options_index)
    for k, v in options_index.items():
        initial[v['index']] = config[k]
    return initial


if __name__ == '__main__':
    main()
