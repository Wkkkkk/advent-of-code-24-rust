advent_of_code::solution!(13);

#[derive(Debug)]
struct ClawMachine {
    a_x: isize,
    a_y: isize,
    b_x: isize,
    b_y: isize,
    prize_x: isize,
    prize_y: isize,
}

fn parse_input(s: &str) -> Vec<ClawMachine> {
    s.split("\n\n")
        .map(|m| {
            let mut lines = m.lines().map(|line| line.split_once(", ").unwrap());
            let (a_x, a_y) = lines.next().unwrap();
            let a_x = a_x.trim_start_matches("Button A: X+").parse().unwrap();
            let a_y = a_y.trim_start_matches("Y+").parse().unwrap();
            let (b_x, b_y) = lines.next().unwrap();
            let b_x = b_x.trim_start_matches("Button B: X+").parse().unwrap();
            let b_y = b_y.trim_start_matches("Y+").parse().unwrap();
            let (prize_x, prize_y) = lines.next().unwrap();
            let prize_x = prize_x.trim_start_matches("Prize: X=").parse().unwrap();
            let prize_y = prize_y.trim_start_matches("Y=").parse().unwrap();
            ClawMachine {
                a_x,
                a_y,
                b_x,
                b_y,
                prize_x,
                prize_y,
            }
        })
        .collect()
}

fn solve(c: &ClawMachine) -> Option<(isize, isize)> {
    let n_b = (c.a_x * c.prize_y - c.a_y * c.prize_x) / (c.b_y * c.a_x - c.b_x * c.a_y);
    let rem_b = (c.a_x * c.prize_y - c.a_y * c.prize_x) % (c.b_y * c.a_x - c.b_x * c.a_y);
    let n_a = (c.prize_x - n_b * c.b_x) / c.a_x;
    let rem_a = (c.prize_x - n_b * c.b_x) % c.a_x;
    if rem_b != 0 || rem_a != 0 {
        return None;
    };
    Some((n_a, n_b))
}

pub fn part_one(input: &str) -> Option<u32> {
    let claw_machines = parse_input(input);
    let tokens = claw_machines
        .iter()
        .filter_map(|c| solve(c))
        .inspect(|(a, b)| println!("Press a: {}, b: {}", a, b))
        .map(|(a, b)| 3 * a.abs() + b.abs())
        .sum::<isize>();

    Some(tokens as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let claw_machines = parse_input(input);
    let tokens = claw_machines
        .into_iter()
        .map(|c| ClawMachine {
            a_x: c.a_x,
            a_y: c.a_y,
            b_x: c.b_x,
            b_y: c.b_y,
            prize_x: c.prize_x + 10000000000000,
            prize_y: c.prize_y + 10000000000000,
        })
        .filter_map(|c| solve(&c))
        .inspect(|(a, b)| println!("Press a: {}, b: {}", a, b))
        .map(|(a, b)| 3 * a.abs() + b.abs())
        .sum::<isize>();

    Some(tokens as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
