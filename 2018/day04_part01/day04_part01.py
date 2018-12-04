from enum import Enum
from sys import argv


class EntryType(Enum):
    BEGIN = 1
    SLEEP = 2
    WAKE = 3


class Parser:
    def __init__(self, s):
        self.i = 0
        self.s = s

    def accept(self, tag):
        if self.s.startswith(tag, self.i):
            self.i += len(tag)
            return True
        return False

    def expect(self, tag):
        if not self.accept(tag):
            raise RuntimeError('Error parsing at position {}. Expected: "{}" Got: "{}"'.format(
                self.i, tag, self.s[self.i:self.i+len(tag)]))

    def parse(self):
        self.parse_timestamp()
        self.expect(' ')
        self.parse_action()
        if not self.i == len(self.s):
            raise RuntimeError(
                'Unparsed values after {} in {}'.format(self.i, self.s))
        return self.entry

    def parse_timestamp(self):
        self.expect('[')
        year = self.parse_int()
        self.expect('-')
        month = self.parse_int()
        self.expect('-')
        day = self.parse_int()
        self.expect(' ')
        hour = self.parse_int()
        self.expect(':')
        minute = self.parse_int()
        self.expect(']')
        self.date = (year, month, day, hour, minute)

    def parse_int(self):
        buf = []
        while self.s[self.i] in '0123456789':
            buf.append(self.s[self.i])
            self.i += 1
        return int(''.join(buf))

    def parse_action(self):
        if self.accept('Guard'):
            self.parse_begins_shift()
        elif self.accept('falls'):
            self.parse_falls_asleep()
        elif self.accept('wakes'):
            self.parse_wakes_up()
        else:
            raise RuntimeError(
                'Unknown action at {}. [{}]'.format(self.i, self.s[self.i:]))

    def parse_begins_shift(self):
        self.expect(' #')
        num = self.parse_int()
        self.expect(' begins shift')
        self.entry = (EntryType.BEGIN, self.date, num)

    def parse_falls_asleep(self):
        self.expect(' asleep')
        self.entry = (EntryType.SLEEP, self.date)

    def parse_wakes_up(self):
        self.expect(' up')
        self.entry = (EntryType.WAKE, self.date)


def main():
    path = argv[1]
    entries = [Parser(line).parse()
               for line in map(str.strip, open(path))]
    entries.sort(key=lambda e: e[1])

    freqs = {}

    i = 0
    while i < len(entries):
        if entries[i][0] != EntryType.BEGIN:
            raise RuntimeError('Expected a BEGIN')

        num = entries[i][2]
        if num not in freqs:
            freqs[num] = [0 for _ in range(60)]
        freq = freqs[num]
        i += 1

        while i < len(entries) and entries[i][0] == EntryType.SLEEP:
            start = entries[i][1][4]
            i += 1

            if entries[i][0] != EntryType.WAKE:
                raise RuntimeError(
                    'Expected to wake {}'.format(str(entries[i])))
            end = entries[i][1][4]
            i += 1

            for hour in range(start, end):
                freq[hour] += 1

    biggestsleeper = (None, 0)
    for num, freq in freqs.items():
        sleepinghours = 0
        for cnt in freq:
            sleepinghours += cnt
        if sleepinghours >= biggestsleeper[1]:
            biggestsleeper = (num, sleepinghours)
    hour = max(enumerate(freqs[biggestsleeper[0]]),
               key=lambda pair: pair[1])[0]

    print(biggestsleeper[0], hour)
    print(biggestsleeper[0] * hour)


if __name__ == "__main__":
    main()
