use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::zip;
use std::cmp::PartialEq;
use std::path::Path;

pub struct DaySeventeen {}

pub fn combo(literal_operand: i32, reg_a: i32, reg_b: i32, reg_c: i32) -> i32 {
    match literal_operand {
        0_i32..=3_i32 => literal_operand,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => unreachable!(),
    }
}

pub fn instruction(
    mut instr_pointer: usize,
    reg_a: &mut i32,
    reg_b: &mut i32,
    reg_c: &mut i32,
    program: &Vec<i32>,
    output: &mut Vec<i32>,
) {
    if instr_pointer >= program.len() {
        return;
    }

    let op_code = program[instr_pointer];
    let literal_operand = program[instr_pointer + 1];

    match op_code {
        0 => {
            // adv
            *reg_a /= 2_i32.pow(combo(literal_operand, *reg_a, *reg_b, *reg_c) as u32);
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
            *reg_b = *reg_a / 2_i32.pow(combo(literal_operand, *reg_a, *reg_b, *reg_c) as u32);
            instr_pointer += 2;
        }
        7 => {
            // cdv
            *reg_c = *reg_a / 2_i32.pow(combo(literal_operand, *reg_a, *reg_b, *reg_c) as u32);
            instr_pointer += 2;
        }
        _ => unreachable!(),
    }
    instruction(instr_pointer, reg_a, reg_b, reg_c, program, output);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operand {
    Multiply(i32),
    Divide(i32),
    RevModulo(i32),
    Multiply2PowSelf,
}
pub fn instruction_2(
    mut instr_pointer: usize,
    reg_b: &mut i32,
    reg_c: &mut i32,
    program: &Vec<i32>,
    calc_history: &mut Vec<Operand>,
    output: &mut Vec<i32>,
) -> i32 {
    if instr_pointer >= program.len() {
        return 0;
    }

    let op_code = program[instr_pointer];
    let literal_operand = program[instr_pointer + 1];

    dbg!(op_code, literal_operand);

    match op_code {
        0 => {
            // adv
            match literal_operand {
                0_i32..=3_i32 => {
                    calc_history.push(Operand::Multiply(2_i32.pow(literal_operand as u32)))
                }
                4 => calc_history.push(Operand::Multiply2PowSelf),
                5 => calc_history.push(Operand::Multiply(2_i32.pow(*reg_b as u32))),
                6 => calc_history.push(Operand::Multiply(2_i32.pow(*reg_c as u32))),
                _ => unreachable!(),
            }
            instr_pointer += 2;
        }
        1 => {
            // bxl
            *reg_b ^= literal_operand;
            instr_pointer += 2;
        }
        2 => {
            // bst
            let combo = match literal_operand {
                0_i32..=3_i32 => literal_operand,
                4 => 1,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            *reg_b = combo % 8;
            instr_pointer += 2;
        }
        3 => {
            // jnz

            // check if reg_a should be zero here
            let mut output_temp = vec![];
            let mut reg_a_temp = 0;
            let mut reg_b_temp = *reg_b;
            let mut reg_c_temp = *reg_c;
            instruction(
                instr_pointer + 2,
                &mut reg_a_temp,
                &mut reg_b_temp,
                &mut reg_c_temp,
                program,
                &mut output_temp,
            );

            let mut output_test = output.clone();
            output_test.extend(output_temp);

            let mut valid_counter = 0;
            let mut correct_counter = 0;
            for (prgrm, test) in program.iter().zip(output_test.iter()) {
                if *test != -1 {
                    valid_counter += 1;
                    if test == prgrm {
                        correct_counter += 1;
                    }
                }
            }

            if valid_counter == correct_counter && program.len() == output_test.len() {
                return 0;
            }

            instr_pointer = literal_operand as usize;

            // if *reg_a != 0 {
            //     instr_pointer = literal_operand as usize;
            // } else {
            //     instr_pointer += 2;
            // }
        }
        4 => {
            // bxc
            *reg_b ^= *reg_c;
            instr_pointer += 2;
        }
        5 => {
            // out
            if literal_operand == 4 {
                let i = output.len();
                if output.len() == program.len() - 1 {
                    return program[i];
                }
                calc_history.push(Operand::RevModulo(program[i]));
                output.push(program[i]);
            } else {
                let combo = match literal_operand {
                    0_i32..=3_i32 => literal_operand,
                    5 => *reg_b,
                    6 => *reg_c,
                    _ => unreachable!(),
                };
                output.push(combo % 8);
            }
            instr_pointer += 2;
        }
        6 => {
            // bdv
            let combo = match literal_operand {
                0_i32..=3_i32 => literal_operand,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            return *reg_b * 2_i32.pow(combo as u32);
        }
        7 => {
            // cdv
            let combo = match literal_operand {
                0_i32..=3_i32 => literal_operand,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            return *reg_c * 2_i32.pow(combo as u32);
        }
        _ => unreachable!(),
    }
    instruction_2(instr_pointer, reg_b, reg_c, program, calc_history, output)
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
            .parse::<i32>()
            .unwrap();
        let mut reg_b = content
            .get(1)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut reg_c = content
            .get(2)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let program = content
            .get(4)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

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
            .parse::<i32>()
            .unwrap();
        let mut reg_b = content
            .get(1)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut reg_c = content
            .get(2)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let program = content
            .get(4)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut output = vec![];
        let mut calc_history: Vec<Operand> = vec![];
        let mut reg_a = instruction_2(
            0,
            &mut reg_b.clone(),
            &mut reg_c.clone(),
            &program,
            &mut calc_history,
            &mut output,
        );

        for calculation in calc_history.iter().rev() {
            match calculation {
                Operand::Multiply(x) => reg_a *= x,
                Operand::Divide(x) => reg_a /= x,
                Operand::Multiply2PowSelf => reg_a *= 2_i32.pow(reg_a as u32),
                Operand::RevModulo(x) => reg_a += x,
            }
        }
        dbg!(&reg_a);

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
