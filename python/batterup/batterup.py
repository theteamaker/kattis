import sys

inp = []

for i in sys.stdin:
    inp.append(str(i))

indexToDivideBy = int(inp[0])

nums = inp[1].split(" ")
realNums = []
for i in nums:
    numWOBadStuff = i.strip("\n")
    realNums.append(numWOBadStuff)

total = 0

for i in realNums:
    if int(i) >= 0:
        total += int(i)
    elif int(i) < 0:
        indexToDivideBy += -1

totalToOutput = total / indexToDivideBy

print(totalToOutput)