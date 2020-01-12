#!/usr/bin/env python3

import sys
import json

from collections import Counter


def main():
    merged = Counter()
    for path in sys.argv[1:]:
        with open(path) as stream:
            for item in json.load(stream):
                merged[tuple(item['neighborhood'])] += item['score']
    json.dump([dict(neighborhood=v[0], score=v[1]) for v in list(sorted(merged.items(), key=lambda v: v[1]))], sys.stdout)


if __name__ == "__main__":
    main()
