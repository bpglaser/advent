import copy
import sys

class Display:
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.pixels = []
        for _ in range(self.height):
            buf = []
            for _ in range(self.width):
                buf.append(False)
            self.pixels.append(buf)

    def handle_line(self, line):
        words = line.split()
        if words[0] == "rect":
            x, y = words[1].split('x')
            x, y = int(x), int(y)
            self.rect(x, y)
        else:
            a = int(words[2].rsplit('=')[-1])
            b = int(words[4])
            if words[1] == 'column':
                self.column(a, b)
            else:
                self.row(a, b)

    def rect(self, x, y):
        for rowindex in range(y):
            for columnindex in range(x):
                self.pixels[rowindex][columnindex] = True

    def column(self, columnindex, count):
        for _ in range(count):
            temp = copy.deepcopy(self.pixels)
            for rowindex in range(self.height - 1):
                self.pixels[rowindex + 1][columnindex] = temp[rowindex][columnindex]
            self.pixels[0][columnindex] = temp[-1][columnindex]

    def row(self, rowindex, count):
        for _ in range(count):
            temp = copy.deepcopy(self.pixels[rowindex])
            for columnindex in range(self.width - 1):
                self.pixels[rowindex][columnindex + 1] = temp[columnindex]
            self.pixels[rowindex][0] = temp[-1]

    def total(self):
        n = 0
        for row in self.pixels:
            for pixel in row:
                if pixel:
                    n += 1
        return n

    def __str__(self):
        s = ''
        for row in self.pixels:
            s = s + ''.join(map(lambda b: 'X' if b else ' ', row))
            s = s + '\n'
        return s

def main():
    display = Display(width=50, height=6)
    filename = sys.argv[1]
    lines = [line.strip() for line in open(filename)]
    for line in lines:
        display.handle_line(line)
    print(display)
    print('total illuminated: {}'.format(display.total()))

if __name__ == '__main__':
    main()
