#!/usr/bin/env python3

import click
import json
import os.path
import subprocess
import sys
import time


@click.command()
@click.option('--aicup_bin', required=True, type=str)
@click.option('--dump_player_preview_bin', required=True, type=str)
@click.argument('paths', nargs=-1)
def main(aicup_bin, dump_player_preview_bin, paths):
    for game_path in paths:
        convert_log(game_path=game_path, aicup_bin=aicup_bin, dump_player_preview_bin=dump_player_preview_bin)


def convert_log(game_path, aicup_bin, dump_player_preview_bin):
    print(game_path)
    replay_path = os.path.join(game_path, 'replay.log')
    if not os.path.exists(replay_path):
        return
    config_path = os.path.join(game_path, 'config.json')
    result_path = os.path.join(game_path, 'result.json')
    config = read_config(config_path)
    log_paths = [os.path.join(game_path, f'{v}.log.json') for v in (0, 1)]
    ports = [33010, 33011]
    repeat_config = make_config(ports)
    repeat_config_path = os.path.join(game_path, 'repeat_config.json')
    write_json(repeat_config, repeat_config_path)
    with open(log_paths[0], 'w') as stream0, open(log_paths[1], 'w') as stream1:
        aicup = subprocess.Popen([
            aicup_bin,
            '--batch-mode',
            '--repeat', replay_path,
            '--config', repeat_config_path,
        ])
        time.sleep(0.5)
        dumps = [subprocess.Popen([dump_player_preview_bin, '127.0.0.1', str(port)], stdout=stream, stderr=subprocess.DEVNULL) for port, stream in zip(ports, [stream0, stream1])]
        aicup.wait()
        for dump in dumps:
            dump.terminate()


def write_json(data, path):
    with open(path, 'w') as stream:
        json.dump(data, stream, indent=4)


def read_config(path):
    with open(path) as stream:
        return json.load(stream)


def make_config(ports):
    return {
        'seed': None,
        'options_preset': {
            'Custom': {
                'level': 'Simple',
                'properties': None,
            }
        },
        'players': [make_player(port) for port in ports]
    }


def make_player(port):
    return {
        'Tcp': {
            'host': None,
            'port': port,
            'accept_timeout': None,
            'timeout': None,
            'token': None,
        }
    }


if __name__ == "__main__":
    main()
