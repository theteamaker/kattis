import sys
for i in sys.stdin:

    out = i[0]

    for x in range(len(i)):
        if i[x] == '-':
            out += i[x + 1]

    print(out)