use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;
use std::path::Path;

pub struct DayTwentyTwo {}

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(a: usize) -> usize {
    a % 16777216
}

impl Problem for DayTwentyTwo {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input)
            .iter()
            .map(|line| line.parse::<usize>().unwrap())
            .collect_vec();

        let mut sum = 0;
        for mut secret in content {
            for _ in 0..2000 {
                secret = mix(secret, secret * 64);
                secret = prune(secret);
                secret = mix(secret, secret / 32);
                secret = prune(secret);
                secret = mix(secret, secret * 2048);
                secret = prune(secret);
            }
            sum += secret;
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}
