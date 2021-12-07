import numpy as np
from io import StringIO

with open('input-day-5.txt') as file:
    content = file.read()
    content_csv = content.replace(' -> ', ',')
    lines: np.ndarray = np.genfromtxt(StringIO(content_csv), delimiter=',', dtype=int)

max_coord = int(lines.max()) + 1

floor_task_1 = np.zeros((max_coord, max_coord))
floor_task_2 = np.zeros((max_coord, max_coord))


def range_inclusive(a, b):
    return range(a, b + 1) if a < b else range(a, b - 1, -1)


for x1, y1, x2, y2 in lines:
    if x1 == x2:
        floor_task_1[x1, range_inclusive(y1, y2)] += 1
    elif y1 == y2:
        floor_task_1[range_inclusive(x1, x2), y1] += 1
    else:
        indices = [(x, y) for x, y in zip(
            range_inclusive(x1, x2),
            range_inclusive(y1, y2)
        )]

        floor_task_2[tuple(zip(*indices))] += 1

floor_task_2 += floor_task_1

print(np.count_nonzero(floor_task_1 >= 2))
print(np.count_nonzero(floor_task_2 >= 2))
