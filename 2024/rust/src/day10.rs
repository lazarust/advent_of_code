use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn challenge() {
    let map =
        fs::read_to_string("../inputs/day10.txt").expect("Should have been able to read the file");
    let rows = map.lines().count();
    if rows == 0 {
        return;
    }
    let cols = map.lines().next().unwrap().len();

    let mut height_map: Vec<Vec<u8>> = Vec::with_capacity(rows);
    for line in map.lines() {
        height_map.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn bfs(
        start: (usize, usize),
        height_map: &Vec<Vec<u8>>,
        rows: usize,
        cols: usize,
        directions: [(i32, i32); 4],
    ) -> u32 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert((start.0, start.1));
        queue.push_back((start.0, start.1));

        let mut count_9s = 0;

        while let Some((x, y)) = queue.pop_front() {
            if height_map[x][y] == 9 {
                count_9s += 1;
            } else {
                for &(dx, dy) in &directions {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if !visited.contains(&(nx, ny))
                            && height_map[x][y] + 1 == height_map[nx][ny]
                        {
                            visited.insert((nx, ny));
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }

        count_9s
    }

    // Find all trailheads (height 0) and calculate their scores
    let mut total_score = 0;
    for i in 0..rows {
        for j in 0..cols {
            if height_map[i][j] == 0 {
                total_score += bfs((i, j), &height_map, rows, cols, directions);
            }
        }
    }

    println!("Total Score: {}", total_score);
}
