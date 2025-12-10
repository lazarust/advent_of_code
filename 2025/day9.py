# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

import sys


def main() -> None:
    use_test = "test" in sys.argv
    filename = "./inputs/test_day9.txt" if use_test else "./inputs/day9.txt"

    with open(filename) as f:
        raw_data = f.read().strip()

    rows = raw_data.splitlines()

    cords = []

    for row in rows:
        split_row = row.split(",")
        cords.append((int(split_row[0]), int(split_row[1])))

    max_area = 0
    for i, cord in enumerate(cords):
        for cord2 in cords[i:]:
            y = abs(cord[0] - cord2[0] + 1)
            x = abs(cord[1] - cord2[1] + 1)

            if x * y > max_area:
                max_area = x * y

    print(max_area)


if __name__ == "__main__":
    main()
