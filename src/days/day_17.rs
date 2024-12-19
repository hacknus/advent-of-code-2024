use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cmp::PartialEq;
use std::path::Path;

pub struct DaySeventeen {}

pub fn combo(literal_operand: i64, reg_a: i64, reg_b: i64, reg_c: i64) -> i64 {
    match literal_operand {
        0_i64..=3_i64 => literal_operand,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => unreachable!(),
    }
}

pub fn instruction(
    mut instr_pointer: usize,
    reg_a: &mut i64,
    reg_b: &mut i64,
    reg_c: &mut i64,
    program: &Vec<i64>,
    output: &mut Vec<i64>,
) {
    if instr_pointer >= program.len() {
        return;
    }

    let op_code = program[instr_pointer];
    let literal_operand = program[instr_pointer + 1];

    match op_code {
        0 => {
            // adv
            *reg_a /= 2_i64.pow(combo(literal_operand, *reg_a, *reg_b, *reg_c) as u32);
            instr_pointer += 2;
        }
        1 => {
            // bxl
            *reg_b ^= literal_operand;
            instr_pointer += 2;
        }
        2 => {
            // bst
            *reg_b = combo(literal_operand, *reg_a, *reg_b, *reg_c) % 8;
            instr_pointer += 2;
        }
        3 => {
            // jnz
            if *reg_a != 0 {
                instr_pointer = literal_operand as usize;
            } else {
                instr_pointer += 2;
            }
        }
        4 => {
            // bxc
            *reg_b ^= *reg_c;
            instr_pointer += 2;
        }
        5 => {
            // out
            output.push(combo(literal_operand, *reg_a, *reg_b, *reg_c) % 8);
            instr_pointer += 2;
        }
        6 => {
            // bdv
            *reg_b = *reg_a / 2_i64.pow(combo(literal_operand, *reg_a, *reg_b, *reg_c) as u32);
            instr_pointer += 2;
        }
        7 => {
            // cdv
            *reg_c = *reg_a / 2_i64.pow(combo(literal_operand, *reg_a, *reg_b, *reg_c) as u32);
            instr_pointer += 2;
        }
        _ => unreachable!(),
    }
    instruction(instr_pointer, reg_a, reg_b, reg_c, program, output);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operand {
    Multiply(i64),
    Divide(i64),
    RevModulo(i64),
    Multiply2PowSelf,
}
pub fn instruction_2(
    mut instr_pointer: usize,
    reg_a: &mut i64,
    reg_b: &mut i64,
    reg_c: &mut i64,
    program: &Vec<i64>,
    output: &mut Vec<i64>,
) -> bool {
    if instr_pointer >= program.len() {
        return *output == *program;
    }

    let op_code = program[instr_pointer];
    let literal_operand = program[instr_pointer + 1];

    match op_code {
        0 => {
            // adv
            *reg_a /= 2_i64.pow(combo(literal_operand, *reg_a, *reg_b, *reg_c) as u32);
            instr_pointer += 2;
        }
        1 => {
            // bxl
            *reg_b ^= literal_operand;
            instr_pointer += 2;
        }
        2 => {
            // bst
            *reg_b = combo(literal_operand, *reg_a, *reg_b, *reg_c) % 8;
            instr_pointer += 2;
        }
        3 => {
            // jnz
            if *reg_a != 0 {
                instr_pointer = literal_operand as usize;
            } else {
                instr_pointer += 2;
            }
        }
        4 => {
            // bxc
            *reg_b ^= *reg_c;
            instr_pointer += 2;
        }
        5 => {
            // out
            output.push(combo(literal_operand, *reg_a, *reg_b, *reg_c) % 8);
            let i = output.len();
            if *output != program[..i] {
                return false;
            }
            instr_pointer += 2;
        }
        6 => {
            // bdv
            *reg_b = *reg_a / 2_i64.pow(combo(literal_operand, *reg_a, *reg_b, *reg_c) as u32);
            instr_pointer += 2;
        }
        7 => {
            // cdv
            *reg_c = *reg_a / 2_i64.pow(combo(literal_operand, *reg_a, *reg_b, *reg_c) as u32);
            instr_pointer += 2;
        }
        _ => unreachable!(),
    }
    instruction_2(instr_pointer, reg_a, reg_b, reg_c, program, output)
}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let mut reg_a = content
            .first()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let mut reg_b = content
            .get(1)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let mut reg_c = content
            .get(2)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let program = content
            .get(4)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut output = vec![];
        instruction(0, &mut reg_a, &mut reg_b, &mut reg_c, &program, &mut output);

        output
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .to_string()
    }

    fn part_two(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let mut reg_a = content
            .first()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let mut reg_b = content
            .get(1)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let mut reg_c = content
            .get(2)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let program = content
            .get(4)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut reg_a = 0;
        let mut counter = 0;
        loop {
            let mut output = vec![];
            reg_a = counter;
            if instruction_2(
                0,
                &mut reg_a,
                &mut reg_b.clone(),
                &mut reg_c.clone(),
                &program,
                &mut output,
            ) {
                reg_a = counter;
                break;
            }
            counter += 1;
        }

        reg_a.to_string()
    }
}
