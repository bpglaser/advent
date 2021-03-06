from collections import deque
from sys import argv


NUM_GENERATIONS = 50000000000


def main():
    state, rules = load_input(argv[1])
    result, ans = solve(state, rules)
    print(''.join(map(lambda pair: pair[1], result)))
    print(ans)


def solve(state, rules):
    lastvalue = None
    seen = set()
    seen.add(stringify(state))

    for i in range(NUM_GENERATIONS):
        pad(state)
        state = step(state, rules)
        unpad(state)

        s = stringify(state)
        if s in seen:
            if lastvalue is not None:
                curr = count_ans(state)
                return state, (NUM_GENERATIONS - i - 1) * (curr - lastvalue) + curr
            else:
                lastvalue = count_ans(state)
        else:
            seen.add(s)

    return state, count_ans(state)


def stringify(state):
    return ''.join(map(lambda t: t[1], state))


def pad(state):
    i, _ = state[0]
    for j in range(1, 6):
        state.appendleft((i - j, '.'))
    i, _ = state[-1]
    for j in range(1, 6):
        state.append((i + j, '.'))


def unpad(state):
    while True:
        i, c = state.pop()
        if c == '#':
            state.append((i, c))
            break

    while True:
        i, c = state.popleft()
        if c == '#':
            state.appendleft((i, c))
            break


def count_ans(state):
    return sum(i for i, c in state if c == '#')


def step(state, rules):
    buf = deque()
    bufnums = deque()

    result = deque()
    for i, c in state:
        buf.append(c)
        bufnums.append(i)
        if len(buf) < 5:
            continue
        if len(buf) == 6:
            buf.popleft()
            bufnums.popleft()
        result.append((bufnums[2], rules[''.join(buf)]))
    return result


def load_input(path):
    with open(path) as f:
        lines = [line.strip() for line in f]
        given_state = lines[0].split(': ')[1]

        state = deque()
        zero = given_state.index('#')
        for i, c in enumerate(given_state):
            state.append((i - zero, c))

        rules = {line.split(' => ')[0]: line.split(' => ')[1]
                 for line in lines[2:]}

        return state, rules


if __name__ == '__main__':
    main()
