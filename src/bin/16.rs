use core::hash::Hash;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::ops::{Add, Sub};

advent_of_code::solution!(16);

type CharGrid = grid::Grid<char>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

const UP: Point = Point::new(-1, 0);
const DOWN: Point = Point::new(1, 0);
const LEFT: Point = Point::new(0, -1);
const RIGHT: Point = Point::new(0, 1);
/// Clockwise order starting with facing right.
const DIRECTIONS: [Point; 4] = [RIGHT, UP, LEFT, DOWN];

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    const fn to(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
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

fn print_bool_grid(grid: &grid::Grid<bool>) {
    let (_max_row, max_col) = (grid.size().0, grid.size().1);
    for ((_row, col), b) in grid.indexed_iter() {
        print!("{}", if *b { 'O' } else { '.' });
        if col == max_col - 1 {
            println!();
        }
    }
}

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
            return Some(Point::new(point.0 as i32, point.1 as i32));
        }
    }

    None
}

fn minimum_cost_from_to(grid: &CharGrid, start: Point, end: Point) -> (usize, usize) {
    let (height, width) = grid.size();
    let mut unvisited = PriorityQueue::new();
    unvisited.push((start, 0), Reverse(0));

    let mut seen = grid::Grid::init(height, width, [u32::MAX; 4]);
    let mut lowest_cost = u32::MAX;
    seen[start.to()][0] = 0;

    while !unvisited.is_empty() {
        while let Some(((position, direction), Reverse(cost))) = unvisited.pop() {
            if cost >= lowest_cost {
                continue;
            }

            if position == end {
                lowest_cost = cost;
                continue;
            }

            let left = (direction + 3) % 4;
            let right = (direction + 1) % 4;
            let next = [
                // Move forward
                (position + DIRECTIONS[direction], direction, cost + 1),
                (position, left, cost + 1000),
                (position, right, cost + 1000),
            ];

            for (next_position, next_direction, next_cost) in next {
                if grid[next_position.to()] != '#'
                    && next_cost < seen[next_position.to()][next_direction]
                {
                    unvisited.push((next_position, next_direction), Reverse(next_cost));
                    seen[next_position.to()][next_direction] = next_cost;
                }
            }
        }
    }

    // Backwards BFS
    let mut visited = VecDeque::new();
    let mut path = grid::Grid::init(height, width, false);

    // Lowest paths can arrive at end node in multiple directions.
    for direction in 0..4 {
        if seen[end.to()][direction] == lowest_cost {
            visited.push_back((end, direction, lowest_cost));
        }
    }

    while let Some((position, direction, cost)) = visited.pop_front() {
        path[position.to()] = true;
        if position == start {
            break;
        }

        // Reverse direction and subtract cost.
        let left = (direction + 3) % 4;
        let right = (direction + 1) % 4;
        let next = [
            (position - DIRECTIONS[direction], direction, cost - 1),
            (position, left, cost - 1000),
            (position, right, cost - 1000),
        ];

        for (next_position, next_direction, next_cost) in next {
            // Trace our cost step by step so it will exactly match possible paths.
            if next_cost == seen[next_position.to()][next_direction] {
                visited.push_back((next_position, next_direction, next_cost));
                // Set cost back to `u32::MAX` to prevent redundant path explorations.
                seen[next_position.to()][next_direction] = u32::MAX;
            }
        }
    }

    // print_bool_grid(&path);
    (
        lowest_cost as usize,
        path.iter().filter(|b| **b).count() as usize,
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_to_grid(input);
    let start = find(&grid, 'S').unwrap();
    let end = find(&grid, 'E').unwrap();
    let (minimum_cost, _) = minimum_cost_from_to(&grid, start, end);
    println!(
        "Going from {:?} to {:?}, minimum_cost {:?}",
        start, end, minimum_cost
    );

    Some(minimum_cost)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_to_grid(input);
    let start = find(&grid, 'S').unwrap();
    let end = find(&grid, 'E').unwrap();
    let (_, total_length) = minimum_cost_from_to(&grid, start, end);
    println!(
        "Going from {:?} to {:?}, total_length {:?}",
        start, end, total_length
    );

    Some(total_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
