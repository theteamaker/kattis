import sys

inp = []

stripped_line = []
while len(stripped_line) < 2:
    for i in sys.stdin:
        inp.append(str(i))
        stripped_line = [s.rstrip() for s in inp]
        break

if len(stripped_line) == 2:
    newestList = list(map(int, stripped_line[0].split()))
    inpList = stripped_line[1]

    organizer = {
        'C' : {
        },
        'A' : {
        },
        'B' : {
        }
    }
    organizer['A']['name'] = min(newestList)
    organizer['C']['name'] = max(newestList)
    newestList.remove(min(newestList))
    newestList.remove(max(newestList))
    organizer['B']['name'] = newestList[0]
    print(organizer[inpList[0]]['name'], end = " ")
    print(organizer[inpList[1]]['name'], end = " ")
    print(organizer[inpList[2]]['name'], end = " ")