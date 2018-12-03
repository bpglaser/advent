from collections import namedtuple
import re


PATH = 'input.txt'
Rectangle = namedtuple('Rectangle', ['id', 'x', 'y', 'w', 'h'])
REGEX = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')
SIZE = 1000


def parse(line):
    match = REGEX.match(line)
    return Rectangle(*map(int, match.groups()))


def rectangles():
    with open(PATH) as f:
        for line in f:
            yield parse(line)


board = [[0 for _ in range(SIZE)] for _ in range(SIZE)]
for rect in rectangles():
    for x in range(rect.x, rect.x + rect.w):
        for y in range(rect.y, rect.y + rect.h):
            board[x][y] += 1

for rect in rectangles():
    if all(board[x][y] == 1 for x in range(rect.x, rect.x + rect.w) for y in range(rect.y, rect.y + rect.h)):
        print(rect)
