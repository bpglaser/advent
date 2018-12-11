from sys import argv


GRID_SIZE = 300


def main():
    x, y, power = solve(int(argv[1]))
    print(f'{x},{y} => {power}')


def solve(serial_number):
    # build the grid
    grid = []
    for y in range(1, GRID_SIZE + 1):
        row = []
        for x in range(1, GRID_SIZE + 1):
            row.append(power_level(x, y, serial_number))
        grid.append(row)

    # find the max kernel
    maxx, maxy, maxp = None, None, None
    for y in range(1, GRID_SIZE - 1):
        for x in range(1, GRID_SIZE - 1):
            p = total_power(grid, x, y)
            if maxp is None or p > maxp:
                maxx, maxy, maxp = x, y, p
    
    return maxx, maxy, maxp


def power_level(x, y, serial_number):
    rack_id = x + 10
    power_level = rack_id * y
    power_level += serial_number
    power_level *= rack_id
    if power_level < 100:
        power_level = 0
    else:
        power_level = (power_level // 100) % 10
    power_level -= 5
    return power_level


def total_power(grid, x, y):
    total = 0
    for dy in range(3):
        for dx in range(3):
            total += grid[y + dy - 1][x + dx - 1]
    return total


if __name__ == '__main__':
    main()
