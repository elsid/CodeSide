#!/usr/bin/env python3

import json
import numpy
import os.path
import statistics
import sys
import operator
import functools
import numbers

from collections import defaultdict, Counter


def main():
    paths = sorted(sys.argv[1:], key=get_time)
    games = list(collect_data(paths))
    stats = get_stats(games)
    print_stats(stats)
    show_stats(stats)


def print_stats(stats):
    row('metric', 'value', '%')
    for metric in ('games', 'draws', 'zero_draws', 'unique_seeds'):
        row(metric, stats[metric], stats[metric] / stats['games'] * 100)
    print()
    players = stats['players']
    row('metric', *players, *['%s %%' % v for v in players], '%s / %s' % (players[0], players[1]), '%s / %s' % (players[1], players[0]), 'total')
    for metric, values in stats.items():
        print_metric(metric, values, players)


def print_metric(metric, values, players):
    if isinstance(values, dict):
        if values:
            if isinstance(tuple(values.values())[0], dict):
                for submetric, subvalues in values.items():
                    print_metric('%s %s' % (metric, submetric), subvalues, players)
            elif isinstance(tuple(values.values())[0], numbers.Number):
                print_counter(metric, values, players)


def print_counter(metric, values, players):
    row_values = [values[v] for v in players]
    total = sum(row_values)
    fractions = [safe_div(v, total) * 100 for v in row_values]
    ratios = [safe_div(values[players[0]], values[players[1]]), safe_div(values[players[1]], values[players[0]])]
    row(metric, *row_values, *fractions, *ratios, total)


def safe_div(a, b):
    return a / b if b else float('inf')


def show_stats(stats):
    import matplotlib.pyplot as pyplot

    show_metric_plot(pyplot, stats, 'scores_dynamic')
    show_metric_plot(pyplot, stats, 'scores_dynamic_cumsum')
    show_metric_plot(pyplot, stats, 'places_dynamic_cumsum')
    show_metric_plot(pyplot, stats, 'wins_dynamic_cumsum')
    show_metric_plot(pyplot, stats, 'losses_dynamic_cumsum')

    show_ratio_plots(pyplot, stats, 'scores_dynamic_cumsum')
    show_ratio_plots(pyplot, stats, 'places_dynamic_cumsum')
    show_ratio_plots(pyplot, stats, 'wins_dynamic_cumsum')
    show_ratio_plots(pyplot, stats, 'losses_dynamic_cumsum')

    show_scores_distribution_plot(pyplot, stats)
    show_positions_distribution_plot(pyplot, stats)
    show_seeds_distribution_plot(pyplot, stats)

    pyplot.show()


def show_ratio_plots(pyplot, stats, metric):
    players = stats['players']
    show_plot(
        pyplot,
        name='%s %s / %s' % (metric, players[0], players[1]),
        values=stats[metric][players[0]] / stats[metric][players[1]],
    )
    show_plot(
        pyplot,
        name='%s %s / %s' % (metric, players[1], players[0]),
        values=stats[metric][players[1]] / stats[metric][players[0]],
    )


def show_metric_plot(pyplot, stats, metric):
    fig, ax = pyplot.subplots()
    fig.canvas.set_window_title(metric)
    ax.set_title(metric)
    for player, values in stats[metric].items():
        ax.plot(numpy.arange(0, len(values), 1), values, label=player)
    total = functools.reduce(operator.add, stats[metric].values())
    ax.plot(numpy.arange(0, len(total), 1), total, label='total')
    ax.grid(True)
    ax.legend()


def show_plot(pyplot, name, values):
    fig, ax = pyplot.subplots()
    fig.canvas.set_window_title(name)
    ax.set_title(name)
    ax.plot(numpy.arange(0, len(values), 1), values, label=name)
    ax.grid(True)
    ax.legend()


def show_scores_distribution_plot(pyplot, stats):
    players = stats['players']
    fig, ax = pyplot.subplots()
    fig.canvas.set_window_title('scores_dynamic')
    ax.set_title('scores_dynamic')
    bins = numpy.linspace(0, max(stats['max_score'][v] for v in players) + 1, 50)
    for player, values in stats['scores_dynamic'].items():
        ax.hist(values, bins=bins, label=player, alpha=0.5)
        ax.set_xticks(bins)
        ax.grid(True)
        ax.legend()


