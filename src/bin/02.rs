advent_of_code::solution!(2);

// Check if the vector is gradually increasing / decreasing
fn is_monotonic(report: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..report.len() {
        if report[i] > report[i - 1] {
            is_decreasing = false;
        } else if report[i] < report[i - 1] {
            is_increasing = false;
        } else if report[i] == report[i - 1] {
            is_increasing = false;
            is_decreasing = false;
        }
    }

    is_increasing || is_decreasing
}

// Check if the gap between the two numbers is less than 3
fn is_gap_less_than(report: &Vec<i32>, max: u32) -> bool {
    for i in 1..report.len() {
        let gap = report[i] - report[i - 1];
        if gap.abs() as u32 > max {
            return false;
        }
    }

    true
}

fn is_safe(report: &Vec<i32>) -> bool {
    is_monotonic(report) && is_gap_less_than(report, 3)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let result = lines
        .filter_map(|line| {
            let parts = line.split(" ");
            let report = parts
                .filter_map(|part| part.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            Some(report)
        })
        .filter(|report| is_safe(report))
        .count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let result = lines
        .filter_map(|line| {
            let report = line
                .split_whitespace()
                .filter_map(|part| part.parse().ok())
                .collect::<Vec<i32>>();

            Some(report)
        })
        .map(|report| {
            if is_safe(&report) {
                return true;
            }

            for i in 0..report.len() {
                let mut modified = report.clone();
                modified.remove(i);
                if is_safe(&modified) {
                    return true;
                }
            }

            false
        })
        .filter(|safe| *safe)
        .count();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
