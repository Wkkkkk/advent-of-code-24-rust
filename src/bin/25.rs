advent_of_code::solution!(25);

type Grid = grid::Grid<char>;

fn parse_to_grid(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    Grid::from_vec(cells, width)
}
struct Key(Vec<u32>);

impl Key {
    fn try_from(input: &str) -> Option<Self> {
        let grid = parse_to_grid(input);
        let (height, width) = grid.size();
        if grid[(0, 0)] != '.' {
            return None;
        }

        let mut key = Vec::new();
        for x in 0..width {
            let mut value = 0;
            for y in 0..height {
                if grid[(y, x)] == '#' {
                    value += 1;
                }
            }
            key.push(value);
        }

        Some(Self(key))
    }
}

struct Lock(Vec<u32>);

impl Lock {
    fn try_from(input: &str) -> Option<Self> {
        let grid = parse_to_grid(input);
        let (height, width) = grid.size();
        if grid[(0, 0)] != '#' {
            return None;
        }

        let mut key = Vec::new();
        for x in 0..width {
            let mut value = 0;
            for y in 0..height {
                if grid[(y, x)] == '#' {
                    value += 1;
                }
            }
            key.push(value);
        }

        Some(Self(key))
    }
}

enum KeyLock {
    Key(Key),
    Lock(Lock),
}

fn parse_input(input: &str) -> Vec<KeyLock> {
    let elements = input.split("\n\n").collect::<Vec<_>>();

    elements
        .iter()
        .filter_map(|element| {
            if element.starts_with("....") {
                Some(KeyLock::Key(Key::try_from(element).unwrap()))
            } else if element.starts_with("#####") {
                Some(KeyLock::Lock(Lock::try_from(element).unwrap()))
            } else {
                None
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elements = parse_input(input);
    let keys = elements.iter().filter_map(|element| match element {
        KeyLock::Key(key) => Some(key),
        _ => None,
    });
    let locks = elements.iter().filter_map(|element| match element {
        KeyLock::Lock(lock) => Some(lock),
        _ => None,
    });

    let mut result = 0;
    // Iterate over all key and lock pairs
    for key in keys {
        for lock in locks.clone() {
            if key.0.iter().zip(&lock.0).all(|(k, l)| k + l <= 5 + 2) {
                result += 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_try_from() {
        let input = ".....
#....
#....
#...#
#.#.#
#.###
#####";
        let key = Key::try_from(input).unwrap();
        assert_eq!(key.0, vec![6, 1, 3, 2, 4]);
    }

    #[test]
    fn test_lock_try_from() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....";
        let lock = Lock::try_from(input).unwrap();
        assert_eq!(lock.0, vec![1, 6, 4, 5, 4]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
