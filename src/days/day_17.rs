use crate::io::read_file_lines;
use crate::problem::Problem;
use std::path::Path;

pub struct DaySeventeen {}

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
            let combo = match literal_operand {
                0_i32..=3_i32 => literal_operand,
                4 => *reg_a,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            *reg_a /= 2_i32.pow(combo as u32);
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
                4 => *reg_a,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            *reg_b = combo % 8;
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
            let combo = match literal_operand {
                0_i32..=3_i32 => literal_operand,
                4 => *reg_a,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            output.push(combo % 8);
            instr_pointer += 2;
        }
        6 => {
            // bdv
            let combo = match literal_operand {
                0_i32..=3_i32 => literal_operand,
                4 => *reg_a,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            *reg_b = *reg_a / 2_i32.pow(combo as u32);
            instr_pointer += 2;
        }
        7 => {
            // cdv
            let combo = match literal_operand {
                0_i32..=3_i32 => literal_operand,
                4 => *reg_a,
                5 => *reg_b,
                6 => *reg_c,
                _ => unreachable!(),
            };
            *reg_c = *reg_a / 2_i32.pow(combo as u32);
            instr_pointer += 2;
        }
        _ => unreachable!(),
    }
    instruction(instr_pointer, reg_a, reg_b, reg_c, program, output);
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
        format!("{}", "Part two not yet implemented.")
    }
}
