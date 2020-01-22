import sys

total = 0
numberOfPeriods = []

while numberOfPeriods == []:
    for i in sys.stdin:
        numberOfPeriods.append(str(i))
        numberOfPeriods = [s.rstrip() for s in numberOfPeriods]
        numberOfPeriods = list(map(int, numberOfPeriods[0].split()))
        break

inpo = []
while len(inpo) < (numberOfPeriods[0]):
    for i in sys.stdin:
        inpo.append(str(i))
        inpo = [s.rstrip() for s in inpo]
        break

# inpo1 = ["1.0 12.0", "0.7 5.2", "0.9 10.7", "0.5 20.4", "0.2 30.0"]

def inpoStrangler (inp):
    objects = list(map(float, inp.split(" ")))
    return objects[0] * objects[1]

if numberOfPeriods[0] != 0 and len(inpo) == numberOfPeriods[0]:
    for i in range(numberOfPeriods[0]):
        total += inpoStrangler(inpo[i])

    print("%5.3f" % total)