use std::ops::ControlFlow;

use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(17);

type Integer = i64;
type Bits = u8;

#[derive(Debug, Clone, Copy)]
enum Opcode {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}

impl Opcode {
    fn from(i: Bits) -> Self {
        match i {
            0 => Self::adv,
            1 => Self::bxl,
            2 => Self::bst,
            3 => Self::jnz,
            4 => Self::bxc,
            5 => Self::out,
            6 => Self::bdv,
            7 => Self::cdv,
            _ => panic!("Invalid instruction"),
        }
    }

    fn to(&self) -> Bits {
        match self {
            Self::adv => 0,
            Self::bxl => 1,
            Self::bst => 2,
            Self::jnz => 3,
            Self::bxc => 4,
            Self::out => 5,
            Self::bdv => 6,
            Self::cdv => 7,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Operand {
    value: Bits,
    register_a: Integer,
    register_b: Integer,
    register_c: Integer,
}

impl Operand {
    fn new(value: Bits, register_a: Integer, register_b: Integer, register_c: Integer) -> Self {
        Self {
            value,
            register_a,
            register_b,
            register_c,
        }
    }

    fn to_literal(&self) -> Bits {
        self.value
    }

    fn to_combo(&self) -> Integer {
        match self.value {
            0..=3 => return self.value as Integer,
            4 => return self.register_a,
            5 => return self.register_b,
            6 => return self.register_c,
            7 => panic!("Reserved operand"),
            _ => panic!("Invalid operand"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    op: Opcode,
    value: Operand,
}

impl Instruction {
    fn new(op: Opcode, value: Operand) -> Self {
        Self { op, value }
    }

    fn parse_from(
        op: Bits,
        value: Bits,
        register_a: Integer,
        register_b: Integer,
        register_c: Integer,
    ) -> Self {
        Self::new(
            Opcode::from(op),
            Operand::new(value, register_a, register_b, register_c),
        )
    }
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: Integer,
    register_b: Integer,
    register_c: Integer,

    input: Vec<u8>,
    offset: usize,
    output: Vec<u8>,
}

impl Computer {
    fn new(register_a: Integer, register_b: Integer, register_c: Integer, input: Vec<u8>) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            input,
            offset: 0,
            output: vec![],
        }
    }

    // Parse following into a computer
    // Register A: 729
    // Register B: 0
    // Register C: 0

    // Program: 0,1,5,4,3,0
    fn parse_from(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let register_a = parts.nth(2).unwrap().parse().unwrap();
        let register_b = parts.nth(2).unwrap().parse().unwrap();
        let register_c = parts.nth(2).unwrap().parse().unwrap();

        let input = parts
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        Self {
            register_a,
            register_b,
            register_c,
            input,
            offset: 0,
            output: vec![],
        }
    }

    fn handle(&mut self, instruction: Instruction) {
        match instruction.op {
            Opcode::adv => {
                let combo = instruction.value.to_combo() as u64;
                let value = self.register_a / (2 as i64).pow(combo as u32) as Integer;

                self.register_a = value;
            }
            Opcode::bxl => {
                let value = self.register_b ^ instruction.value.to_literal() as Integer;

                self.register_b = value
            }
            Opcode::bst => {
                let combo = instruction.value.to_combo() as u64;
                let value = (combo % 8) as Integer;

                self.register_b = value;
            }
            Opcode::jnz => {
                if self.register_a != 0 {
                    self.offset = instruction.value.to_literal() as usize;
                    return;
                }
            }
            Opcode::bxc => {
                let value = self.register_b ^ self.register_c as Integer;

                self.register_b = value
            }
            Opcode::out => {
                self.output.push((instruction.value.to_combo() % 8) as u8);
            }
            Opcode::bdv => {
                let combo = instruction.value.to_combo() as u64;
                let value = self.register_a / (2 as i64).pow(combo as u32) as Integer;

                self.register_b = value;
            }
            Opcode::cdv => {
                let combo = instruction.value.to_combo() as u64;
                let value = self.register_a / (2 as i64).pow(combo as u32) as Integer;

                self.register_c = value;
            }
        }

        self.offset += 2;
    }

    fn execute(&mut self) {
        let input = self.input.clone();
        while (self.offset) < input.len() {
            // println!("offset: {:?}, len: {:?}", self.offset, input.len());

            let op = input[self.offset];
            let value = input[self.offset + 1];

            let instruction = Instruction::parse_from(
                op,
                value,
                self.register_a,
                self.register_b,
                self.register_c,
            );

            println!("instruction: {:?}", instruction);
            self.handle(instruction);
            println!("Computer: {:?}", self);
        }

        // println!("Output: {:?}", self.output);
    }
}

// Convert a vector of u8 to a string, joined by commas
// [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0] -> "4,2,5,6,7,7,7,7,3,1,0"
fn to_string(input: &[u8]) -> String {
    input.iter().join(",")
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::parse_from(input);
    computer.execute();

    let output = to_string(&computer.output);
    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let computer = Computer::parse_from(input);

    // Start with known final value of `a` as 0.
    helper(&computer.input, computer.input.len() - 1, 0).break_value()
}

fn helper(program: &[u8], index: usize, a: u64) -> ControlFlow<u64> {
    // Try all 8 combination of lower 3 bits.
    for i in 0..8 {
        let next_a = (a << 3) | i;
        let mut computer = Computer::new(next_a as i64, 0, 0, program.to_vec());
        computer.execute();
        if computer.output == program {
            return ControlFlow::Break(next_a);
        }

        if let Some(out) = computer.output.first() {
            if *out == program[index] {
                helper(&program, index - 1, next_a)?;
            }
        }
    }

    ControlFlow::Continue(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }

    #[test]
    fn test1() {
        let mut computer =
            Computer::parse_from("Register A: 0 Register B: 0 Register C: 9 Program: 2,6");
        let instruction = Instruction::parse_from(2, 6, 0, 0, 9);
        computer.handle(instruction);
        assert!(computer.register_b == 1);
    }

    #[test]
    fn test2() {
        let mut computer =
            Computer::parse_from("Register A: 10 Register B: 0 Register C: 0 Program: 5,0,5,1,5,4");
        computer.execute();
        assert!(computer.output == [0, 1, 2]);
    }

    #[test]
    fn test3() {
        let mut computer = Computer::parse_from(
            "Register A: 2024 Register B: 0 Register C: 0 Program: 0,1,5,4,3,0",
        );
        computer.execute();
        assert!(computer.register_a == 0);
        assert!(computer.output == [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test4() {
        let mut computer =
            Computer::parse_from("Register A: 0 Register B: 29 Register C: 0 Program: 1,7");
        computer.execute();
        assert!(computer.register_b == 26);
    }

    #[test]
    fn test5() {
        let mut computer =
            Computer::parse_from("Register A: 0 Register B: 2024 Register C: 43690 Program: 4,0");
        computer.execute();
        assert!(computer.register_b == 44354);
    }
}
