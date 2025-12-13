# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

import sys


def main() -> None:
    use_test = "test" in sys.argv
    filename = "./inputs/test_day12.txt" if use_test else "./inputs/day12.txt"

    with open(filename) as f:
        raw_data = f.read()

    groups = raw_data.split("\n\n")

    presents = {}
    for i, group in enumerate(groups[:-1]):
        lines = group.strip().split("\n")
        shape = lines[1:]
        presents[i] = len(shape) * len(shape[0])

    regions_raw = groups[-1].strip().split("\n")
    regions = []
    for region in regions_raw:
        parts = region.split(": ", 1)
        dimensions = tuple(int(x) for x in parts[0].split("x"))
        counts = tuple(int(x) for x in parts[1].split(" "))
        regions.append((dimensions, counts))

    result = 0
    for (x, y), piece_counts in regions:
        total_area = sum(
            count * presents[shape] for shape, count in enumerate(piece_counts)
        )
        if total_area <= x * y:
            result += 1

    print(result)


if __name__ == "__main__":
    main()
