use crate::io::read_file_lines;
use crate::problem::Problem;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn atoi<F: FromStr>(input: &str) -> Result<F, <F as FromStr>::Err> {
    let i = input.find(|c: char| !c.is_numeric()).unwrap_or(input.len());
    input[..i].parse::<F>()
}
pub struct DayThree {}

impl Problem for DayThree {
    fn part_one(&self, input: &Path) -> String {
        let content = fs::read_to_string(input).expect("Should have been able to read the file");
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let sum = re
            .captures_iter(&content)
            .map(|c| c.extract())
            .collect::<Vec<(&str, [&str; 2])>>()
            .iter()
            .map(|(_, a)| a[0].parse::<i32>().unwrap() * a[1].parse::<i32>().unwrap())
            .sum::<i32>();
        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let content = fs::read_to_string(input).expect("Should have been able to read the file");

        let mut sum = 0;
        for part in content.split("do()") {
            let split_donts = part.split("don't()").collect::<Vec<&str>>();

            let dos = split_donts.first().unwrap();

            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
            sum += re
                .captures_iter(&dos)
                .map(|c| c.extract())
                .collect::<Vec<(&str, [&str; 2])>>()
                .iter()
                .map(|(_, a)| a[0].parse::<i32>().unwrap() * a[1].parse::<i32>().unwrap())
                .sum::<i32>();
        }

        format!("{}", sum)
    }
}
