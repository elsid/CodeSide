#!/usr/bin/env python3

import sys
import numpy
import matplotlib.pyplot

values = [[float(w) for w in v.split()] for v in sys.stdin]
x = numpy.array([v[0] for v in values])
y = numpy.array([v[1] for v in values])

matplotlib.pyplot.plot(x, y)
matplotlib.pyplot.grid(True)
matplotlib.pyplot.show()
