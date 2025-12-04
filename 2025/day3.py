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
        first_max = 0
        second_max = 0
        for i, char in enumerate(bank):
            voltage = int(char)
            if voltage > first_max and i < len(bank) - 1:
                first_max = voltage
                second_max = 0
            elif voltage > second_max:
                second_max = voltage

        num_to_add = int(str(first_max) + str(second_max))
        running_sum += num_to_add

    print(f"Answer: {running_sum}")


if __name__ == "__main__":
    main()
