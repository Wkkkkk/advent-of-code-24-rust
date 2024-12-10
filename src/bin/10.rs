use std::collections::HashSet;

use grid::*;

advent_of_code::solution!(10);

fn parse_to_grid(input: &str) -> Grid<u32> {
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    Grid::from_vec(cells, width)
}

fn find_start_positions(grid: &Grid<u32>) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for ((row, col), i) in grid.indexed_iter() {
        if *i == 0 {
            positions.push((col, row));
        }
    }

    positions
}

fn find_path(
    grid: &Grid<u32>,
    start: (usize, usize),
) -> HashSet<((usize, usize), Vec<(usize, usize)>)> {
    // Find the path from the start position to any end position with value 9.
    let mut reachable_ends = HashSet::new();

    let mut stack = vec![(start, vec![start])];
    while let Some(((col, row), mut trace)) = stack.pop() {
        let current_height = grid[(row, col)];
        if current_height == 9 {
            reachable_ends.insert(((col, row), trace));
            continue;
        }

        let mut next = Vec::new();
        if col > 0 {
            next.push((col - 1, row));
        }
        if col < grid.size().1 - 1 {
            next.push((col + 1, row));
        }
        if row > 0 {
            next.push((col, row - 1));
        }
        if row < grid.size().0 - 1 {
            next.push((col, row + 1));
        }

        for (col, row) in next {
            let next_height = grid[(row, col)];
            if next_height == current_height + 1 {
                trace.push((col, row));
                stack.push(((col, row), trace.clone()));
            }
        }
    }

    reachable_ends
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let start_positions = find_start_positions(&grid);
    let score = start_positions
        .iter()
        .map(|start| {
            find_path(&grid, *start)
                .iter()
                .map(|(end, _path)| end)
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>();

    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let start_positions = find_start_positions(&grid);
    let score = start_positions
        .iter()
        .map(|start| find_path(&grid, *start))
        .inspect(|paths| {
            for (end, path) in paths.iter() {
                println!("End: {:?}, Path: {:?}", end, path);
            }
        })
        .map(|paths| paths.len())
        .sum::<usize>();

    Some(score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
