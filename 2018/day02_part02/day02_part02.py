def get_masks(s):
    masks = []

    for i in range(len(s)):
        masks.append('{}.{}'.format(s[:i], s[i+1:]))

    return masks

with open('input.txt') as f:
    parents = set()

    for line in f:
        for mask in get_masks(line):
            if mask in parents:
                print(mask.replace('.', ''))
                break

            parents.add(mask)