import numpy as np
from scipy import stats

inputs = np.loadtxt('input.txt', str)
inputs = np.array(list(map(lambda i: list(map(int, list(i))), inputs)))

print(inputs)

# Task 1
gamma = stats.mode(inputs)[0][0]
epsilon = 1 - gamma


def to_int(a: np.ndarray):
    return a.dot(1 << np.arange(a.shape[-1] - 1, -1, -1))


print("Task 1 Solution:")
print(to_int(gamma) * to_int(epsilon))


# Task 2
def find_rating(data: np.ndarray, flip: bool):
    for bit in range(data.shape[1]):
        ones = np.count_nonzero(data[:, bit])
        zeros = data.shape[0] - ones

        if (ones >= zeros) != flip:
            data = data[data[:, bit] == 1]
        else:
            data = data[data[:, bit] == 0]

        if (len(data)) == 1:
            return data


oxygen_generator_rating = to_int(find_rating(inputs, False))
co2_scrubber_rating = to_int(find_rating(inputs, True))

print("Task 2 Solution:")
print(oxygen_generator_rating * co2_scrubber_rating)
