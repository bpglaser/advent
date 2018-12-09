from itertools import cycle
import re
from sys import argv


SPECIAL_MULTIPLE = 23
SPECIAL_OFFSET = 7


def solve(numplayers, lastmarblescore):
    scores = {p: 0 for p in range(1, numplayers + 1)}

    circle = [0]
    i = 0
    num = 1

    for player in cycle(scores):
        if num > lastmarblescore:
            break

        if num % SPECIAL_MULTIPLE == 0:
            scores[player] += num
            i = (i - 7) % len(circle)
            scores[player] += circle[i]
            del circle[i]
        else:
            i = ((i + 1) % len(circle)) + 1
            circle.insert(i, num)

        num += 1

    return max(scores.values())


def main():
    with open(argv[1]) as f:
        pattern = re.compile(
            r'(\d+) players; last marble is worth (\d+) points')
        match = pattern.match(f.readline())
    numplayers, lastmarblescore = map(int, match.groups())
    print(solve(numplayers, lastmarblescore))


if __name__ == '__main__':
    main()
