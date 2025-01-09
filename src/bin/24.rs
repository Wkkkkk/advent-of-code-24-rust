advent_of_code::solution!(24);
use std::collections::{HashMap, VecDeque};

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<[String; 5]>) {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let mut values = HashMap::new();
    for line in prefix.lines() {
        let (key, value) = line.split_once(": ").unwrap();
        values.insert(key.to_string(), value == "1");
    }

    let gates = suffix
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            // Collect parts into an array
            let mut arr = [const { String::new() }; 5];
            for i in 0..5 {
                arr[i] = parts.next().unwrap().to_string();
            }
            arr
        })
        .collect();

    (values, gates)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (values, gates) = parse_input(input);

    let mut todo: VecDeque<_> = gates.iter().collect();
    let mut values = values;

    while let Some(gate @ [left, kind, right, _, to]) = todo.pop_front() {
        let left = values.get(left);
        let right = values.get(right);

        if left.is_some() && right.is_some() {
            let new_value = match kind.as_str() {
                "AND" => left.unwrap() & right.unwrap(),
                "OR" => left.unwrap() | right.unwrap(),
                "XOR" => left.unwrap() ^ right.unwrap(),
                _ => unreachable!(),
            };
            values.insert(to.to_string(), new_value);
        } else {
            todo.push_back(gate);
        }
    }

    // filter out key-value pairs whose key start with z
    let mut gates_z = values
        .iter()
        .filter(|(key, _)| key.starts_with('z'))
        .collect::<Vec<_>>();
    gates_z.sort_by_key(|(key, _)| (*key).clone());

    let mut result = 0;
    for (key, &value) in gates_z.into_iter().rev() {
        println!("{}: {}", key, value);
        result = (result << 1) | (value as u64);
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
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
