use crate::io::read_file_lines;
use crate::problem::Problem;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct DayEleven {}

impl Problem for DayEleven {
    fn part_one(&self, input: &Path) -> String {
        let mut stones = fs::read_to_string(input)
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        println!("{:?}", stones);

        let mut states: HashMap<String, Vec<String>> = HashMap::new();
        for blink in 0..25 {
            dbg!(blink);
            let mut i = 0;
            loop {
                if let Some(state) = states.get(&stones[i]) {
                    stones[i] = state[0].clone();
                    if state.len() == 2 {
                        stones.insert(i + 1, state[1].clone());
                        i += 1;
                    }
                } else if stones[i] == "0" {
                    states.insert(stones[i].clone(), vec!["1".to_string()]);
                    stones[i] = "1".to_string();
                } else if stones[i].len() % 2 == 0 {
                    let left = stones[i]
                        .chars()
                        .take(stones[i].len() / 2)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    let right = stones[i]
                        .chars()
                        .skip(stones[i].len() / 2)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    states.insert(stones[i].clone(), vec![left.to_string(), right.to_string()]);
                    stones[i] = left.to_string();
                    stones.insert(i + 1, right.to_string());
                    i += 1;
                } else {
                    states.insert(
                        stones[i].clone(),
                        vec![format!("{}", stones[i].parse::<usize>().unwrap() * 2024)],
                    );
                    stones[i] = format!("{}", stones[i].parse::<usize>().unwrap() * 2024);
                }

                i += 1;
                if i >= stones.len() {
                    break;
                }
            }
        }

        format!("{}", stones.len())
    }

    fn part_two(&self, input: &Path) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}
