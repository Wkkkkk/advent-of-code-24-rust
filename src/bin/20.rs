advent_of_code::solution!(20);
use core::hash::Hash;
use std::collections::HashMap;
use std::ops::{Add, Sub};

type CharGrid = grid::Grid<char>;
type DistanceGrid = grid::Grid<i32>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    const fn to(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }

    const fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    const fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }

    fn manhattan(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

const UP: Point = Point::new(-1, 0);
const DOWN: Point = Point::new(1, 0);
const LEFT: Point = Point::new(0, -1);
const RIGHT: Point = Point::new(0, 1);
/// Clockwise order starting with facing right.
const DIRECTIONS: [Point; 4] = [RIGHT, UP, LEFT, DOWN];

fn parse_to_grid(input: &str) -> CharGrid {
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    CharGrid::from_vec(cells, width)
}

fn find(grid: &CharGrid, target: char) -> Option<Point> {
    for (point, i) in grid.indexed_iter() {
        if *i == target {
            return Some(Point::new(point.1 as i32, point.0 as i32));
        }
    }

    None
}

fn distance_from_to(grid: &CharGrid, start: Point, end: Point) -> DistanceGrid {
    let (height, width) = grid.size();
    let mut distance_grid = grid::Grid::init(height, width, i32::MAX);
    let mut elapsed = 0;

    let mut pos = start;
    let mut direction = DIRECTIONS
        .into_iter()
        .find(|&dir| {
            let next_position = pos + dir;
            grid[next_position.to()] != '#'
        })
        .unwrap();

    while pos != end {
        distance_grid[pos.to()] = elapsed;
        elapsed += 1;

        direction = [
            direction,
            direction.clockwise(),
            direction.counter_clockwise(),
        ]
        .into_iter()
        .find(|&dir| {
            let next_position = pos + dir;
            grid[next_position.to()] != '#'
        })
        .unwrap();
        pos = pos + direction;
    }

    distance_grid[end.to()] = elapsed;
    distance_grid
}

fn cheat_from(distances: &DistanceGrid, first: Point, delta: Point) -> u32 {
    let (width, height) = distances.size();
    let second = first + delta;

    if second.x < 0 || second.y < 0 || second.x >= width as i32 || second.y >= height as i32 {
        return 0;
    }

    // Land on track.
    if distances[second.to()] != i32::MAX {
        // We can save time if we can reach the second point in less time.
        let first_time = distances[first.to()];
        let second_time = distances[second.to()];
        let cost = first.manhattan(second);
        let saved = (first_time - second_time).abs();

        return (saved - cost) as u32;
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let (width, height) = grid.size();
    let start = find(&grid, 'S').unwrap();
    let end = find(&grid, 'E').unwrap();
    let distances = distance_from_to(&grid, start, end);

    let path = (1..height - 1)
        .flat_map(|row| (1..width - 1).map(move |col| (row, col)))
        .filter_map(|(row, col)| {
            let point = Point::new(col as i32, row as i32);
            if grid[point.to()] == '#' {
                return None;
            } else {
                return Some(point);
            }
        })
        .collect::<Vec<_>>();

    let mut saved_time_map = HashMap::new();
    for point in path {
        let jumps = vec![Point::new(2, 0), Point::new(0, 2)];
        for jump in jumps {
            let saved_time = cheat_from(&distances, point, jump);
            *saved_time_map.entry(saved_time).or_insert(0) += 1;
        }
    }

    // sum values of keys that are greater than 100
    let cheats = saved_time_map
        .iter()
        .filter(|(key, _)| **key >= 100)
        .map(|(_, value)| *value)
        .sum::<u32>();

    Some(cheats)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let (width, height) = grid.size();
    let start = find(&grid, 'S').unwrap();
    let end = find(&grid, 'E').unwrap();
    let distances = distance_from_to(&grid, start, end);

    let path = (1..height - 1)
        .flat_map(|row| (1..width - 1).map(move |col| (row, col)))
        .filter_map(|(row, col)| {
            let point = Point::new(col as i32, row as i32);
            if grid[point.to()] == '#' {
                return None;
            } else {
                return Some(point);
            }
        })
        .collect::<Vec<_>>();

    let mut saved_time_map = HashMap::new();
    for point in path {
        let mut jumps = Vec::new();
        for x in 2..21 {
            jumps.push(Point::new(x, 0));
        }

        for y in 1..21 {
            for x in (y - 20)..(21 - y) {
                jumps.push(Point::new(x, y));
            }
        }

        for jump in jumps {
            let saved_time = cheat_from(&distances, point, jump);
            *saved_time_map.entry(saved_time).or_insert(0) += 1;
        }
    }

    // sum values of keys that are greater than 100
    let cheats = saved_time_map
        .iter()
        .filter(|(key, _)| **key >= 100)
        .map(|(_, value)| *value)
        .sum::<u32>();

    Some(cheats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
