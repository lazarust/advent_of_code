# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///


def remove_rolls(rows):
    to_remove = []
    for i, row in enumerate(rows):
        for j, pos in enumerate(row):
            if pos == "@":
                prev_row = ""
                next_row = ""
                prev_pos = ""
                next_pos = ""
                if i != 0:
                    prev_row = rows[i - 1][
                        min(j, abs(j - 1)) : j + 2 if j + 1 <= len(row) else j
                    ]
                if i < len(rows) - 1:
                    next_row = rows[i + 1][
                        min(j, abs(j - 1)) : j + 2 if j + 1 <= len(row) else j
                    ]

                if j != 0:
                    prev_pos = row[j - 1]
                if j < len(row) - 1:
                    next_pos = row[j + 1]
                sum = (
                    prev_row.count("@")
                    + next_row.count("@")
                    + prev_pos.count("@")
                    + next_pos.count("@")
                )

                if sum < 4:
                    to_remove.append((i, j))

    return to_remove


def main() -> None:
    # with open("./inputs/test_day4.txt") as f:
    with open("./inputs/day4.txt") as f:
        raw_data = f.read().strip()

    rows = raw_data.split("\n")

    count_removal = 0
    to_remove = remove_rolls(rows)
    count_removal += len(to_remove)

    while len(to_remove) > 0:
        for i, j in to_remove:
            temp_row = rows[i]
            rows[i] = temp_row[:j] + "." + temp_row[j + 1 :]
        to_remove = remove_rolls(rows)
        count_removal += len(to_remove)

    print(count_removal)


if __name__ == "__main__":
    main()
