use rayon::prelude::*;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
}

// parse "p=0,4 v=3,-3" into a Robot struct
fn parse(input: &str, width: usize, height: usize) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let p = parts.next().unwrap();
            let v = parts.last().unwrap();
            let mut p = p[2..].split(',');
            let mut v = v[2..].split(',');
            let x = p.next().unwrap().parse().unwrap();
            let y = p.next().unwrap().parse().unwrap();
            let dx: i32 = v.next().unwrap().parse().unwrap();
            let dy: i32 = v.next().unwrap().parse().unwrap();
            Robot {
                x,
                y,
                dx: dx.rem_euclid(width as i32) as usize,
                dy: dy.rem_euclid(height as i32) as usize,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = 11;
    let height = 7;
    let robots = parse(input, width, height);
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let robots_moved = robots
        .into_iter()
        .map(|robot| {
            let x = (robot.x + 100 * robot.dx) % width;
            let y = (robot.y + 100 * robot.dy) % height;
            Robot {
                x,
                y,
                dx: robot.dx,
                dy: robot.dy,
            }
        })
        .collect::<Vec<Robot>>();

    let mut grid = vec![vec!['.'; width]; height];
    for robot in &robots_moved {
        println!("robot: {:?}", robot);
        let x = robot.x;
        let y = robot.y;
        grid[y][x] = '#';

        if x < width / 2 {
            if y < height / 2 {
                q1 += 1;
            }
            if y > height / 2 {
                q2 += 1;
            }
        }
        if x > width / 2 {
            if y < height / 2 {
                q3 += 1;
            }
            if y > height / 2 {
                q4 += 1;
            }
        }
    }

    // print grid
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }

    println!("q1: {}, q2: {}, q3: {}, q4: {}", q1, q2, q3, q4);
    Some((q1 * q2 * q3 * q4) as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let width = 101;
    let height = 103;
    let robots = parse(input, width, height);
    let minimum_time = (0..(width * height * 1000) as usize)
        .into_par_iter()
        .find_first(|t| {
            let mut grid = vec![vec!['.'; width]; height];

            for robot in &robots {
                let (x, y) = (
                    (robot.x + robot.dx * t) % width,
                    (robot.y + robot.dy * t) % height,
                );

                grid[y][x] = '#';
            }

            // looking to see 2 rows with a continuous line of at least 16 robots
            // this will match the lines above and below the christmas tree
            grid.iter()
                .filter(|&row| {
                    row.windows(16)
                        .any(|window| window.iter().all(|&cell| cell == '#'))
                })
                .take(2)
                .count()
                >= 2
        })
        .ok_or(|| 0);

    minimum_time.ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
