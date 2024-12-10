use itertools::Itertools;

advent_of_code::solution!(9);

// Parse the input into a list of numbers.
// E.g.: 233313 -> [2, 3, 3, 3, 1, 3]
fn parse_to(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

// Map the list of numbers into blocks of n elements.
// E.g.: [1, 2, 3, 4, 5] -> [1..222....33333]
fn to_blocks(digits: &Vec<usize>) -> Vec<usize> {
    // The first element is always a content.
    // Index starts at 1 instead of 0.
    let mut index = 1;
    let mut is_content = true;
    let mut acc = Vec::new();

    for &d in digits {
        if is_content {
            acc.extend(vec![index; d]);
            index += 1;
        } else {
            acc.extend(vec![0; d]);
        }
        is_content = !is_content;
    }

    acc
}

pub fn part_one(input: &str) -> Option<u64> {
    let digits = parse_to(input);
    let mut blocks = to_blocks(&digits);

    loop {
        let first_slot_pos = blocks.iter().find_position(|&&x| x == 0)?.0;
        let last_content_pos = blocks.iter().rev().find_position(|&&x| x != 0)?.0;
        let last_content_pos = blocks.len() - last_content_pos - 1;

        if first_slot_pos >= last_content_pos {
            break;
        }

        // Swap the values.
        blocks.swap(first_slot_pos, last_content_pos);
    }

    let result = blocks
        .iter()
        .enumerate()
        // Filter out the free space
        .filter(|(_, &value)| value != 0)
        // Content index starts at 1, so we need to subtract 1.
        .map(|(i, value)| i * (value - 1))
        .sum::<usize>();

    Some(result as u64)
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Content,
    Space,
}

#[derive(Debug)]
struct Block {
    value: Vec<usize>,
    capacity: usize,
    size: usize,
    type_: Type,
    movable: bool,
}

fn into_blocks(digits: &Vec<usize>) -> Vec<Block> {
    let mut index = 1;
    let mut is_content = true;
    let mut acc = Vec::new();

    for &d in digits {
        if is_content {
            acc.push(Block {
                value: vec![index; d],
                capacity: d,
                size: d,
                type_: Type::Content,
                movable: true,
            });
            index += 1;
        } else {
            acc.push(Block {
                value: vec![0; d],
                capacity: d,
                size: 0,
                type_: Type::Space,
                movable: true,
            });
        }
        is_content = !is_content;
    }

    acc
}

pub fn part_two(input: &str) -> Option<u64> {
    let digits = parse_to(input);
    let mut blocks = into_blocks(&digits);

    loop {
        let last_content_block_index = blocks
            .iter()
            .rev()
            .position(|b| b.type_ == Type::Content && b.movable);

        // If there are no more movable content blocks, we are done.
        if last_content_block_index.is_none() {
            break;
        }

        let last_content_block_index = blocks.len() - 1 - last_content_block_index.unwrap();
        let last_content_block_size = blocks[last_content_block_index].size;
        blocks[last_content_block_index].movable = false;

        let space_block_index = blocks
            .iter()
            .position(|b| b.type_ == Type::Space && b.capacity >= last_content_block_size);

        // If there are no more space blocks, we skip to the next iteration.
        if space_block_index.is_none() {
            continue;
        }
        let space_block_index = space_block_index.unwrap();
        if space_block_index >= last_content_block_index {
            continue;
        }

        // Swap the content block with the space block.
        for i in 0..last_content_block_size {
            let size = blocks[space_block_index].size;
            blocks[space_block_index].value[size + i] = blocks[last_content_block_index].value[i];
            blocks[last_content_block_index].value[i] = 0;
        }
        blocks[last_content_block_index].size = 0;
        blocks[space_block_index].size += last_content_block_size;
        blocks[space_block_index].capacity -= last_content_block_size;
    }

    // Concatenate the values of the blocks.
    let values = blocks
        .into_iter()
        .map(|b| b.value)
        .flatten()
        .collect::<Vec<usize>>();

    let result = values
        .iter()
        .enumerate()
        .filter(|(_, &value)| value != 0)
        .map(|(i, value)| i * (value - 1))
        .sum::<usize>();

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
