import numpy as np


def parse_dot(line: str):
    if "," in line:
        x, y = line.strip().split(",")
        return int(y), int(x)  # reverse coords to be row-major


def parse_fold(line: str):
    if "=" in line:
        a, n = line.replace("fold along", "").strip().split("=")
        return a, int(n)


with open('input-day-13.txt') as file:
    lines = file.readlines()
    dots = [d for line in lines if (d := parse_dot(line)) is not None]
    folds = [f for line in lines if (f := parse_fold(line)) is not None]

shape = np.array(dots).max(axis=0) + 1
data = np.zeros(shape)

data[tuple(zip(*dots))] = 1

for i, (axis, row) in enumerate(folds):
    if axis == "x":
        data = data[:, 0:row] + data[:, :row:-1]

    if axis == "y":
        data = data[0:row, :] + data[:row:-1, :]

    if i == 0:
        print("Task 1:")
        print(np.count_nonzero(data))
        print()

print("Task 2:")
for row in data:
    print("".join("." if i == 0 else "#" for i in row))
