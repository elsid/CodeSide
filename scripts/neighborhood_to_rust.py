#!/usr/bin/env python3

import sys
import json


TEMPLATE = '''use crate::my_strategy::NeighborhoodScore;

pub const NEIGHBORHOOD_SCORE: &[NeighborhoodScore] = &[
{0}
];'''

VALUE_TEMPLATE = '''    NeighborhoodScore {{
        neighborhood: {neighborhood},
        score: {score},
    }},'''

def main():
    values = json.load(sys.stdin)
    print(TEMPLATE.format('\n'.join(VALUE_TEMPLATE.format(**v) for v in values)))


if __name__ == "__main__":
    main()
