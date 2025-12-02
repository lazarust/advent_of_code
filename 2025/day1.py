# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///


def main() -> None:
    with open("./inputs/day1.txt") as f:
        data = f.read()

    datas = data.replace("R", "").replace("L", "-").strip().split("\n")

    temp_num = 50
    count = 0
    for num in datas:
        float_num = float(num)

        if abs(float_num) >= 100:
            hundreds = int((abs(float_num) // 100) * 100)
            if float_num < 0:
                float_num += hundreds
            else:
                float_num -= hundreds

        temp_num += float_num
        if temp_num < 0:
            temp_num += 100
        elif temp_num >= 100:
            temp_num -= 100

        if temp_num == 0:
            count += 1

    print(count)


if __name__ == "__main__":
    main()
