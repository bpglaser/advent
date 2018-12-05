from sys import argv


def main():
    path = argv[1]
    with open(path) as f:
        line = f.readline()
        letters = [c for c in line]

    removed = 1
    while removed > 0:
        removed = 0
        clone = []

        i = 0
        while i < len(letters):
            if i == len(letters) - 1:
                clone.append(letters[i])
                break

            a, b = letters[i], letters[i + 1]

            if a.lower() == b.lower() and (bool(a.isupper()) != bool(b.isupper())):
                removed += 2
                i += 2
            else:
                clone.append(a)
                i += 1

        letters = clone

    print(''.join(letters))
    print(len(letters))


if __name__ == "__main__":
    main()
