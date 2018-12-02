from collections import Counter


with open('input.txt') as f:
    twos = set()
    threes = set()

    for line in f:
        for k, v in Counter(line).items():
            if v == 2:
                twos.add(line)
            elif v == 3:
                threes.add(line)

    print(len(twos) * len(threes))