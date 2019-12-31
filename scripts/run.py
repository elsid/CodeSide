#!/usr/bin/env python3

import sys
import json
import time
import pathlib
import os.path
import datetime
import subprocess
import click
import copy
import os


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
@click.option('--player_port', default=31010, type=int)
@click.option('--other_port', default=32010, type=int)
@click.option('--max_runs', default=2**64 - 1, type=int)
@click.option('--prefix', default='default')
@click.option('--aicup_bin', required=True, type=str)
@click.option('--config_template_path', required=True, type=str)
@click.option('--output_path', default=os.path.join(os.getcwd(), 'results'), type=str)
def main(opponent_type, game_type, player_port, other_port, max_runs, prefix, aicup_bin, config_template_path, output_path):
    config_template = read_json(config_template_path)
    session = '%s.%s.%s.%s' % (prefix, game_type, player_port, datetime.datetime.now().strftime('%Y-%m-%d_%H-%M-%S'))
    session_path = os.path.join(output_path, opponent_type, game_type, session)
    os.makedirs(session_path, exist_ok=False)
    for number in range(max_runs):
        run_game(
            number=number,
            session_path=session_path,
            opponent_type=opponent_type,
            game_type=game_type,
            player_port=player_port,
            other_port=other_port,
            aicup_bin=aicup_bin,
            config_template=config_template,
        )


def run_game(number, session_path, opponent_type, game_type, player_port, other_port, aicup_bin, config_template):
    game = '%s.%s' % (number, int(time.time() * 1e6))
    game_path = os.path.join(session_path, game)
    os.makedirs(game_path, exist_ok=False)
    config_path = os.path.join(game_path, 'config.json')
    result_path = os.path.join(game_path, 'result.json')
    replay_path = os.path.join(game_path, 'replay.log')
    swap = number % 2 == 1
    config = generate_config(
        config_template=config_template,
        opponent_type=opponent_type,
        game_type=game_type,
        player_port=player_port,
        other_port=other_port,
        swap=swap,
    )
    write_json(data=config, path=config_path)
    player_names = [other_port, player_port] if swap else [player_port, other_port]
    print(number, game_path, player_names)
    run_aicup(
        aicup_bin=aicup_bin,
        config_path=config_path,
        result_path=result_path,
        replay_path=replay_path,
        player_names=player_names,
    )


def run_aicup(aicup_bin, config_path, result_path, replay_path, player_names):
    subprocess.run([
        aicup_bin,
        '--batch-mode',
        '--config', config_path,
        '--save-results', result_path,
        '--save-replay', replay_path,
        '--player-names', *[str(v) for v in player_names],
    ])


def read_json(path):
    with open(path) as stream:
        return json.load(stream)


def write_json(data, path):
    with open(path, 'w') as stream:
        json.dump(data, stream, indent=4)


def generate_config(config_template, opponent_type, game_type, player_port, other_port, swap):
    player = make_player(player_port)
    opponent = generate_opponent(opponent_type=opponent_type, port=other_port)
    level, properties = generate_game(game_type, template=config_template['options_preset']['Custom']['properties'])
    return {
        'seed': None,
        'options_preset': {
            'Custom': {
                'level': level,
                'properties': properties,
            }
        },
        'players': [opponent, player] if swap else [player, opponent]
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


def generate_opponent(opponent_type, port):
    if opponent_type == 'Tcp':
        return make_player(port)
    if opponent_type == 'Quickstart':
        return 'Quickstart'
    if opponent_type == 'Empty':
        return {'Empty': None}
    raise RuntimeError('Invalid opponent_type: %s' % opponent_type)


def generate_game(game_type, template):
    if game_type == '2x1':
        return 'Simple', make_properties(team_size=1, template=template)
    if game_type == '2x2':
        return 'Simple', make_properties(team_size=2, template=template)
    if game_type == '2x2+':
        return 'Complex', make_properties(team_size=2, template=template)
    raise RuntimeError('Invalid game_type: %s' % game_type)


def make_properties(team_size, template):
    data = copy.deepcopy(template)
    data['team_size'] = team_size
    return data


if __name__ == "__main__":
    main()
