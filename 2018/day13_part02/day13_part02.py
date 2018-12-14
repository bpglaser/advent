from sys import argv


def main():
    grid = load_input(argv[1])

    carts = find_carts(grid)
    while True:
        newstate = {}

        todo = list(carts.keys())
        todo.sort(key=lambda pair: (pair[1], pair[0]))
        unprocessed = set(todo)

        for x, y in todo:
            if (x, y) not in unprocessed:
                continue
            unprocessed.remove((x, y))

            facing, turn = carts[(x, y)]
            nx, ny = next_pos(x, y, facing)

            if (nx, ny) in newstate:
                # print(f'{nx},{ny}')
                del newstate[(nx, ny)]
                continue
            if (nx, ny) in unprocessed:
                # print(f'{nx},{ny}')
                unprocessed.remove((nx, ny))
                continue

            c = grid[ny][nx]
            newstate[(nx, ny)] = reflect(facing, turn, c)

        if len(newstate) == 1:
            for x, y in newstate:
                print(f'{x},{y}')
                return

        carts = newstate


def reflect(facing, turn, c):
    if c == '/':
        if facing == '^':
            return '>', turn
        elif facing == 'v':
            return '<', turn
        elif facing == '<':
            return 'v', turn
        elif facing == '>':
            return '^', turn
        else:
            raise RuntimeError
    elif c == '\\':
        if facing == '^':
            return '<', turn
        elif facing == 'v':
            return '>', turn
        elif facing == '<':
            return '^', turn
        elif facing == '>':
            return 'v', turn
        else:
            raise RuntimeError
    elif c == '+':
        if turn == '<':
            if facing == '^':
                return '<', '^'
            elif facing == 'v':
                return '>', '^'
            elif facing == '<':
                return 'v', '^'
            elif facing == '>':
                return '^', '^'
            else:
                raise RuntimeError
        elif turn == '^':
            if facing == '^':
                return '^', '>'
            elif facing == 'v':
                return 'v', '>'
            elif facing == '<':
                return '<', '>'
            elif facing == '>':
                return '>', '>'
            else:
                raise RuntimeError
        elif turn == '>':
            if facing == '^':
                return '>', '<'
            elif facing == 'v':
                return '<', '<'
            elif facing == '<':
                return '^', '<'
            elif facing == '>':
                return 'v', '<'
            else:
                raise RuntimeError
        else:
            raise RuntimeError
    return facing, turn


def next_pos(x, y, facing):
    if facing == '^':
        return x, y - 1
    elif facing == 'v':
        return x, y + 1
    elif facing == '<':
        return x - 1, y
    else:
        return x + 1, y


def replace_carts(grid):
    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c == '^':
                grid[y][x] = '|'
            elif c == 'v':
                grid[y][x] = '|'
            elif c == '<':
                grid[y][x] = '-'
            elif c == '>':
                grid[y][x] = '-'
            else:
                continue


def find_carts(grid):
    carts = {}
    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c in '^v<>':
                carts[(x, y)] = (c, '<')
    return carts


def load_input(path):
    with open(path) as f:
        return [list(line.strip('\r\n')) for line in f]


if __name__ == '__main__':
    main()