def show_positions_distribution_plot(pyplot, stats):
    players = stats['players']
    fig, ax = pyplot.subplots()
    fig.canvas.set_window_title('positions_dynamic')
    ax.set_title('positions_dynamic')
    bins = [0, 1, 2]
    for player, values in stats['positions_dynamic'].items():
        ax.hist(values, bins=bins, label=player, alpha=0.5)
        ax.set_xticks(bins)
        ax.grid(True)
        ax.legend()


def show_seeds_distribution_plot(pyplot, stats):
    fig, ax = pyplot.subplots()
    fig.canvas.set_window_title('seeds')
    ax.set_title('seeds')
    bins = numpy.linspace(0, 2**64, 32)
    ax.hist(stats['seeds'], bins=32)
    ax.set_xticks(bins)
    ax.grid(True)


def get_stats(games):
    draws = 0
    zero_draws = 0
    players = set()
    wins = Counter()
    losses = Counter()
    places = defaultdict(Counter)
    crashes = Counter()
    positions = defaultdict(Counter)
    places_positions = defaultdict(lambda: defaultdict(Counter))
    seeds = set()
    scores = defaultdict(list)
    places_dynamic = defaultdict(list)
    positions_dynamic = defaultdict(list)
    wins_dynamic = defaultdict(list)
    losses_dynamic = defaultdict(list)
    for number, game in enumerate(games):
        game_scores = numpy.array(sorted(frozenset(v['score'] for v in game['results'].values()), reverse=True))
        if len(game_scores) == 1:
            draws += 1
            if game_scores[0] == 0:
                zero_draws += 1
        max_score = max(game_scores)
        min_score = min(game_scores)
        if 1 == sum(1 for v in game_scores if v == max_score):
            winner = next(k for k, v in game['results'].items() if v['score'] == max_score)
            wins[winner] += 1
            wins_dynamic[winner].append(1)
        if 1 == sum(1 for v in game_scores if v == min_score):
            loser = next(k for k, v in game['results'].items() if v['score'] == min_score)
            losses[loser] += 1
            losses_dynamic[loser].append(1)
        for place, score in enumerate(game_scores):
            for k, v in game['results'].items():
                if v['score'] == score:
                    places[place + 1][k] += 1
                    places_dynamic[k].append(place + 1)
                    places_positions[place + 1][v['position']][k] += 1
        for k, v in game['results'].items():
            players.add(k)
            if v['crashed']:
                crashes[k] += 1
            scores[k].append(v['score'])
            positions[v['position']][k] += 1
            positions_dynamic[k].append(v['position'])
            if len(wins_dynamic[k]) < number + 1:
                wins_dynamic[k].append(0)
            if len(losses_dynamic[k]) < number + 1:
                losses_dynamic[k].append(0)
        seeds.add(game['seed'])
    for k in scores.keys():
        scores[k] = numpy.array(scores[k])
        places_dynamic[k] = numpy.array(places_dynamic[k])
    return dict(
        games=len(games),
        draws=draws,
        zero_draws=zero_draws,
        unique_seeds=len(seeds),
        players=sorted(players),
        wins=wins,
        losses=losses,
        places=places,
        crashes=crashes,
        positions=positions,
        places_positions=places_positions,
        total_score={k: sum(v) for k, v in scores.items()},
        median_score={k: statistics.median(v) for k, v in scores.items()},
        mean_score={k: statistics.mean(v) for k, v in scores.items()},
        stdev_score={k: statistics.stdev(v) for k, v in scores.items()},
        min_score={k: min(v) for k, v in scores.items()},
        max_score={k: max(v) for k, v in scores.items()},
        q95_score={k: numpy.quantile(v, 0.95) for k, v in scores.items()},
        scores_dynamic=scores,
        scores_dynamic_cumsum=cumsums(scores),
        places_dynamic=places_dynamic,
        places_dynamic_cumsum=cumsums(places_dynamic),
        wins_dynamic=wins_dynamic,
        wins_dynamic_cumsum=cumsums(wins_dynamic),
        losses_dynamic=losses_dynamic,
        losses_dynamic_cumsum=cumsums(losses_dynamic),
        positions_dynamic=positions_dynamic,
        seeds=numpy.array(sorted(seeds)),
    )


def cumsums(values):
    return {k: numpy.cumsum(v) for k, v in values.items()}


def get_time(path):
    return int(os.path.basename(path).split('.')[-1])


def row(*args):
    print(('{:>25}' * len(args)).format(*args))


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
