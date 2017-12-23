b = 106700
c = 123700
h = 0

for i in range(b, c + 1, 17):
    for j in range(2, i):
        if i % j == 0:
            h += 1
            break;

print(h)