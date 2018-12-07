from collections import defaultdict, deque
import string
from sys import argv


def main():
    points = load_input(argv[1])
    bounds = find_bounds(points)
    names = {p: c for p, c in zip(points, string.ascii_letters)}

    grids = {p: fill(p, bounds) for p in points}

    master_grid = {}
    for 


def fill(parent, bounds):
    minx, miny, maxx, maxy = bounds
    grid = {}
    for x in range(minx, maxx + 1):
        for y in range(miny, maxy + 1):
            grid[(x, y)] = manhattan_distance((x, y), parent)
    return grid


def manhattan_distance(a, b):
    x1, y1 = a
    x2, y2 = b
    return abs(x1 - x2) + abs(y1 - y1)


def print_grid(grid, bounds, names):
    minx, miny, maxx, maxy = bounds
    for y in range(miny, maxy):
        for x in range(minx, maxx):
            if (x, y) in names:
                print(names[(x, y)].upper(), end='')
                continue
            if (x, y) not in grid:
                print('_', end='')
            elif grid[(x, y)] is None:
                print('.', end='')
            else:
                print(names[grid[(x, y)]], end='')
        print()


def outofbounds(point, bounds):
    x, y = point
    minx, miny, maxx, maxy = bounds
    return x < minx or x > maxx or y < miny or y > maxy


def neighbors(point):
    x, y = point
    # up, down left right
    return (x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)


def find_bounds(points):
    minx, miny, maxx, maxy = points[0][0], points[0][0], points[0][1], points[0][1]

    for p in points[1:]:
        x, y = p
        minx = min(minx, x - 1)
        miny = min(miny, y - 1)
        maxx = max(maxx, x + 1)
        maxy = max(maxy, y + 1)

    return minx, miny, maxx, maxy


def load_input(path):
    with open(path) as f:
        points = []
        for line in f:
            point = tuple(map(int, line.split(', ')))
            points.append(point)
        return points


if __name__ == "__main__":
    main()
