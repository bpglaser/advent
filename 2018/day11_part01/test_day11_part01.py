from day11_part01 import *


def test_power_level():
    assert power_level(3, 5, 8) == 4
    assert power_level(122, 79, 57) == -5
    assert power_level(217, 196, 39) == 0
    assert power_level(101, 153, 71) == 4


def test_solve():
    assert solve(18) == (33, 45, 29)
    assert solve(42) == (21, 61, 30)


def test_total_power():
    grid = [
        [-2, -4, 4, 4, 4],
        [-4, 4, 4, 4, -5],
        [4, 3, 3, 4, -4],
        [1, 1, 2, 4, -3],
        [-1, 0, 2, -5, -2],
    ]
    assert total_power(grid, 2, 2) == 29

    grid = [
        [-3, 4, 2, 2, 2],
        [-4, 4, 3, 3, 4],
        [-5, 3, 3, 4, -4],
        [4, 3, 3, 4, -3],
        [3, 3, 3, -5, -1],
    ]
    assert total_power(grid, 2, 2) == 30
