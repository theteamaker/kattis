import sys

class Point:
    def __init__(self, point, sweet=True):
        self.point = point
        self.sweet = sweet
    
    def sourify(self):
        self.sweet = False

for line in sys.stdin:
    dangerous_distance, cases = [x.strip("\n") for x in line.split(" ")]
    points = []

    for i in range(int(cases)):
        point = [float(x) for x in sys.stdin.readline().strip("\n").split(" ")]
        points.append(Point(point))
    
    for i1 in range(len(points)):
        point_index = i1
        x = points[i1].point[0]
        y = points[i1].point[1]

        for i2 in points:
            if points.index(i2) == point_index:
                continue        

            comparison_x = i2.point[0]
            comparison_y = i2.point[1]

            abs_x = abs(x - comparison_x)
            abs_y = abs(y - comparison_y)
            distance = (abs_x**2 + abs_y**2)**0.5

            if distance <= float(dangerous_distance):
                i2.sourify()
                points[i1].sourify()
                continue
    
    total_sweet, total_sour = 0, 0

    for i in points:
        if i.sweet is True:
            total_sweet += 1
        else:
            total_sour += 1
    
    if len(points) > 0:
        print(f"{total_sour} sour, {total_sweet} sweet")