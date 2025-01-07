advent_of_code::solution!(18);
use core::hash::Hash;
use priority_queue::PriorityQueue;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::ops::{Add, Sub};

type CharGrid = grid::Grid<char>;

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

fn parse_to_grid(input: &str, size: usize, num: usize) -> CharGrid {
    let mut grid = CharGrid::init(size, size, '.');
    let bytes = input
        .lines()
        // Parse line like '5, 4' into a Point.
        .map(|line| {
            let mut iter = line.split(",");
            let x = iter.next().unwrap().parse::<i32>().unwrap();
            let y = iter.next().unwrap().parse::<i32>().unwrap();

            Point::new(x, y)
        })
        .take(num)
        .collect::<Vec<Point>>();

    for point in bytes {
        grid[point.to()] = '#';
    }

    grid
}

#[allow(dead_code)]
fn print_grid(grid: &CharGrid) {
    for ((_row, col), i) in grid.indexed_iter() {
        print!("{i}");
        if col == grid.size().0 - 1 {
            println!();
        }
    }
}

#[allow(dead_code)]
fn print_bool_grid(grid: &grid::Grid<bool>) {
    println!("-----------------");
    let (_max_row, max_col) = (grid.size().0, grid.size().1);
    for ((_row, col), b) in grid.indexed_iter() {
        print!("{}", if *b { 'O' } else { '.' });
        if col == max_col - 1 {
            println!();
        }
    }
}

fn minimum_cost_from_to(grid: &CharGrid, start: Point, end: Point) -> (usize, usize) {
    let (height, width) = grid.size();
    let mut unvisited = PriorityQueue::new();
    unvisited.push(start, Reverse(0));

    let mut seen = grid::Grid::init(height, width, u32::MAX);
    let mut lowest_cost = u32::MAX;
    seen[start.to()] = 0;

    while !unvisited.is_empty() {
        while let Some((position, Reverse(cost))) = unvisited.pop() {
            if cost >= lowest_cost {
                continue;
            }

            if position == end {
                lowest_cost = cost;
                continue;
            }

            let next = DIRECTIONS
                .iter()
                .filter_map(|&dir| {
                    let next_position = position + dir;
                    if next_position.x >= 0
                        && next_position.y >= 0
                        && next_position.x < width as i32
                        && next_position.y < height as i32
                    {
                        Some((next_position, cost + 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for (next_position, next_cost) in next {
                if grid[next_position.to()] != '#' && next_cost < seen[next_position.to()] {
                    unvisited.push(next_position, Reverse(next_cost));
                    seen[next_position.to()] = next_cost;
                }
            }
        }
    }

    // Backwards BFS
    let mut visited = VecDeque::new();
    let mut path = grid::Grid::init(height, width, false);

    if seen[end.to()] == lowest_cost {
        visited.push_back((end, lowest_cost));
    }

    while let Some((position, cost)) = visited.pop_front() {
        path[position.to()] = true;
        if position == start {
            break;
        }

        // Reverse direction and subtract cost.
        let next = DIRECTIONS
            .iter()
            .filter_map(|&dir| {
                let next_position = position - dir;
                if next_position.x >= 0
                    && next_position.y >= 0
                    && next_position.x < width as i32
                    && next_position.y < height as i32
                {
                    Some((next_position, cost - 1))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for (next_position, next_cost) in next {
            // Trace our cost step by step so it will exactly match possible paths.
            if next_cost == seen[next_position.to()] {
                visited.push_back((next_position, next_cost));
                // Set cost back to `u32::MAX` to prevent redundant path explorations.
                seen[next_position.to()] = u32::MAX;
            }
        }
    }

    // print_bool_grid(&path);
    (
        lowest_cost as usize,
        path.iter().filter(|b| **b).count() as usize,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input, 71, 1024);
    let (cost, _) = minimum_cost_from_to(&grid, Point::new(0, 0), Point::new(70, 70));

    Some(cost as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let max_length = input.lines().count();
    let num = (1..max_length).into_par_iter().find_first(|n| {
        let grid = parse_to_grid(input, 71, *n);
        let (cost, _) = minimum_cost_from_to(&grid, Point::new(0, 0), Point::new(70, 70));
        cost == u32::MAX as usize
    });

    let byte = input.lines().nth(num.unwrap() - 1).unwrap().to_string();
    Some(byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
