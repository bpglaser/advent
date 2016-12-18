from sys import argv

class State(object):
    def __init__(self, instructions):
        self.register = {'a': 0, 'b': 0, 'c': 1, 'd': 0}
        self.execution_index = 0
        self.instructions = instructions

    def __call__(self):
        while self.execution_index >= 0 and self.execution_index < len(self.instructions):
            instruction = self.instructions[self.execution_index]
            result = getattr(self, instruction[0])(*instruction[1:])
            print('[{}]\t{}\t\t=> {}'.format(self.execution_index, instruction, self.register))
            if result != None:
                self.execution_index = result
            else:
                self.execution_index += 1
        return self.register

    def cpy(self, x, y):
        try:
            self.register[y] = int(x)
        except ValueError:
            self.register[y] = self.register[x]

    def inc(self, x):
        self.register[x] += 1

    def dec(self, x):
        self.register[x] -= 1

    def jnz(self, x, y):
        try:
            x = int(x)
        except ValueError:
            x = self.register[x]
        if x != 0:
            return self.execution_index + int(y)
        else:
            return self.execution_index + 1

def main():
    instructions = [line.split() for line in open(argv[1])]
    state = State(instructions)
    result = state()
    print(result)

if __name__ == '__main__':
    main()
