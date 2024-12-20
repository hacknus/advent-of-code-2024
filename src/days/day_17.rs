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
pub fn instruction_reverse(
    mut instr_pointer: usize,
    reg_a: &mut i64,
    reg_b: &mut i64,
    reg_c: &mut i64,
    program: &Vec<i64>,
    mut output_index: isize,
    output: &Vec<i64>,
) -> bool {
    dbg!(output_index);
    dbg!(&reg_a);

    dbg!(&reg_b, &reg_c, output_index, instr_pointer);
    if *reg_b == 0 && *reg_c == 0 && output_index == -1 && instr_pointer == program.len() - 2 {
        println!("got it!");
        dbg!(&reg_a);
        return true;
    }

    let op_code = program[instr_pointer];
    let literal_operand = program[instr_pointer + 1];

    match op_code {
        0 => {
            // adv
            let combo = match literal_operand {
                0_i64..=3_i64 => literal_operand,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            // breaks down here, since 3//8 is 0, but 0*8 is not 3
            for i in 0..2_i64.pow(combo as u32) {
                let mut temp_reg_a = *reg_a * 2_i64.pow(combo as u32) + i;
                let new_instr_pointer = if instr_pointer > 1 {
                    instr_pointer - 2
                } else {
                    program.len() - 2
                };
                if instruction_reverse(
                    new_instr_pointer,
                    &mut temp_reg_a,
                    &mut reg_b.clone(),
                    &mut reg_c.clone(),
                    program,
                    output_index,
                    output,
                ) {
                    *reg_a = temp_reg_a;
                    return true;
                }
            }
            if instr_pointer > 1 {
                instr_pointer -= 2;
            } else {
                instr_pointer = program.len() - 2;
            }
        }
        1 => {
            // bxl
            *reg_b ^= literal_operand;
            if instr_pointer > 1 {
                instr_pointer -= 2;
            } else {
                instr_pointer = program.len() - 2;
            }
        }
        2 => {
            // bst
            match literal_operand {
                0_i64..=3_i64 => {
                    if *reg_b != literal_operand % 8 {
                        println!("BST: combo does not match up");
                        return false;
                    }
                }
                4 => *reg_a = *reg_b,
                5 => {
                    if *reg_b != *reg_b % 8 {
                        println!("BST: combo does not match up");
                        return false;
                    }
                }
                6 => *reg_c = *reg_b,
                _ => unreachable!(),
            }
            if instr_pointer > 1 {
                instr_pointer -= 2;
            } else {
                instr_pointer = program.len() - 2;
            }
        }
        3 => {
            // jnz
            // noop
            if instr_pointer > 1 {
                instr_pointer -= 2;
            } else {
                instr_pointer = program.len() - 2;
            }
        }
        4 => {
            // bxc
            *reg_b ^= *reg_c;
            if instr_pointer > 1 {
                instr_pointer -= 2;
            } else {
                instr_pointer = program.len() - 2;
            }
        }
        5 => {
            // out
            let combo = match literal_operand {
                0_i64..=3_i64 => literal_operand,
                4 => *reg_a,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            if combo % 8 != output[output_index as usize] {
                println!("OUT: output does not match up");
                return false;
            }
            println!("OUT: correct:");
            dbg!(combo);
            dbg!(output[output_index as usize]);
            output_index -= 1;
            if instr_pointer > 1 {
                instr_pointer -= 2;
            } else {
                instr_pointer = program.len() - 2;
            }
        }
        6 => {
            // bdv
            let combo = match literal_operand {
                0_i64..=3_i64 => literal_operand,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            for i in 0..2_i64.pow(combo as u32) {
                let mut temp_reg_b = *reg_a * 2_i64.pow(combo as u32) + i;
                let mut temp_reg_a = *reg_a;
                let new_instr_pointer = if instr_pointer > 1 {
                    instr_pointer - 2
                } else {
                    program.len() - 2
                };
                if instruction_reverse(
                    new_instr_pointer,
                    &mut temp_reg_a,
                    &mut temp_reg_b,
                    &mut reg_c.clone(),
                    program,
                    output_index,
                    output,
                ) {
                    *reg_a = temp_reg_a;
                    return true;
                }
            }
            if instr_pointer > 1 {
                instr_pointer -= 2;
            } else {
                instr_pointer = program.len() - 2;
            }
        }
        7 => {
            // cdv
            let combo = match literal_operand {
                0_i64..=3_i64 => literal_operand,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            for i in 0..2_i64.pow(combo as u32) {
                let mut temp_reg_c = *reg_a * 2_i64.pow(combo as u32) + i;
                let mut temp_reg_a = *reg_a;
                let new_instr_pointer = if instr_pointer > 1 {
                    instr_pointer - 2
                } else {
                    program.len() - 2
                };
                if instruction_reverse(
                    new_instr_pointer,
                    &mut temp_reg_a,
                    &mut reg_b.clone(),
                    &mut temp_reg_c,
                    program,
                    output_index,
                    output,
                ) {
                    *reg_a = temp_reg_a;
                    return true;
                }
            }
            if instr_pointer > 1 {
                instr_pointer -= 2;
            } else {
                instr_pointer = program.len() - 2;
            }
        }
        _ => unreachable!(),
    }
    instruction_reverse(
        instr_pointer,
        reg_a,
        reg_b,
        reg_c,
        program,
        output_index,
        output,
    )
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
        let mut reg_b = 0;
        let mut reg_c = 0;
        if instruction_reverse(
            program.len() - 2,
            &mut reg_a,
            &mut reg_b,
            &mut reg_c,
            &program,
            program.len() as isize - 1,
            &program,
        ) {
            println!("success! A: {}", reg_a);
        } else {
            panic!();
        }

        let mut reg_b = 0;
        let mut reg_c = 0;
        let mut output = vec![];
        instruction(0, &mut reg_a, &mut reg_b, &mut reg_c, &program, &mut output);

        output
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .to_string()
    }
}
