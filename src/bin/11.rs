use im::HashMap;

advent_of_code::solution!(11);

fn read_stones(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn digits(num: u64) -> usize {
    num.to_string().len()
}

fn is_even_digits(num: u64) -> bool {
    digits(num) % 2 == 0
}

fn split_stone(num: u64) -> (u64, u64) {
    let num_str = num.to_string();
    let half = num_str.len() / 2;
    let (left, right) = num_str.split_at(half);
    (left.parse().unwrap(), right.parse().unwrap())
}

fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    if !is_even_digits(stone) {
        return vec![stone * 2024];
    }

    let (left, right) = split_stone(stone);
    vec![left, right]
}

fn blink_cached(cache: &mut HashMap<(u64, u64), u64>, stone: u64, blinks: u64) -> u64 {
    if blinks == 1 {
        if stone == 0 {
            return 1;
        }
        return if is_even_digits(stone) { 2 } else { 1 };
    }

    // Build a cache with the key (stone, blinks) and the number of split stones
    let key = (stone, blinks);
    if let Some(&count) = cache.get(&key) {
        return count;
    }

    // Recursively calculate the number of split stones for the given stone and one blink
    let count = if is_even_digits(stone) {
        let (left, right) = split_stone(stone);
        blink_cached(cache, left, blinks - 1) + blink_cached(cache, right, blinks - 1)
    } else if stone == 0 {
        blink_cached(cache, 1, blinks - 1)
    } else {
        blink_cached(cache, stone * 2024, blinks - 1)
    };

    // Insert the calculated count into the cache
    cache.insert(key, count);

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = read_stones(input);

    for _ in 0..25 {
        stones = stones.into_iter().flat_map(|stone| blink(stone)).collect();
    }

    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = read_stones(input);
    let cache = &mut HashMap::new();

    let count = stones
        .into_iter()
        .map(|stone| blink_cached(cache, stone, 75))
        .sum::<u64>();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
