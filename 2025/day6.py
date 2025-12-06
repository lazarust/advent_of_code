# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///


def main() -> None:
    with open("./inputs/day6.txt") as f:
        # with open("./inputs/test_day6.txt") as f:
        raw_data = f.read().strip()

    rows = raw_data.splitlines()
    problems = []
    ops = rows[-1]
    for row in rows[:-1]:
        col = 0
        split_row = row.strip().split(" ")
        for num in split_row:
            if num != "":
                if col <= len(problems) - 1 and isinstance(problems[col], list):
                    problems[col].append(int(num))
                else:
                    problems.append([int(num)])

                col += 1

    op_locations = 0
    for op in ops.strip().split(" "):
        if op != "":
            problems[op_locations].append(op)
            op_locations += 1

    running_sum = 0
    for problem in problems:
        problem.reverse()
        op = problem[0]
        problem.pop(0)
        temp_answer = problem[0]
        problem.pop(0)
        for num in problem:
            if op == "*":
                temp_answer *= num
            else:
                temp_answer += num

        running_sum += temp_answer

    print(running_sum)


if __name__ == "__main__":
    main()
