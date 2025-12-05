# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///


def main() -> None:
    # with open("./inputs/test_day3.txt") as f:
    with open("./inputs/day3.txt") as f:
        raw_data = f.read().strip()

    banks = raw_data.split("\n")

    running_sum = 0
    for bank in banks:
        running_str = ""
        pos = 0
        for i in range(12):
            remaining_needed = 12 - i - 1
            max_digit = max(bank[pos : len(bank) - remaining_needed])
            running_str += max_digit
            pos = bank.index(max_digit, pos) + 1

        running_sum += int(running_str)

    print(f"Answer: {running_sum}")


if __name__ == "__main__":
    main()
