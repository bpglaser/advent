from itertools import cycle

with open('input.txt') as f:
    seen = {0}
    n = 0
    for i in cycle(map(int, f)):
        n += i
        if n in seen:
            print(n)
            break
        seen.add(n)
