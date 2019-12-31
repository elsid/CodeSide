#!/usr/bin/env python3

import matplotlib.pyplot
import numpy
import sys


def main():
    user_time = list()
    system_time = list()
    ticks = list()
    for path in sys.argv[1:]:
        with open(path) as stream:
            for line in stream:
                if 'result' in line:
                    ticks.append(int(line.strip().split(' ')[-1]))
                elif 'User time (seconds)' in line and len(ticks) > len(user_time):
                    user_time.append(float(line.strip().split(' ')[-1]))
                elif 'System time (seconds)' in line and len(ticks) > len(system_time):
                    system_time.append(float(line.strip().split(' ')[-1]))
    user_time = numpy.array(user_time) * 1000
    system_time = numpy.array(system_time) * 1000
    ticks = numpy.array(ticks)
    print(len(ticks), len(user_time), len(system_time))
    matplotlib.pyplot.hist(user_time / ticks, label='User time per tick (ms)')
    matplotlib.pyplot.hist(system_time / ticks, label='System time per tick (ms)')
    matplotlib.pyplot.grid(True)
    matplotlib.pyplot.legend()
    matplotlib.pyplot.show()


if __name__ == "__main__":
    main()
