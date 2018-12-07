from collections import defaultdict
import re
from sys import argv
import pdb


NUM_HELPERS = 4
TIME_OFFSET = 60


def main():
    graph = load_graph(argv[1])

    noincoming = set()
    for node in graph:
        if not any(node in children for children in graph.values()):
            noincoming.add(node)

    helpers = {}
    ans = []
    i = 0
    while len(noincoming) > 0:
        helped = []
        for node in helpers:
            helpers[node] -= 1
            if helpers[node] == 0:
                ans.append(node)
                noincoming.remove(node)
                del graph[node]
                helped.append(node)

        for node in helped:
            del helpers[node]

        for node in graph:
            if not any(node in children for children in graph.values()):
                noincoming.add(node)

        ready = [node for node in noincoming]
        ready.sort()

        for node in ready:
            if node not in helpers and len(helpers) < NUM_HELPERS:
                helpers[node] = duration(node)

        names = list(helpers.keys())
        name0 = names[0] if len(names) > 0 else '.'
        name1 = names[1] if len(names) > 1 else '.'
        done = ''.join(ans)
        print('{}\t{}\t{}\t{}'.format(i, name0, name1, done))
        
        if len(noincoming) > 0:
            i += 1

    print(''.join(ans))
    print(i)


def duration(c):
    return TIME_OFFSET + (ord(c) - ord('A') + 1)


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
