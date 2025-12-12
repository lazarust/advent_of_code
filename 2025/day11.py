# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

import sys


def main() -> None:
    use_test = "test" in sys.argv
    filename = "./inputs/test_day11.txt" if use_test else "./inputs/day11.txt"

    with open(filename) as f:
        raw_data = f.read().strip()

    rows = raw_data.splitlines()
    maps = {}
    for row in rows:
        split_row = row.split(":")
        maps[split_row[0]] = split_row[1].strip().split(" ")

    count = 0
    locations = maps["you"]
    for loc in locations:
        new_locs = maps[loc]
        if new_locs != ["out"]:
            locations.extend(new_locs)
        else:
            count += 1

    print(count)


if __name__ == "__main__":
    main()
