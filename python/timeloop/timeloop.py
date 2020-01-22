import sys

for i in sys.stdin:
  for f in range(1, int(i) + 1):
    whatTheWizardSays = 'Abracadabra'
    print(str(f) + ' ' + whatTheWizardSays)