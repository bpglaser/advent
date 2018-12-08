from sys import argv


class Node:
    def __init__(self, children, meta):
        self.children = children
        self.meta = meta

    def value(self):
        if len(self.children) == 0:
            return sum(self.meta)
        else:
            total = 0
            for m in self.meta:
                try:
                    total += self.children[m - 1].value()
                except IndexError:
                    pass
            return total


class Parser:
    def __init__(self, s):
        self.s = s
        self.i = 0

    def parse_node(self):
        numchildren, nummeta = self.parse_header()
        children = [self.parse_node() for _ in range(numchildren)]
        meta = [self.parse_meta() for _ in range(nummeta)]
        return Node(children, meta)

    def parse_header(self):
        numchildren = self.parse_int()
        self.expect(' ')
        nummeta = self.parse_int()
        self.accept(' ')
        return numchildren, nummeta

    def parse_int(self):
        buf = []
        while self.i < len(self.s) and self.s[self.i] in '0123456789':
            buf.append(self.s[self.i])
            self.i += 1
        assert len(buf) > 0
        return int(''.join(buf))

    def parse_meta(self):
        meta = self.parse_int()
        self.accept(' ')
        return meta

    def accept(self, tag):
        if self.s.startswith(tag, self.i):
            self.i += len(tag)
            return True
        return False

    def expect(self, tag):
        assert self.accept(tag)


def main():
    with open(argv[1]) as f:
        line = f.readline()
    root = Parser(line).parse_node()
    print(root.value())


if __name__ == '__main__':
    main()
