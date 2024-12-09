advent_of_code::solution!(7);

struct Puzzle {
    total: u64,
    values: Vec<u64>,
}

fn parse_to(input: &str) -> Vec<Puzzle> {
    // Read lines and parse. E.g.: 21037: 9 7 18 13
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let sum = parts.next().unwrap().parse().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();

            Puzzle { total: sum, values }
        })
        .collect()
}

// Concatenate two numbers. E.g.: 12, 34 -> 1234
fn concatenate(a: u64, b: u64) -> u64 {
    let mut rest = b;
    let mut a = a;
    loop {
        a *= 10;
        rest /= 10;

        if rest == 0 {
            break;
        }
    }
    a + b
}

impl Puzzle {
    fn solve1(&self) -> bool {
        // Given a list of numbers, enumerate all possible combinations of the numbers
        // and the operations. Then, check if any of the combinations equals the sum.
        if self.values.len() == 1 {
            return self.values[0] == self.total;
        }

        let mut stack = vec![(0, self.values.first().unwrap().clone())];
        while let Some((i, accumulation)) = stack.pop() {
            if i == self.values.len() - 1 {
                if accumulation == self.total {
                    return true;
                }
                continue;
            }

            let next = self.values[i + 1];
            stack.push((i + 1, accumulation + next));
            stack.push((i + 1, accumulation * next));
        }

        false
    }

    fn solve2(&self) -> bool {
        // Given a list of numbers, enumerate all possible combinations of the numbers
        // and the operations. Then, check if any of the combinations equals the sum.
        if self.values.len() == 1 {
            return self.values[0] == self.total;
        }

        let mut stack = vec![(0, self.values.first().unwrap().clone())];
        while let Some((i, accumulation)) = stack.pop() {
            if i == self.values.len() - 1 {
                if accumulation == self.total {
                    return true;
                }
                continue;
            }

            let next = self.values[i + 1];
            stack.push((i + 1, accumulation + next));
            stack.push((i + 1, accumulation * next));
            stack.push((i + 1, concatenate(accumulation, next)));
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzles = parse_to(input);
    let result = puzzles
        .iter()
        .filter(|p| p.solve1())
        .map(|p| p.total)
        .sum::<u64>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzles = parse_to(input);
    let result = puzzles
        .iter()
        .filter(|p| p.solve2())
        .map(|p| p.total)
        .sum::<u64>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(12, 34), 1234);
        assert_eq!(concatenate(56, 78), 5678);
        assert_eq!(concatenate(0, 1), 1);
        assert_eq!(concatenate(1, 0), 10);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
