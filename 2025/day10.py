# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

import sys


def main() -> None:
    use_test = "test" in sys.argv
    filename = "./inputs/test_day10.txt" if use_test else "./inputs/day10.txt"

    with open(filename) as f:
        raw_data = f.read().strip()

    rows = raw_data.splitlines()

    running_sum = 0
    for row in rows:
        split_row = row.split("]")
        lights = split_row[0]
        rest = split_row[1].split("{")
        buttons = rest[0].strip().split(" ")
        target_lights = lights.replace("[", "")
        initial_lights = "." * len(target_lights)

        queue = [(initial_lights, 0, 0)]

        while queue:
            lights_state, num_presses, start_loc = queue.pop(0)
            for i in range(start_loc, len(buttons)):
                test_lights = list(lights_state)
                flips = buttons[i].replace("(", "").replace(")", "").split(",")
                for flip in flips:
                    int_flip = int(flip)
                    test_lights[int_flip] = "#" if test_lights[int_flip] == "." else "."

                lights = "".join(test_lights)

                if lights == target_lights:
                    running_sum += num_presses + 1
                    queue = None
                    break

                queue.append((lights, num_presses + 1, i + 1))

    print(running_sum)


if __name__ == "__main__":
    main()
