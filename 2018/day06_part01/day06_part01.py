from sys import argv


def main():
    points = load_input(argv[1])
    bounds = find_bounds(points)
    minx, miny, maxx, maxy = bounds

    grid = {}
    for y in range(miny, maxy + 1):
        for x in range(minx, maxx + 1):
            closest_point, closest_point_dist = None, None
            for point in points:
                dist = manhattan_distance((x, y), point)
                if closest_point_dist is None or dist < closest_point_dist:
                    closest_point = point
                    closest_point_dist = dist
                elif dist == closest_point_dist:
                    closest_point = None
            grid[(x, y)] = closest_point

    counts = {}
    inf = set()
    for point, parent in grid.items():
        x, y = point
        if parent is None or parent in inf:
            continue
        if x == minx or x == maxx or y == miny or y == maxy:
            inf.add(parent)
            if parent in counts:
                del counts[parent]
            continue
        if parent not in counts:
            counts[parent] = 0
        counts[parent] += 1
    print(max(counts.values()))


def manhattan_distance(a, b):
    x1, y1 = a
    x2, y2 = b
    return abs(x1 - x2) + abs(y1 - y2)


def find_bounds(points):
    safety = 100
    minx, miny, maxx, maxy = points[0][0], points[0][0], points[0][1], points[0][1]

    for p in points[1:]:
        x, y = p
        minx = min(minx, x - safety)
        miny = min(miny, y - safety)
        maxx = max(maxx, x + safety)
        maxy = max(maxy, y + safety)

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
