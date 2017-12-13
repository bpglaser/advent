from math import floor
from sys import argv


def main():
    scanners = load_input()
    severity = 0
    for k, v in scanners.items():
        g = make_sequence_generator(v)
        if g(k) == 0:
            severity += k * v
            print('collision: ({})'.format(k))
    print('total severity: {}'.format(severity))


def make_sequence_generator(n):
    def generator(m):
        return sum([(-1) ** floor((i - 1) / (n - 1)) for i in range(1, m + 1)])
    return generator


def load_input():
    with open(argv[1]) as f:
        scanners = {}
        for line in f:
            params = [word for word in map(int, line.split(':'))]
            scanners[params[0]] = params[1]
        return scanners


if __name__ == '__main__':
    main()
