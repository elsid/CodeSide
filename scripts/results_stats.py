#!/usr/bin/env python3

import json
import numpy
import os.path
import statistics
import sys


def main():
    import matplotlib.pyplot

    paths = sorted(sys.argv[1:], key=get_time)
    games = list(collect_data(paths))
    players = dict(
        first=dict(places=[], scores=[], score_diffs=[]),
        second=dict(places=[], scores=[], score_diffs=[]),
    )
    for r in games:
        for k, p in players.items():
            p['places'].append(r[k]['place'])
            p['scores'].append(r[k]['score'])
        players['first']['score_diffs'].append(r['first']['score'] - r['second']['score'])
        players['second']['score_diffs'].append(r['second']['score'] - r['first']['score'])
    _1 = [sum(w == 1 for w in v['places']) for v in players.values()]
    _2 = [sum(w == 2 for w in v['places']) for v in players.values()]
    for v in players.values():
        v['places'] = numpy.array(v['places'])
        v['scores'] = numpy.array(v['scores'])
    scores_partial_sums = [numpy.cumsum(v['scores']) for v in players.values()]
    place_1_partial_sums = [numpy.cumsum([w == 1 for w in v['places']]) for v in players.values()]
    stats = dict(
        _1=_1,
        _2=_2,
        wins=[
            sum(w[0] > w[1] for w in zip(*[v['scores'] for v in players.values()])),
            sum(w[1] > w[0] for w in zip(*[v['scores'] for v in players.values()]))
        ],
        total_score=[sum(v['scores']) for v in players.values()],
        min_score=[min(v['scores']) for v in players.values()],
        max_score=[max(v['scores']) for v in players.values()],
        mean_score=[statistics.mean(v['scores']) for v in players.values()],
        median_score=[statistics.median(v['scores']) for v in players.values()],
        stdev_score=[statistics.stdev(v['scores']) for v in players.values()],
        q95_score=[numpy.quantile(v['scores'], 0.95) for v in players.values()],
        min_score_diff=[min(v['score_diffs']) for v in players.values()],
        max_score_diff=[max(v['score_diffs']) for v in players.values()],
        mean_score_diff=[statistics.mean(v['score_diffs']) for v in players.values()],
        median_score_diff=[statistics.median(v['score_diffs']) for v in players.values()],
        stdev_score_diff=[statistics.stdev(v['score_diffs']) for v in players.values()],
    )
    draws = sum(w[0] == w[1] for w in zip(*[v['scores'] for v in players.values()]))
    row('games', len(games))
    row('second wins', stats['wins'][1], stats['wins'][1] / len(games))
    row('draws', draws, draws / len(games))
    row('first wins', stats['wins'][0], stats['wins'][0] / len(games))
    print()
    row('', *(list(players.keys()) + ['ratio (second/first)']))
    for k, v in stats.items():
        row(k, *(v + [ratio(v)]))
    print()
    fig, ax = matplotlib.pyplot.subplots()
    ax.set_title('scores distribution')
    bins = numpy.linspace(0, max(max(v['scores']) for v in players.values()) + 1)
    for k, v in players.items():
        ax.hist(v['scores'], bins=bins, label=k, alpha=0.5)
        ax.set_xticks(bins)
        ax.grid(True)
        ax.legend()
    _1 = {k: [0] for k in players.keys()}
    fig, ax = matplotlib.pyplot.subplots()
    ax.set_title('place 1 dynamic')
    for g in games:
        for k, v in _1.items():
            v.append(v[-1] + (g[k]['place'] == 1))
    for k, v in _1.items():
        ax.plot(numpy.arange(0, len(games) + 1, 1), v, label=k)
        ax.grid(True)
        ax.legend()
    scores = {k: [0] for k in players.keys()}
    fig, ax = matplotlib.pyplot.subplots()
    ax.set_title('scores dynamic')
    for g in games:
        for k, v in scores.items():
            v.append(v[-1] + g[k]['score'])
    for k, v in scores.items():
        ax.plot(numpy.arange(0, len(games) + 1, 1), v, label=k)
        ax.grid(True)
        ax.legend()
    fig, ax = matplotlib.pyplot.subplots()
    ax.set_title('scores diffs')
    bins = numpy.linspace(
        min(min(v['score_diffs']) for v in players.values()),
        max(max(v['score_diffs']) for v in players.values()) + 1
    )
    for k, v in players.items():
        ax.hist(v['score_diffs'], bins=50, label=k, alpha=0.5)
        ax.set_xticks(bins)
        ax.grid(True)
        ax.legend()
    fig, ax = matplotlib.pyplot.subplots()
    seeds = numpy.array([v['seed'] for v in games])
    ax.hist(seeds)
    fig, ax = matplotlib.pyplot.subplots()
    ax.set_title('scores ratio dynamic first / second')
    scores_partial_sums_ratio = numpy.array([v[0] / v[1] for v in zip(*scores_partial_sums)])
    ax.plot(numpy.arange(0, len(games), 1), scores_partial_sums_ratio)
    ax.grid(True)
    fig, ax = matplotlib.pyplot.subplots()
    ax.set_title('place 1 ratio dynamic second / first')
    place_1_partial_sums_ratio = numpy.array([v[1] / (v[0] or 1) for v in zip(*place_1_partial_sums)])
    ax.plot(numpy.arange(0, len(games), 1), place_1_partial_sums_ratio)
    ax.grid(True)
    matplotlib.pyplot.show()


def get_time(path):
    components = os.path.basename(path).split('.')
    if len(components) == 6:
        return int(components[-3]) * 1e6 + int(components[-2])
    else:
        return int(components[-2]) * 1e6


def ratio(values):
    if values[1] == values[0]:
        return 1
    elif values[0] == 0:
        return float('inf')
    else:
        return values[1] / values[0]


def row(*args):
    print(('{:>25}' * len(args)).format(*args))


def collect_data(paths):
    for path in paths:
        content = read_result(path)
        if content:
            yield parse_result(content)


def read_result(path):
    with open(path) as f:
        return f.read()


def parse_result(content):
    data = json.loads(content)
    return dict(first=get_record(data, 0), second=get_record(data, 1), seed=data['seed'])


def get_record(data, index):
    return dict(
        crashed=data['players'][index]['crashed'],
        score=data['results'][index],
        place=next(n for n, v in enumerate(sorted(data['results'], reverse=True)) if v == data['results'][index]) + 1,
    )


if __name__ == '__main__':
    main()
