use std::collections::HashSet;
use std::fs;

pub fn challenge() -> Result<(), Box<dyn std::error::Error>> {
    let contents =
        fs::read_to_string("../inputs/day6.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();

    let mut current_position = (0, 0);
    'outer: for (i, row) in grid.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == '^' {
                current_position = (i as isize, j as isize);
                break 'outer;
            }
        }
    }
    visited_positions.insert((current_position.0, current_position.1));

    let mut direction = "up";
    loop {
        match direction {
            "up" => {
                let next_position = (current_position.0 - 1, current_position.1);
                if next_position.0 >= 0
                    && grid[next_position.0 as usize][next_position.1 as usize] == '#'
                {
                    direction = "right";
                } else {
                    current_position = next_position;
                }
            }
            "right" => {
                let next_position = (current_position.0, current_position.1 + 1);
                if next_position.1 <= grid[next_position.0 as usize].len() as isize
                    && grid[next_position.0 as usize][next_position.1 as usize] == '#'
                {
                    direction = "down";
                } else {
                    current_position = next_position;
                }
            }
            "down" => {
                let next_position = (current_position.0 + 1, current_position.1);
                if next_position.0 < grid.len() as isize
                    && grid[next_position.0 as usize][next_position.1 as usize] == '#'
                {
                    direction = "left";
                } else {
                    current_position = next_position;
                }
            }
            "left" => {
                let next_position = (current_position.0, current_position.1 - 1);
                if next_position.1 >= 0
                    && grid[next_position.0 as usize][next_position.1 as usize] == '#'
                {
                    direction = "up";
                } else {
                    current_position = next_position;
                }
            }
            _ => unreachable!(),
        }

        if (current_position.0 < 0
            || current_position.0 >= grid.len() as isize
            || current_position.1 < 0
            || current_position.1 >= grid[current_position.0 as usize].len() as isize)
        {
            break;
        }
        visited_positions.insert((current_position.0, current_position.1));
    }

    println!("Number of positions moved: {}", visited_positions.len());

    Ok(())
}
