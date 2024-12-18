use std::collections::HashSet;

use grid::*;
use im::HashMap;

advent_of_code::solution!(12);

fn parse_to_grid(input: &str) -> Grid<char> {
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    Grid::from_vec(cells, width)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let (width, height) = grid.size();

    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let adjacent_to_perimeter: HashMap<usize, usize> = [(0, 4), (1, 3), (2, 2), (3, 1), (4, 0)]
        .iter()
        .cloned()
        .collect();

    let mut unknown = Vec::new();
    for ((row, col), _) in grid.indexed_iter() {
        unknown.push((row as i32, col as i32));
    }

    let mut total_price = 0;
    while !unknown.is_empty() {
        let position = unknown.pop().unwrap();
        let mut queue = vec![position];
        let mut visited = HashSet::new();

        while !queue.is_empty() {
            let now = queue.pop().unwrap();
            let plant = grid.get(now.0, now.1).unwrap();
            visited.insert(now);

            for direction in &directions {
                let next = (now.0 as i32 + direction.0, now.1 as i32 + direction.1);
                if next.0 < 0 || next.1 < 0 || next.0 >= width as i32 || next.1 >= height as i32 {
                    continue;
                }

                let next_plant = grid.get(next.0 as usize, next.1 as usize).unwrap();
                if next_plant == plant && !visited.contains(&next) {
                    queue.push(next);
                }
            }
        }

        let area = visited.len();
        let mut perimeter = 0;
        for position in &visited {
            let neighbor = directions
                .iter()
                .map(|direction| {
                    let next = (position.0 + direction.0, position.1 + direction.1);
                    if next.0 < 0 || next.1 < 0 || next.0 >= width as i32 || next.1 >= height as i32
                    {
                        return 0;
                    }

                    if visited.contains(&next) {
                        return 1;
                    }

                    0
                })
                .sum::<usize>();

            let adjacent = adjacent_to_perimeter.get(&neighbor).unwrap();
            perimeter += *adjacent;
        }

        total_price += area * perimeter;

        for position in visited {
            unknown.retain(|&x| x != position);
        }
    }

    Some(total_price as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // skip part two for now
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }
}
