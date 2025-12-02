# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///


def main() -> None:
    # with open("./inputs/test_day2.txt") as f:
    with open("./inputs/day2.txt") as f:
        raw_data = f.read()

    ranges = raw_data.strip().replace("\n", "").split(",")

    running_sum = 0
    for raw_range in ranges:
        split_range = raw_range.split("-")
        lower_bound = int(split_range[0])
        higher_bound = int(split_range[1])
        for num in range(lower_bound, higher_bound + 1):
            str_num = str(num)
            if len(str_num) % 2 != 0:
                continue
            first_half = str_num[: (len(str_num) // 2)]
            second_half = str_num[(len(str_num) // 2) :]
            if first_half == second_half:
                running_sum += num

    print(f"Answer: {running_sum}")


if __name__ == "__main__":
    main()
