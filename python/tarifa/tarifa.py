import sys

inp = []
for i in sys.stdin:
    inp.append(int(i))

megabyteLimit = int(inp[0])
monthsValue = int(inp[1])
totalLeft = 0

for i in range(monthsValue):
    addVal = megabyteLimit - int(inp[2 + i])
    totalLeft += addVal

totalLeft += inp[0]
print(totalLeft)