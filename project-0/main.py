total = 0
target_perfect_squares_count = 266000
for i in range(target_perfect_squares_count):
    square = (i + 1) ** 2
    if square % 2 == 1:
        total += (i + 1) ** 2

print(f"The sum of the odd squares of the first {target_perfect_squares_count} perfect squares is {total}")
