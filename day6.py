import numpy as np
from io import StringIO

with open('input-day-6.txt') as file:
    fish_list = [int(i) for i in file.read().split(',')]

fish_by_timer = [0] * 9

for f in fish_list:
    fish_by_timer[f] += 1

for day in range(256):
    if day == 80:
        print("Task 1", sum(fish_by_timer))

    num_spawning_fish, *fish_by_timer = fish_by_timer
    fish_by_timer[6] += num_spawning_fish
    fish_by_timer.append(num_spawning_fish)

print("Task 2", sum(fish_by_timer))
