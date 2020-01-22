import sys

contestantScores = []

totals = []

for i in sys.stdin:
    toAppend = i.strip('\n')
    contestantScores.append(toAppend)

#print(contestantScores)

for i in contestantScores:
    numList = i.split(" ")
    listOfIntegers = []
    total = 0

    for i in numList:
        listOfIntegers.append(int(i))

    for i in listOfIntegers:
        total += i
    
    totals.append(total)

winningIndex = int(totals.index(max(totals))) + 1

print(winningIndex, max(totals))