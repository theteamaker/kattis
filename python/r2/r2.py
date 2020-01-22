import sys

inputList = []

for i in sys.stdin:
    inputList.append(i)
    inputList = inputList[0].split(' ')

inputList[:] = [line.rstrip('\n') for line in inputList]

R2 = (2 * int(inputList[1])) - int(inputList[0])

print(int(R2))