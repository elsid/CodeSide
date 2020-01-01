#!/usr/bin/env python3

import sys

from collections import defaultdict


def main():
    text = sys.stdin.buffer.read().decode('utf-8')
    buffer = defaultdict(lambda: '.')
    pos_x = 2
    pos_y = 2
    for x in range(40):
        buffer[(x, 0)] = '#'
        buffer[(x, 29)] = '#'
    for y in range(30):
        buffer[(0, y)] = '#'
        buffer[(39, y)] = '#'
    max_y = 1
    for letter in text:
        if letter == '\n':
            pos_y += max_y
            pos_x = 2
            max_y = 1
        elif letter == ' ':
            pos_x += 1
        else:
            dx, dy = render_letter(letter, buffer, pos_x, pos_y)
            pos_x += dx + 1
            max_y = max(max_y, dy)
    print_buffer(buffer)


def render_letter(letter, buffer, pos_x, pos_y):
    image = LETTERS[letter]
    y = 0
    max_x = 0
    lines = image.split('\n')
    for line in lines:
        x = 0
        for pixel in line.rstrip():
            if pixel != ' ':
                buffer[(pos_x + x, pos_y + y)] = pixel
            x += 1
        max_x = max(max_x, x)
        y += 1
    return max_x, y


def print_buffer(buffer):
    for y in range(30):
        for x in range(40):
            print(buffer[(x, y)], end='')
        print()


LETTERS = {
    'С':
'''
 ####
#
#
#
 ####
''',
    'Н':
'''
#   #
#   #
#####
#   #
#   #
''',
    'О':
'''
 ###
#   #
#   #
#   #
 ###
''',
    'В':
'''
####
#   #
####
#   #
####
''',
    'Ы':
'''
#   #
#   #
##  #
# # #
##  #
''',
    'М':
'''
#   #
## ##
# # #
#   #
#   #
''',
    'Г':
'''
#####
#
#
#
#
''',
    'Д':
'''
  #
 # #
 # #
#####
#   #
''',
    '!':
'''
#
#
#

#
''',
    '❄':
'''
    H
 H  H  H
   HHH
HHHH HHHH
   HHH
 H  H  H
    H
''',
}


if __name__ == "__main__":
    main()
