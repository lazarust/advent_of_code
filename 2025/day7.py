# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///


def main() -> None:
    # with open("./inputs/test_day7.txt") as f:
    with open("./inputs/day7.txt") as f:
        raw_data = f.read().strip()

    rows = raw_data.splitlines()
    first_row = rows[0]

    first_row.split()
    column_with_lazer = set([first_row.find("S")])
    split_count = 0
    for row in rows[1:]:
        for lazer_loc in set(column_with_lazer):
            if row[lazer_loc] == "^":
                column_with_lazer.add(lazer_loc - 1)
                column_with_lazer.add(lazer_loc + 1)
                column_with_lazer.remove(lazer_loc)
                split_count += 1

    print(split_count)


if __name__ == "__main__":
    main()
