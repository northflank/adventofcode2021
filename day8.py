import itertools

all_segments = "abcdefg"
digit_segments = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"]


def get_digit(segments):
    try:
        return digit_segments.index("".join(sorted(segments)))
    except ValueError:
        return None


def could_be_digit(segments):
    return get_digit(segments) is not None


def permute(segments, permutation_dict):
    return "".join([permutation_dict[s] for s in segments])


with open('input-day-8.txt') as file:
    inputs = file.readlines()
    inputs = [i.split('|') for i in inputs]
    inputs = [(a.strip().split(' '), b.strip().split(' ')) for a, b in inputs]

permutations = itertools.permutations(all_segments)
permutations = [{a: b for a, b in zip(all_segments, p)} for p in permutations]

unscrambled_digits = []

for observations, digits in inputs:
    for permutation in permutations:
        permuted = [permute(o, permutation) for o in observations]

        if all(could_be_digit(p) for p in permuted):
            digits = [permute(n, permutation) for n in digits]
            digits = [get_digit(n) for n in digits]

            unscrambled_digits.append(digits)
            break


digit_counts = dict()
for digits in unscrambled_digits:
    for digit in digits:
        digit_counts[digit] = digit_counts.get(digit, 0) + 1

print("Task 1:", sum(digit_counts[i] for i in [1, 4, 7, 8]))

unscrambled_numbers = [int("".join(map(str, digits))) for digits in unscrambled_digits]

print("Task 2:", sum(unscrambled_numbers))
