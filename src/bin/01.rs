advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let pairs = lines
        .filter_map(|line| {
            // Split the line into two parts
            let mut parts = line.split("   ");
            // Get the first part
            let left = parts.next().and_then(|part| part.parse::<i32>().ok())?;
            // Get the second part
            let right = parts.next().and_then(|part| part.parse::<i32>().ok())?;
            Some((left, right))
        })
        .collect::<Vec<(i32, i32)>>();

    let mut left_vec = pairs.iter().map(|(left, _)| *left).collect::<Vec<i32>>();
    let mut right_vec = pairs.iter().map(|(_, right)| *right).collect::<Vec<i32>>();

    left_vec.sort();
    right_vec.sort();

    let total_distance = left_vec
        .iter()
        .zip(right_vec.iter())
        .map(|(left, right)| (right - left).abs() as u32)
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let pairs = lines
        .filter_map(|line| {
            // Split the line into two parts
            let mut parts = line.split("   ");
            // Get the first part
            let left = parts.next().and_then(|part| part.parse::<i32>().ok())?;
            // Get the second part
            let right = parts.next().and_then(|part| part.parse::<i32>().ok())?;
            Some((left, right))
        })
        .collect::<Vec<(i32, i32)>>();

    let mut left_vec = pairs.iter().map(|(left, _)| *left).collect::<Vec<i32>>();
    let mut right_vec = pairs.iter().map(|(_, right)| *right).collect::<Vec<i32>>();

    left_vec.sort();
    right_vec.sort();

    let total_similarity = left_vec
        .iter()
        .map(|left| *left as u32 * right_vec.iter().filter(|&n| n == left).count() as u32)
        .sum();

    Some(total_similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
