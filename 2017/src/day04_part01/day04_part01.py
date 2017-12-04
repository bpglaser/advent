from sys import argv

def main():
    path = argv[1]
    with open(path) as f:
        count = 0
        for line in f:
            words = [word for word in line.strip().split()]
            if len(words) == len(set(words)):
                count += 1
        print('valid passphrases: {}'.format(count))


if __name__ == '__main__':
    main()
