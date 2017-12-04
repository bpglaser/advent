from sys import argv


def are_anagrams(words):
    anagram = set(words[0])
    for word in words[1:]:
        if set(word) != anagram:
            return False
    return True


def main():
    path = argv[1]
    with open(path) as f:
        count = 0

        for line in f:
            words = [word for word in line.strip().split()]
            valid = True

            for n, word in enumerate(words):
                for other in words[n + 1:]:
                    if are_anagrams([word, other]):
                        valid = False

            if valid:
                count += 1

        print('valid passphrases: {}'.format(count))


if __name__ == '__main__':
    main()
