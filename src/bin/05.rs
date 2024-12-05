use is_sorted::IsSorted;
use std::cmp::Ordering::*;

advent_of_code::solution!(5);

fn read_order_and_pages(input: &str) -> ([[std::cmp::Ordering; 100]; 100], Vec<Vec<usize>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let mut order = [[Equal; 100 as usize]; 100 as usize];

    // Parse rule "12|45" into format 12 and 45
    for rule in rules.lines() {
        let (from, to) = rule.split_once("|").unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();

        order[from][to] = Less;
        order[to][from] = Greater;
    }

    // Parse pages into a vector of usize
    // Example: "75,47,61,53,29" -> [75, 47, 61, 53, 29]
    let pages = pages
        .lines()
        .map(|line| {
            line.split(",")
                .map(|page| page.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    (order, pages)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (order, pages) = read_order_and_pages(input);

    let sum = pages
        .iter()
        .filter(|page| {
            IsSorted::is_sorted_by(&mut page.iter(), |&from, &to| Some(order[*from][*to]))
        })
        .map(|page| page[page.len() / 2])
        .sum::<usize>();

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order, pages) = read_order_and_pages(input);

    let sum = pages
        .iter()
        .filter(|page| {
            !IsSorted::is_sorted_by(&mut page.iter(), |&from, &to| Some(order[*from][*to]))
        })
        .map(|incorrect_page| {
            // Sort the page by the order
            let mut page = incorrect_page.clone();
            page.sort_by(|&from, &to| order[from][to]);
            page[page.len() / 2]
        })
        .sum::<usize>();

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
