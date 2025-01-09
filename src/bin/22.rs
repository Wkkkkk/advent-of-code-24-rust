use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn mix(value: i64, secret: i64) -> i64 {
    value ^ secret
}

fn prune(secret: i64) -> i64 {
    secret.rem_euclid(16777216)
}

fn next(secret: i64) -> i64 {
    // Step1
    let multiply = secret * 64;
    let secret = mix(multiply, secret);
    let secret = prune(secret);

    // Step2
    let divide = secret / 32;
    let secret = mix(divide, secret);
    let secret = prune(secret);

    // Step3
    let multiply = secret * 2048;
    let secret = mix(multiply, secret);
    let secret = prune(secret);

    secret
}

pub fn part_one(input: &str) -> Option<i64> {
    let secrets = parse_input(input);

    secrets
        .into_par_iter()
        .map(|secret| {
            let mut secret = secret;
            for _ in 0..2000 {
                secret = next(secret);
            }
            secret as i64
        })
        .sum::<i64>()
        .into()
}

pub fn part_two(input: &str) -> Option<i64> {
    let secrets = parse_input(input);
    let mut profits: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    for secret in secrets {
        let mut secret = secret;
        let mut prices = [0; 2000];
        for i in 0..2000 {
            secret = next(secret);
            prices[i] = secret.rem_euclid(10);
        }

        let mut seen = HashSet::new();
        for (a, b, c, d, e) in prices.iter().tuple_windows() {
            let diffs = ((b - a), (c - b), (d - c), (e - d));

            if seen.insert(diffs) {
                *profits.entry(diffs).or_default() += *e;
            }
        }
    }

    profits.into_values().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(15, 42), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_next() {
        assert_eq!(next(123), 15887950);
        assert_eq!(next(15887950), 16495136);
        assert_eq!(next(16495136), 527345);
        assert_eq!(next(527345), 704524);
        assert_eq!(next(704524), 1553684);
        assert_eq!(next(1553684), 12683156);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
