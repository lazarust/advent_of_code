# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

import math
import sys


def main() -> None:
    use_test = "test" in sys.argv
    filename = "./inputs/test_day8.txt" if use_test else "./inputs/day8.txt"

    with open(filename) as f:
        raw_data = f.read().strip()

    coordinates = []
    for row in raw_data.splitlines():
        x, y, z = row.split(",")
        coordinates.append((int(x), int(y), int(z)))

    pairs = []
    for i in range(len(coordinates)):
        for j in range(i + 1, len(coordinates)):
            cord1, cord2 = coordinates[i], coordinates[j]
            dist = math.sqrt(
                (cord2[0] - cord1[0]) ** 2
                + (cord2[1] - cord1[1]) ** 2
                + (cord2[2] - cord1[2]) ** 2
            )
            pairs.append((dist, i, j))

    pairs.sort()

    parent = {i: i for i in range(len(coordinates))}
    size = {i: 1 for i in range(len(coordinates))}

    for dist, i, j in pairs[:1000]:
        root_i = i
        while parent[root_i] != root_i:
            root_i = parent[root_i]
        x = i
        while parent[x] != root_i:
            parent[x], x = root_i, parent[x]

        root_j = j
        while parent[root_j] != root_j:
            root_j = parent[root_j]
        x = j
        while parent[x] != root_j:
            parent[x], x = root_j, parent[x]

        if root_i != root_j:
            if size[root_i] < size[root_j]:
                root_i, root_j = root_j, root_i
            parent[root_j] = root_i
            size[root_i] += size[root_j]

    circuit_sizes = [size[i] for i in range(len(coordinates)) if parent[i] == i]
    circuit_sizes.sort(reverse=True)

    result = circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
    print(result)


if __name__ == "__main__":
    main()
