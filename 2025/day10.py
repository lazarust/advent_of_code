# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "scipy",
# ]
# ///

import sys

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp


def main() -> None:
    use_test = "test" in sys.argv
    filename = "./inputs/test_day10.txt" if use_test else "./inputs/day10.txt"

    with open(filename) as f:
        raw_data = f.read().strip()

    rows = raw_data.splitlines()

    running_sum = 0
    for row in rows:
        split_row = row.split("]")
        rest = split_row[1].split("{")
        buttons = rest[0].strip().split(" ")

        joltage_str = rest[1].replace("}", "").strip()
        target_joltage_levels = tuple(int(x) for x in joltage_str.split(","))

        parsed_buttons = []
        for b in buttons:
            indices = tuple(
                int(x) for x in b.replace("(", "").replace(")", "").split(",")
            )
            parsed_buttons.append(indices)

        num_buttons = len(parsed_buttons)
        num_counters = len(target_joltage_levels)

        zeros = np.zeros((num_counters, num_buttons))
        for i, button_indices in enumerate(parsed_buttons):
            for j in button_indices:
                zeros[j][i] = 1

        c = np.ones(num_buttons)

        target_array = np.array(target_joltage_levels, dtype=float)
        constraints = LinearConstraint(zeros, target_array, target_array)

        bounds = Bounds(lb=0, ub=np.inf)
        integrality = np.ones(num_buttons)
        result = milp(
            c, constraints=constraints, bounds=bounds, integrality=integrality
        )

        if result.success:
            running_sum += int(round(result.fun))

    print(running_sum)


if __name__ == "__main__":
    main()
