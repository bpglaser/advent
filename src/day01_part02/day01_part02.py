import sys


class BunnyFoundError(Exception):
    pass


class Agent:
    def __init__(self):
        self.facing = 0
        self.x = 0
        self.y = 0
        self.history = set()
        self.history.add(self.position)

    @property
    def position(self):
        return (self.x, self.y)

    def turn(self, direction):
        if direction == 'R':
            self.facing += 1
            if self.facing > 3:
                self.facing = 0
        else:
            self.facing -= 1
            if self.facing < 0:
                self.facing = 3

    def move_distance(self, distance):
        for i in range(distance):
            self.forward()

    def forward(self):
        if self.facing == 0:
            self.y += 1
        elif self.facing == 1:
            self.x += 1
        elif self.facing == 2:
            self.y -= 1
        elif self.facing == 3:
            self.x -= 1
        else:
            raise RuntimeError
        if self.position in self.history:
            raise BunnyError()
        self.history.add(self.position)

    def distance(self):
        return sum([abs(n) for n in self.position])


def parse_command(s):
    s = s.strip()
    return (s[0], int(s[1:]))

def find_distance(commands):
    agent = Agent()
    try:
        for direction, n in commands:
            agent.turn(direction)
            agent.move_distance(n)
    except BunnyFoundError:
        return agent.distance()

def main():
    commands = [parse_command(s) for s in sys.argv[1].split(',')]
    distance = find_distance(commands)
    print(distance)

if __name__ == '__main__':
    main()
