use diagonal::*;

advent_of_code::solution!(4);
const XMAS: &str = "XMAS";

fn count_xmas(input: &str, substring: &str) -> usize {
    input
        .as_bytes()
        .windows(substring.len())
        .filter(|&w| w == substring.as_bytes())
        .count()
}

fn read_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn matrix_to_string(matrix: &Vec<Vec<char>>) -> Vec<String> {
    let directions = vec![straight_x, straight_y, diagonal_pos_pos, diagonal_pos_neg];
    let mut results = Vec::new();

    for direction in directions {
        let result = direction(&matrix)
            .iter()
            .map(|chars| chars.iter().copied().collect::<String>())
            .collect::<Vec<String>>();

        let result_r = result
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<String>>();

        let mut sum = result.into_iter().chain(result_r).collect::<Vec<_>>();
        results.append(&mut sum);
    }

    results
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = read_to_matrix(input);
    let strings = matrix_to_string(&matrix);
    let count = strings.iter().map(|s| count_xmas(s, XMAS)).sum::<usize>();

    Some(count as u32)
}

fn matrix_to_string2(matrix: &Vec<Vec<char>>) -> Vec<String> {
    let mut results = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if i == 0 || i == matrix.len() - 1 || y == 0 || y == row.len() - 1 {
                continue;
            }

            if *col == 'A' {
                let surrounding = vec![
                    matrix[i - 1][y - 1],
                    matrix[i - 1][y + 1],
                    matrix[i][y],
                    matrix[i + 1][y - 1],
                    matrix[i + 1][y + 1],
                ];

                let result = surrounding.iter().collect::<String>();
                results.push(result);
            }
        }
    }

    results
}

fn equal_to_xmas(input: &str) -> bool {
    let xmas = vec!["MMASS", "MSAMS", "SSAMM", "SMASM"];

    xmas.iter().any(|x| *x == input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = read_to_matrix(input);
    let strings = matrix_to_string2(&matrix);
    let count = strings.iter().filter(|s| equal_to_xmas(s)).count();
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
