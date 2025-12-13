# /// script
# requires-python = ">=3.11"
# dependencies = ["shapely"]
# ///

import sys

from shapely import box
from shapely.geometry import Polygon


def main() -> None:
    use_test = "test" in sys.argv
    filename = "./inputs/test_day9.txt" if use_test else "./inputs/day9.txt"

    with open(filename) as f:
        raw_data = f.read().strip()

    rows = raw_data.splitlines()

    cords = []
    for row in rows:
        split_row = row.split(",")
        cords.append((int(split_row[0]), int(split_row[1])))

    poly = Polygon(cords)

    max_area = 0
    for i, (x1, y1) in enumerate(cords):
        for (x2, y2) in cords[:i]:
            rect = box(min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2))
            if poly.contains(rect):
                area = (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)
                max_area = max(max_area, area)

    print(max_area)


if __name__ == "__main__":
    main()
