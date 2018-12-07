from sys import argv


def main():
    points = load_input(argv[1])
    bounds = find_bounds(points)
    minx, miny, maxx, maxy = bounds

    grid = {}
    compound_region = set()
    for y in range(miny, maxy + 1):
        for x in range(minx, maxx + 1):
            total_dist = 0
            closest_point, closest_point_dist = None, None
            for point in points:
                dist = manhattan_distance((x, y), point)
                total_dist += dist
                if closest_point_dist is None or dist < closest_point_dist:
                    closest_point = point
                    closest_point_dist = dist
                elif dist == closest_point_dist:
                    closest_point = None
            grid[(x, y)] = closest_point
            if total_dist < 10000:
                compound_region.add((x, y))

    print(len(compound_region))


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
