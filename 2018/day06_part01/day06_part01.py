from collections import defaultdict, deque
from sys import argv


def main():
    points = load_input(argv[1])
    bounds = find_bounds(points)
    names = {p: c for p, c in zip(points, 'abcdefghijklmnopqrstuvwxyz')}

    counts = {p: 0 for p in points}
    grid = {}
    queue = deque()

    for parent in points:
        # ensure that no two given points are in the same spot
        assert parent not in grid

        counts[parent] += 1
        grid[parent] = (parent, 0)
        for neighbor in neighbors(parent):
            queue.append((parent, neighbor))

    while len(queue) > 0:
        parent, point = queue.popleft()

        print('=================')
        print_grid(grid, bounds, names)
        print(names[parent], point)
        print(outofbounds(point, bounds))

        # import pdb
        # pdb.set_trace()

        if outofbounds(point, bounds):
            continue

        if point in grid:
            if grid[point] is None:
                continue
            otherparent, dist = grid[point]
            if otherparent != parent and :
                continue
            elif 
            else:
                counts[grid[point]] -= 1
                grid[point] = None
                continue

        counts[parent] += 1
        grid[point] = parent

        for neighbor in neighbors(point):
            queue.append((parent, neighbor))

    for p, count in counts.items():
        print(names[p], count)


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
