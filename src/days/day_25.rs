use crate::problem::Problem;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct DayTwentyFive {}

impl Problem for DayTwentyFive {
    fn part_one(&self, input: &Path) -> String {
        let content = fs::read_to_string(input).unwrap();
        let mut keys = vec![];
        let mut locks = vec![];
        for entry in content.split("\n\n") {
            let mut arr = [0; 5];
            for line in entry.split("\n").skip(1).take(5) {
                for (j, c) in line.chars().enumerate() {
                    if c == '#' {
                        arr[j] += 1;
                    }
                }
            }
            if entry.starts_with("#####") {
                locks.push(arr);
            } else if entry.starts_with(".....") {
                keys.push(arr);
            } else {
                unreachable!()
            }
        }

        let mut sum = 0;

        for key in keys.iter() {
            for lock in locks.iter() {
                let mut overlap = false;
                for (k, l) in key.iter().zip(lock.iter()) {
                    if 5 - *k < *l {
                        overlap = true;
                    }
                }
                if !overlap {
                    sum += 1;
                }
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}
