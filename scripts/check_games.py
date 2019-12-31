#!/usr/bin/env python3

import json
import numpy
import os.path
import statistics
import sys
import operator
import functools

from collections import defaultdict, Counter


def main():
    paths = sorted(sys.argv[1:], key=get_time)
    games = list(collect_data(paths))
    check_games(games)


def check_games(games):
    for game in games:
        check_game(game)


def check_game(game):
    seed = game['seed']
    results = game['results']
    if sum(1 for v in results.values() if v['score'] == 0) == len(results):
        print('zero_draw', seed)
    scores = frozenset(v['score'] for v in results.values())
    min_score = min(scores)
    if len(scores) != 1 and 'Tcp_310' in results and results['Tcp_310']['score'] == min_score:
        print('loss', seed, results['Tcp_310']['position'])
    for k, v in results.items():
        if v['crashed']:
            print('crashed', k, seed)


def get_time(path):
    return int(os.path.basename(path).split('.')[-1])


def collect_data(paths):
    for path in paths:
        config_path = os.path.join(path, 'config.json')
        if not os.path.exists(config_path):
            continue
        config_content = read_file(config_path)
        if not config_content:
            continue
        players = parse_config(config_content)
        result_path = os.path.join(path, 'result.json')
        if not os.path.exists(result_path):
            continue
        result_content = read_file(result_path)
        if not result_content:
            continue
        yield parse_result(result_content, players)


def read_file(path):
    with open(path) as f:
        return f.read()


def parse_config(content):
    data = json.loads(content)
    return {get_player_name(v): n for n, v in enumerate(data['players'])}


def get_player_name(data):
    if isinstance(data, str):
        return data
    if isinstance(data, dict):
        if 'Empty' in data:
            return 'Empty'
        if 'Tcp' in data:
            return 'Tcp_%s' % str(data['Tcp']['port'])[:3]
    raise RuntimeError('Invalid player data: %s' % data)


def parse_result(content, players):
    data = json.loads(content)
    results = {name: get_record(data, index) for name, index in players.items()}
    return dict(results=results, seed=data['seed'])


def get_record(data, index):
    return dict(
        crashed=data['players'][index]['crashed'],
        score=data['results'][index],
        position=index,
    )


if __name__ == '__main__':
    main()
