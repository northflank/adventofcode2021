import numpy as np
from io import StringIO


with open('input-day-4.txt') as file:
    content = file.read()

numbers, *boards = content.split("\n\n")
numbers = [float(n) for n in numbers.split(",")]

boards = [np.genfromtxt(StringIO(b)) for b in boards]
boards_marked = [np.zeros_like(b) for b in boards]

first_winning_score = None
last_winning_score = None

for n in numbers:
    winning_boards = []

    for i, (board, marks) in enumerate(zip(boards, boards_marked)):
        marks[np.equal(n, board)] = 1

        won = any([
            *(marks.sum(0) == board.shape[0]),
            *(marks.sum(1) == board.shape[1]),
        ])

        if won:
            board[np.equal(1, marks)] = 0
            score = board.sum() * n

            last_winning_score = score
            if not first_winning_score:
                first_winning_score = score

            winning_boards.append(i)

    # Filter winning boards for next step
    boards = [b for i, b in enumerate(boards) if i not in winning_boards]
    boards_marked = [b for i, b in enumerate(boards_marked) if i not in winning_boards]

print(first_winning_score)
print(last_winning_score)
