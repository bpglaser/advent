import sys


class Agent:
    def __init__(self):
        self.facing = 0
        self.total = [0, 0]

    def turn(self, direction):
        if direction == 'R':
            self.facing += 1
            if self.facing > 3:
                self.facing = 0
        else:
            self.facing -= 1
            if self.facing < 0:
                self.facing = 3

    def move(self, count):
        try:
            self.total[self.facing] += count
        except IndexError:
            self.total[self.facing - 2]  -= count

    def distance(self):
        return sum([abs(n) for n in self.total])

def parse_command(s):
    s = s.strip()
    return (s[0], int(s[1:]))

def find_distance(commands):
    agent = Agent()
    for direction, n in commands:
        agent.turn(direction)
        agent.move(n)
    return agent.distance()

def main():
    commands = [parse_command(s) for s in sys.argv[1].split(',')]
    distance = find_distance(commands)
    print(distance)

if __name__ == '__main__':
    main()
