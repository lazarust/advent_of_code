# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///


def main() -> None:
    with open("./inputs/day1.txt") as f:
        # with open("./inputs/test_day1.txt") as f:
        data = f.read()

    datas = data.replace("R", "").replace("L", "-").strip().split("\n")

    temp_num = 50
    inc_count = False
    count = 0
    for num in datas:
        float_num = float(num)

        if abs(float_num) >= 100:
            hundreds = int((abs(float_num) // 100) * 100)
            if float_num < 0:
                float_num += hundreds
            else:
                float_num -= hundreds
            count += hundreds / 100

        temp_num += float_num
        if temp_num < 0:
            temp_num += 100
            if inc_count:
                count += 1
        elif temp_num > 100:
            temp_num -= 100
            if inc_count:
                count += 1

        if temp_num == 0:
            if inc_count:
                count += 1
            inc_count = False
        elif temp_num == 100:
            temp_num = 0
            if inc_count:
                count += 1
            inc_count = True
        else:
            inc_count = True

    print(count)


if __name__ == "__main__":
    main()
