use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let rex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(
        rex.captures_iter(input)
            .map(|m| m.extract())
            .map(|(_, [a, b])| (a.parse::<i32>(), b.parse::<i32>()))
            .filter_map(|(a, b)| match (a, b) {
                (Ok(a), Ok(b)) => Some((a, b)),
                _ => None,
            })
            .map(|(a, b)| (a * b) as u32)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let rex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    Some(
        rex.captures_iter(input)
            .fold((0, 1), |(sum, factor), caps| {
                let first = caps.get(0).map(|m| m.as_str()).unwrap();
                if first == "do()" {
                    (sum, 1)
                } else if first == "don't()" {
                    (sum, 0)
                } else {
                    let a = caps
                        .get(1)
                        .map(|m| m.as_str())
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    let b = caps
                        .get(2)
                        .map(|m| m.as_str())
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    (sum + (factor * a * b), factor)
                }
            })
            .0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
