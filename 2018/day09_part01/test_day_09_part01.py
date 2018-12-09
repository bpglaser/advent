from day09_part01 import solve


def test_given1():
    assert solve(10, 1618) == 8317


def test_given2():
    assert solve(13, 7999) == 146373


def test_given3():
    assert solve(17, 1104) == 2764


def test_given4():
    assert solve(21, 6111) == 54718


def test_given5():
    assert solve(30, 5807) == 37305
