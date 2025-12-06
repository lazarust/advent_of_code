# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///


def main() -> None:
    with open("./inputs/day6.txt") as f:
        # with open("./inputs/test_day6.txt") as f:
        raw_data = f.read().rstrip("\n")

    rows = raw_data.splitlines()
    problems = []
    ops = rows[-1]

    op_locations = []
    for i, op in enumerate(ops):
        if op != " ":
            op_locations.append(i)

    for i, location in enumerate(op_locations):
        problem = []
        for row in rows[:-1]:
            for j in range(
                location,
                op_locations[i + 1] - 1 if i < len(op_locations) - 1 else len(row),
            ):
                number = row[j]
                if j - location < len(problem):
                    problem[j - location] += number
                else:
                    problem.append(number)

        problems.append(problem)

    running_sum = 0
    for i, op_loc in enumerate(op_locations):
        op = ops[op_loc]
        temp_sum = int(problems[i][0])
        for num in problems[i][1:]:
            if op == "*":
                temp_sum *= int(num)
            else:
                temp_sum += int(num)

        running_sum += temp_sum

    print(running_sum)


if __name__ == "__main__":
    main()
