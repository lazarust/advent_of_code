contents = open("../inputs/day4.txt", "r").read()

split_contents = contents.split("\n")

def find_xmas_shape_occurrences(grid):
    rows = len(grid)
    cols = len(grid[0])

    occurrences = []

    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == 'A':
                while True:
                    dx, dy = (i-1, j+1)

                    if not (0 <= dx < rows and 0 <= dy < cols):
                        break
                    value_to_check = grid[dx][dy]
                    if value_to_check != 'M' and value_to_check != 'S':
                        break

                    dx, dy = (i+1, j-1)
                    if not (0 <= dx < rows and 0 <= dy < cols):
                        break
                    new_value_to_check = grid[dx][dy]
                    if value_to_check == new_value_to_check or (value_to_check == "S" and new_value_to_check != "M") or (value_to_check == "M" and new_value_to_check != "S"):
                        break

                    dx, dy = (i-1, j-1)
                    if not (0 <= dx < rows and 0 <= dy < cols):
                        break
                    value_to_check = grid[dx][dy]
                    if value_to_check != 'M' and value_to_check != 'S':
                        break

                    dx, dy = (i+1, j+1)
                    if not (0 <= dx < rows and 0 <= dy < cols):
                        break
                    new_value_to_check = grid[dx][dy]
                    if value_to_check == new_value_to_check or (value_to_check == "S" and new_value_to_check != "M") or (value_to_check == "M" and new_value_to_check != "S"):
                        break


                    occurrences.append((i, j))
                    break

    return occurrences

print(len(find_xmas_shape_occurrences(split_contents[:-1])))
