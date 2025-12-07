# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

import sys


def main() -> None:
    use_test = "test" in sys.argv
    filename = "./inputs/test_day7.txt" if use_test else "./inputs/day7.txt"

    with open(filename) as f:
        raw_data = f.read().strip()

    rows = raw_data.splitlines()
    first_row = rows[0]
    start_col = first_row.find("S")
    path_counts = {start_col: 1}
    for row in rows[1:]:
        if "^" not in row:
            continue

        new_counts = {}

        for col, count in path_counts.items():
            if row[col] == "^":
                new_counts[col - 1] = new_counts.get(col - 1, 0) + count
                new_counts[col + 1] = new_counts.get(col + 1, 0) + count
            else:
                new_counts[col] = new_counts.get(col, 0) + count

        path_counts = new_counts

    print(sum(path_counts.values()))


if __name__ == "__main__":
    main()
