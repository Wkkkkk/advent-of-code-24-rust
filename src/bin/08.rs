use std::collections::{HashMap, HashSet};

use grid::*;
use itertools::Itertools; // 0.8.2

advent_of_code::solution!(8);

fn parse_to_grid(input: &str) -> Grid<char> {
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    Grid::from_vec(cells, width)
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Antenna {
    x: i64,
    y: i64,
}

fn find_antennas(grid: &Grid<char>) -> HashMap<char, HashSet<Antenna>> {
    let mut antennas = HashMap::new();
    for ((row, col), i) in grid.indexed_iter() {
        if *i != '.' {
            let antenna = Antenna {
                x: col as i64,
                y: row as i64,
            };

            antennas
                .entry(*i)
                .or_insert_with(HashSet::new)
                .insert(antenna);
        }
    }

    antennas
}

fn project(antenna1: &Antenna, antenna2: &Antenna, grid: &Grid<char>) -> Option<Antenna> {
    let (width, height) = grid.size();
    let x = antenna2.x + antenna2.x - antenna1.x;
    let y = antenna2.y + antenna2.y - antenna1.y;

    if x < 0 || x >= width as i64 || y < 0 || y >= height as i64 {
        return None;
    }

    Some(Antenna { x, y })
}

fn project2(antenna1: &Antenna, antenna2: &Antenna, grid: &Grid<char>) -> HashSet<Antenna> {
    let mut results = HashSet::new();
    let mut from = *antenna1;
    let mut to = *antenna2;

    while let Some(new) = project(&from, &to, grid) {
        from = to;
        to = new;

        results.insert(new);
    }

    results
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let antennas = find_antennas(&grid);

    let locations = antennas
        .values()
        .flat_map(|value| {
            value
                .iter()
                .permutations(2)
                .unique()
                .filter_map(|perm| project(perm[0], perm[1], &grid))
        })
        .collect::<HashSet<_>>();

    Some(locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let antennas = find_antennas(&grid);

    let locations = antennas
        .values()
        .flat_map(|value| {
            value
                .iter()
                .permutations(2)
                .unique()
                .flat_map(|perm| project2(perm[0], perm[1], &grid))
        })
        .chain(antennas.values().flatten().cloned())
        .collect::<HashSet<_>>();

    Some(locations.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
