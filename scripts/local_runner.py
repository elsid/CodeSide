#!/usr/bin/env python3

import click
import copy
import datetime
import helpers
import json
import os
import os.path
import subprocess
import sys
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
@click.option('--player_port', default=31010, type=int)
@click.option('--opponent_port', default=32010, type=int)
@click.option('--max_runs', default=2**64 - 1, type=int)
@click.option('--prefix', default='default')
@click.option('--bin_path', required=True, type=str)
@click.option('--config_template_path', required=True, type=str)
@click.option('--output_path', default=os.path.join(os.getcwd(), 'results'), type=str)
@click.option('--verbose', is_flag=True)
def main(**kwargs):
    run(**kwargs)


def run(opponent_type, game_type, player_port, opponent_port, max_runs, prefix, bin_path,
        aicup_config_template_path, output_path, verbose, should_stop=None):
    config_template = helpers.read_json(aicup_config_template_path)
    session = '%s.%s.%s.%s' % (prefix, game_type, player_port, datetime.datetime.now().strftime('%Y-%m-%d_%H-%M-%S'))
    session_path = os.path.join(output_path, opponent_type, game_type, session)
    os.makedirs(session_path, exist_ok=False)
    for number in range(max_runs):
        if should_stop is not None and should_stop.is_set():
            break
        game = '%s.%s' % (number, int(time.time() * 1e6))
        game_path = os.path.join(session_path, game)
        process = run_game(
            opponent_type=opponent_type,
            game_type=game_type,
            player_port=player_port,
            opponent_port=opponent_port,
            swap=number % 2 == 1,
            bin_path=bin_path,
            config_template=config_template,
            verbose=verbose,
            output_path=game_path,
        )
        try:
            process.wait(timeout=120)
        except subprocess.TimeoutExpired:
            traceback.print_exc()


def run_game(opponent_type, game_type, player_port, opponent_port, swap, bin_path, config_template, verbose,
             output_path, seed=None):
    os.makedirs(output_path, exist_ok=False)
    config_path = os.path.join(output_path, 'config.json')
    result_path = os.path.join(output_path, 'result.json')
    config = generate_config(
        config_template=config_template,
        opponent_type=opponent_type,
        game_type=game_type,
        player_port=player_port,
        opponent_port=opponent_port,
        swap=swap,
        seed=seed,
    )
    helpers.write_json(data=config, path=config_path)
    player_names = [opponent_port, player_port] if swap else [player_port, opponent_port]
    args=[
        bin_path,
        '--batch-mode',
        '--config', config_path,
        '--save-results', result_path,
        '--player-names', *[str(v) for v in player_names],
    ]
    if verbose:
        print('run', *args)
    return subprocess.Popen(
        args=args,
        stdout=None if verbose else subprocess.DEVNULL,
        stderr=None if verbose else subprocess.DEVNULL,
    )


def generate_config(config_template, opponent_type, game_type, player_port, opponent_port, swap, seed):
    player = make_player(player_port)
    opponent = generate_opponent(opponent_type=opponent_type, port=opponent_port)
    level, properties = generate_game(game_type, template=config_template['options_preset']['Custom']['properties'])
    return {
        'seed': seed,
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
