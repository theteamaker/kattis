import sys

hive_maps = []

for i in sys.stdin:
    fruit = [x.strip("\n") for x in i.split(" ")]

    hive_map = []
    hive_map.append(float(fruit[0]))

    for x in range(int(fruit[1])):
        hive_map.append([float(x) for x in sys.stdin.readline().strip("\n").split(" ")])
    
    if len(hive_map) > 1:
        hive_maps.append(hive_map)

# get ready for the yandere dev if/else blocks

def sweet_and_sour(hive_map):
    fighting_distance = hive_map[0]

    points = hive_map.copy()
    points.remove(points[0])

    hives = {} #coords, status: 'sweet' or 'sour'
    
    for i1 in range(len(points)):
        try:
            hives[i1]
        except:
            hives[i1] = "sweet"

        point_index = i1

        x = points[i1][0]
        y = points[i1][1]
        
        for i2 in points:
            comparison_point_index = points.index(i2)

            if comparison_point_index == point_index:
                continue

            comparison_x = i2[0]
            comparison_y = i2[1]

            if (comparison_x, comparison_y) == (x, y): # if it's the same hive, no need to make a comparison
                continue
            
            abs_x = abs(x - comparison_x)
            abs_y = abs(y - comparison_y)
            abs_c = (abs_x**2 + abs_y**2)**0.5

            if abs_c < fighting_distance:
                hives[point_index] = "sour"
                hives[comparison_point_index] = "sour"
                continue
    
    sweet_sum = 0
    sour_sum = 0

    for k, v in hives.items():
        if v == "sour":
            sour_sum += 1
        else:
            sweet_sum += 1
    
    print(f"{sour_sum} sour, {sweet_sum} sweet")

# update: actually, the conditional statements don't look as bad as i thought they would

for i in hive_maps:
    sweet_and_sour(i)