from day09_part02 import *


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

def test_insert():
    root = Node(0)
    root.insert_right(1)
    assert root.n == 0
    assert root.right.n == 1
    assert root.left.n == 1

    root.insert_right(2)
    assert root.n == 0
    assert root.right.n == 2
    assert root.right.left.n == 0
    assert root.right.right.n == 1
    assert root.right.right.left.n == 2
    assert root.right.right.right.n == 0

    root.right.right.left.insert_right(3)
    assert root.n == 0
    assert root.left.n == 1
    assert root.right.n == 2
    assert root.right.right.n == 3
