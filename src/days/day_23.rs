use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;
use std::collections::HashSet;
use std::path::Path;

pub struct DayTwentyThree {}

impl Problem for DayTwentyThree {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);

        let mut nodes = HashSet::new();
        for line in content.iter() {
            for node in line.split("-") {
                nodes.insert(node.to_string());
            }
        }

        let mut loops = HashSet::new();

        for pair_1 in content.iter() {
            let from_1 = pair_1.split("-").nth(0).unwrap();
            let to_1 = pair_1.split("-").nth(1).unwrap();

            for pair_2 in content.iter() {
                if !pair_2.contains(to_1) && !pair_2.contains(from_1) {
                    continue;
                }
                if pair_1 == pair_2 {
                    continue;
                }
                let from_2 = pair_2.split("-").nth(0).unwrap();
                let to_2 = pair_2.split("-").nth(1).unwrap();
                for pair_3 in content.iter() {
                    if !pair_3.contains(to_2) && !pair_3.contains(from_2) {
                        continue;
                    }
                    if pair_3 == pair_2 {
                        continue;
                    }
                    if pair_1 == pair_3 {
                        continue;
                    }
                    let from_3 = pair_3.split("-").nth(0).unwrap();
                    let to_3 = pair_3.split("-").nth(1).unwrap();
                    if !(to_3 == from_1 || to_3 == to_1 || from_3 == to_1 || from_3 == from_1) {
                        continue;
                    }
                    let mut set = HashSet::new();
                    set.insert(from_1);
                    set.insert(from_2);
                    set.insert(from_3);
                    set.insert(to_1);
                    set.insert(to_2);
                    set.insert(to_3);

                    let mut set_vec = set.into_iter().collect_vec();
                    set_vec.sort();

                    loops.insert(set_vec);
                }
            }
        }

        let mut sum = 0;
        for l in loops.iter() {
            if l.len() == 3 && l.iter().filter(|n| n.starts_with("t")).count() > 0 {
                sum += 1;
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}
