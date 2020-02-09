#!/usr/bin/env python3

import sys
import numpy
import matplotlib.pyplot

values = numpy.array([float(v.strip()) for v in sys.stdin])

matplotlib.pyplot.hist(values)
matplotlib.pyplot.grid(True)
matplotlib.pyplot.show()
