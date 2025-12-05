# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///


def main() -> None:
    with open("./inputs/test_day5.txt") as f:
        # with open("./inputs/day5.txt") as f:
        raw_data = f.read().strip()

    split_data = raw_data.split("\n\n")
    ranges = split_data[0].splitlines()
    ingredients = split_data[1].splitlines()

    list_range = []
    for range_ids in ranges:
        split_range = range_ids.split("-")
        start = int(split_range[0])
        end = int(split_range[1])

        list_range.append((start, end))

    list_range.sort()
    overlaps = []
    for start, end in list_range:
        if len(overlaps) == 0:
            overlaps.append((start, end))
        else:
            if start <= overlaps[-1][1] + 1:
                overlaps[-1] = (overlaps[-1][0], max(overlaps[-1][1], end))
            else:
                overlaps.append((start, end))

    count = 0
    for start, end in overlaps:
        count += len(range(start, end + 1))

    print(count)


if __name__ == "__main__":
    main()
