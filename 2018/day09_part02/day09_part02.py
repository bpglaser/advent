from itertools import cycle
import re
from sys import argv


SPECIAL_MULTIPLE = 23
SPECIAL_OFFSET = 7


class Node:
    def __init__(self, n):
        self.n = n
        self.left = self
        self.right = self
    
    def insert_right(self, n):
        node = Node(n)
        node.left = self
        node.right = self.right
        self.right.left = node
        self.right = node

    def delete_left(self):
        self.left.left.right = self
        self.left = self.left.left


def solve(numplayers, lastmarblescore):
    scores = {p: 0 for p in range(1, numplayers + 1)}

    circle = Node(0)
    num = 1

    for player in cycle(scores):
        if num > lastmarblescore:
            break

        if num % SPECIAL_MULTIPLE == 0:
            scores[player] += num
            for _ in range(7):
                circle = circle.left
            scores[player] += circle.n
            circle = circle.right
            circle.delete_left()
        else:
            circle = circle.right
            circle.insert_right(num)
            circle = circle.right

        num += 1

    return max(scores.values())


def main():
    with open(argv[1]) as f:
        pattern = re.compile(
            r'(\d+) players; last marble is worth (\d+) points')
        match = pattern.match(f.readline())
    numplayers, lastmarblescore = map(int, match.groups())
    lastmarblescore *= 100
    print(solve(numplayers, lastmarblescore))


if __name__ == '__main__':
    main()
