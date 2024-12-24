use crate::problem::Problem;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct DayTwentyFour {}

impl Problem for DayTwentyFour {
    fn part_one(&self, input: &Path) -> String {
        let content = fs::read_to_string(input).unwrap();
        let splits = content
            .split("\n\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut registers = HashMap::new();
        for sp in splits.first().unwrap().split("\n") {
            let sp = sp.split(": ").collect::<Vec<&str>>();
            registers.insert(sp[0].to_string(), sp[1].parse::<usize>().unwrap());
        }

        let mut output = HashMap::new();

        let instructions = splits.get(1).unwrap();
        let mut instructions = instructions
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut instructions_index = 0;
        loop {
            if instructions.is_empty() {
                break;
            }
            if instructions_index >= instructions.len() {
                instructions_index = 0;
            }
            let sp = instructions[instructions_index]
                .split(" -> ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let out_reg = sp.last().unwrap();
            let sp = sp.first().unwrap().split(" ").collect::<Vec<&str>>();
            let reg_a = sp[0];
            let reg_a_val = if let Some(r) = registers.get(reg_a) {
                r
            } else {
                instructions_index += 1;
                continue;
            };
            let op = sp[1];
            let reg_b = sp[2];
            let reg_b_val = if let Some(r) = registers.get(reg_b) {
                r
            } else {
                instructions_index += 1;
                continue;
            };
            let out_val = match op {
                "AND" => reg_a_val & reg_b_val,
                "XOR" => reg_a_val ^ reg_b_val,
                "OR" => reg_a_val | reg_b_val,
                _ => {
                    panic!("Unknown op: {}", op)
                }
            };

            if out_reg.starts_with("z") {
                output.insert(out_reg.to_string(), out_val);
            } else {
                registers.insert(out_reg.to_string(), out_val);
            }
            instructions.remove(instructions_index);
            instructions_index += 1;
        }

        let number = output
            .iter()
            .sorted()
            .rev()
            .map(|(k, v)| v)
            .fold(0, |acc, &b| acc * 2 + b);

        format!("{}", number)
    }

    fn part_two(&self, input: &Path) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}
