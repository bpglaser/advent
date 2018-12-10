from collections import namedtuple
from itertools import count
from PIL import Image
import re
from sys import argv


Point = namedtuple('Point', ['x', 'y', 'vx', 'vy'])


def main():
    points = loadpoints(argv[1])

    minsize = size(points)
    mint = 0

    for t in count(start=1):
        state = [simulate(p, t) for p in points]
        n = size(state)
        if n < minsize:
            minsize = n
            mint = t
        elif n > minsize:
            break
    print(mint)
    render([simulate(p, mint) for p in points])


def render(points):
    minx, maxx, miny, maxy = bounds(points)
    img = Image.new('1', (maxx - minx + 1, maxy - miny + 1))
    for p in points:
        img.putpixel((p.x - minx, p.y - miny), 1)
    img.save('out.ppm')


def simulate(point, time):
    return Point(point.x + point.vx * time, point.y + point.vy * time, point.vx, point.vy)


def size(points):
    minx, maxx, miny, maxy = bounds(points)
    return (maxx - minx) * (maxy - miny)


def bounds(points):
    minx = min(map(lambda p: p.x, points))
    maxx = max(map(lambda p: p.x, points))
    miny = min(map(lambda p: p.y, points))
    maxy = max(map(lambda p: p.y, points))
    return minx, maxx, miny, maxy


def loadpoints(path):
    pattern = re.compile(r'position=<([ -]?\d+), ([ -]?\d+)> velocity=<([ -]?\d+), ([ -]?\d+)>')
    with open(path) as f:
        points = []
        for line in f:
            match = pattern.match(line)
            x, y, vx, vy = map(int, match.groups())
            points.append(Point(x, y, vx, vy))
        return points


if __name__ == '__main__':
    main()
