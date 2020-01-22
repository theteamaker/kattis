import sys

computer_input = []

for i in sys.stdin:
  computer_input.append(i)
  computer_input = [i.replace('\n', '') for i in computer_input]

x_values = []
y_values = []

missing_value = []

def xDivider(inp):
  listToPull = inp.split(" ")
  x_values.append(int(listToPull[0]))
  y_values.append(int(listToPull[1]))

for i in range(len(computer_input)):
  xDivider(computer_input[i])

for i in range(len(x_values)):
  tester = x_values.count(x_values[i])
  if tester == 1:
    missing_value.append(x_values[i])

for i in range(len(y_values)):
  tester = y_values.count(y_values[i])
  if tester == 1:
    missing_value.append(y_values[i])

print(int(missing_value[0]), end = ' ')
print(int(missing_value[1]))