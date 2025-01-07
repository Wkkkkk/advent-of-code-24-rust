advent_of_code::solution!(19);

use std::collections::{HashMap, HashSet};

struct Patterns<'a> {
    patterns: HashSet<&'a str>,
}

impl<'a> Patterns<'a> {
    fn from_str(input: &'a str) -> Self {
        let patterns = input.split(", ").collect::<HashSet<_>>();
        Self { patterns }
    }

    fn is_match(&self, design: &str) -> bool {
        if design.is_empty() {
            return true;
        }

        for &pattern in &self.patterns {
            if design.starts_with(pattern) {
                let rest = &design[pattern.len()..];
                if self.is_match(rest) {
                    return true;
                }
            }
        }

        false
    }

    fn count_match(&self, design: &str) -> usize {
        let mut cache = HashMap::new();
        self.count_match_impl(design, &mut cache)
    }

    fn count_match_impl(&self, design: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(&count) = cache.get(design) {
            return count;
        }

        let count = self
            .patterns
            .iter()
            .filter_map(|&pattern| {
                if design.starts_with(pattern) {
                    let rest = &design[pattern.len()..];
                    Some(self.count_match_impl(rest, cache))
                } else {
                    None
                }
            })
            .sum();

        cache.insert(design, count);
        count
    }
}

struct Designs<'a> {
    designs: Vec<&'a str>,
}

impl<'a> Designs<'a> {
    fn from_str(input: &'a str) -> Self {
        let designs = input.lines().collect::<Vec<_>>();
        Self { designs }
    }

    fn is_matching(&self, patterns: &Patterns) -> usize {
        self.designs
            .iter()
            .filter(|design| patterns.is_match(design))
            .count()
    }

    fn count_matching(&self, patterns: &Patterns) -> usize {
        self.designs
            .iter()
            .map(|design| patterns.count_match(design))
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (patterns, design) = input.split_once("\n\n").unwrap();
    let patterns = Patterns::from_str(patterns);
    let designs = Designs::from_str(design);
    Some(designs.is_matching(&patterns) as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, design) = input.split_once("\n\n").unwrap();
    let patterns = Patterns::from_str(patterns);
    let designs = Designs::from_str(design);
    Some(designs.count_matching(&patterns) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
