use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn challenge() {
    let file = File::open("../inputs/day8.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    if grid.last().unwrap().is_empty() {
        grid.pop();
    }

    let n = grid.len();
    let m = grid[0].len();

    let mut nodes: std::collections::HashMap<char, Vec<(usize, usize)>> =
        std::collections::HashMap::new();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '.' {
                nodes.entry(grid[i][j]).or_insert(Vec::new()).push((i, j));
            }
        }
    }

    let mut antinodes: std::collections::HashSet<(isize, isize)> = std::collections::HashSet::new();

    fn antinode(
        antinodes: &mut std::collections::HashSet<(isize, isize)>,
        pr1: (isize, isize),
        pr2: (isize, isize),
        n: usize,
        m: usize,
    ) {
        let (x1, y1) = pr1;
        let (x2, y2) = pr2;
        let (dx, dy) = (x2 - x1, y2 - y1);
        let mut newx = x2 + dx;
        let mut newy = y2 + dy;
        antinodes.insert((x2, y2));
        while newx >= 0 && newx < n as isize && newy >= 0 && newy < m as isize {
            antinodes.insert((newx, newy));
            newx += dx;
            newy += dy;
        }
    }

    for node_list in nodes.values() {
        for i in 0..node_list.len() {
            for j in 0..i {
                let node1 = (node_list[i].0 as isize, node_list[i].1 as isize);
                let node2 = (node_list[j].0 as isize, node_list[j].1 as isize);
                antinode(&mut antinodes, node1, node2, n, m);
                antinode(&mut antinodes, node2, node1, n, m);
            }
        }
    }

    println!("{}", antinodes.len());
}
