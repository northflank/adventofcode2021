from collections import Counter

with open("input-day-14.txt") as f:
    initial, _, *insertion_rules = [line.strip() for line in f.readlines()]
    insertion_rules = dict(rule.split(" -> ") for rule in insertion_rules)


######################################
# Task 1 Solution
######################################
def pairs(s: str):
    yield from (s[i] + s[i + 1] for i in range(len(s) - 1))


def grow_step(string: str, rules: dict):
    result = string[0]

    for pair in pairs(string):
        result += rules[pair] + pair[1]

    return result


def grow(current: str, rules: dict, iterations: int):
    for _ in range(iterations):
        current = grow_step(current, rules)

    return current


counts_task_1 = Counter(grow(initial, insertion_rules, 10)).most_common()
print("Task 2", counts_task_1[0][1] - counts_task_1[-1][1])


######################################
# Task 2 Solution
######################################
def simulate_growth(s: str, rules: dict, iterations: int):
    counts = Counter(pairs(s))

    for _ in range(iterations):
        counts = (Counter({s[0] + rules[s]: c, rules[s] + s[1]: c}) for s, c in counts.items())
        counts = sum(counts, Counter())

    counts = (*(Counter({s[1]: c}) for s, c in counts.items()), Counter(s[0]))
    counts = sum(counts, Counter())

    return counts


counts_task_2 = simulate_growth(initial, insertion_rules, 40).most_common()
print("Task 2", counts_task_2[0][1] - counts_task_2[-1][1])
