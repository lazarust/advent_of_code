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
    paths_to_explore = [(loc, ["svr", loc]) for loc in maps["svr"]]

    for current_loc, path in paths_to_explore:
        next_locs = maps[current_loc]
        if next_locs == ["out"]:
            if "dac" in path and "fft" in path:
                count += 1
        else:
            for next_loc in next_locs:
                paths_to_explore.append((next_loc, path + [next_loc]))

    print(count)


if __name__ == "__main__":
    main()
