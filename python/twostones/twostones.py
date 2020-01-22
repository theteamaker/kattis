import sys

for i in sys.stdin:
  fruit = int(i) / 2
  if fruit.is_integer():
    print("Bob")
  else:
    print("Alice")