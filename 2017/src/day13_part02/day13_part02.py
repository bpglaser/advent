from itertools import count
from math import floor
from sys import argv


def main():
    scanners = load_input()

    for delay in count():
        collided = False
        for k, v in scanners.items():
            if position(delay + k, v) == 0:
                collided = True
                break
        if not collided:
            print('collisionless delay: {}'.format(delay))
            break


def position(t, n):
    m = t % ((n - 1) * 2)
    return 2 * (n - 1) - m if m > n - 1 else m 


def load_input():
    with open(argv[1]) as f:
        scanners = {}
        for line in f:
            params = [word for word in map(int, line.split(':'))]
            scanners[params[0]] = params[1]
        return scanners


if __name__ == '__main__':
    main()
