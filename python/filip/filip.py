import sys

inputList = []
compareList = []

while len(inputList) < 1:
    for i in sys.stdin:
        inputList = i.split(" ")
        inputList = [s.rstrip() for s in inputList]
        break

def numReverser(ogNumber):
    listOfNumbers = []
    for i in range(len(ogNumber)):
        listOfNumbers.append(ogNumber[i])
    return (listOfNumbers[2] + listOfNumbers[1] + listOfNumbers[0])

for i in range(len(inputList)):
    compareList.append(int(numReverser(inputList[i])))

print(max(compareList))