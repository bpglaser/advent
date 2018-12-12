from collections import defaultdict
from day12_part01 import *


given_rules = defaultdict(lambda: '.')
given_rules.update({
    '...##': '#',
    '..#..': '#',
    '.#...': '#',
    '.#.#.': '#',
    '.#.##': '#',
    '.##..': '#',
    '.####': '#',
    '#.#.#': '#',
    '#.###': '#',
    '##.#.': '#',
    '##.##': '#',
    '###..': '#',
    '###.#': '#',
    '####.': '#',
})


def test_given():
    state = deque((i - 3, c)
                  for i, c in enumerate('...#..#.#..##......###...###...........'))
    result, ans = solve(state, given_rules)
    assert ans == 325
