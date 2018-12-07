from collections import defaultdict
import re
from sys import argv
import pdb


def main():
    graph = load_graph(argv[1])

    noincoming = set()
    for node in graph:
        if not any(node in children for children in graph.values()):
            noincoming.add(node)

    ans = []
    while len(noincoming) > 0:
        ready = [node for node in noincoming]
        ready.sort()
        ans.append(ready[0])
        noincoming.remove(ready[0])
        del graph[ready[0]]

        for node in graph:
            if not any(node in children for children in graph.values()):
                noincoming.add(node)

    print(''.join(ans))


def load_graph(path):
    pattern = re.compile(
        r'Step (\w+) must be finished before step (\w+) can begin.')
    with open(path) as f:
        graph = defaultdict(set)
        for line in f:
            parent, child = pattern.match(line).groups()
            graph[parent].add(child)
            if child not in graph:
                graph[child] = set()
    return graph


if __name__ == '__main__':
    main()
