import sys

sysInput = []
for i in sys.stdin:
  sysInput.append(i)
  sysInput = [i.strip("\n") for i in sysInput]

def funcChecker(inp):
  splitList = inp.split(" ")
  if int(splitList[0]) > (int(splitList[1]) - int(splitList[2])):
    print("do not advertise")
  elif int(splitList[0]) == (int(splitList[1]) - int(splitList[2])):
    print("does not matter")
  else:
    print("advertise")


for i in range(int(sysInput[0])):
  funcChecker(sysInput[i + 1])