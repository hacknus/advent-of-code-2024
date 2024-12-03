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
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let matches: Vec<&str> = re
            .find_iter(&content)
            // try to parse the string matches as i64 (inferred from fn type signature)
            // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
            .filter_map(|digits| Option::from(digits.as_str()))
            // collect the results in to a Vec<i64> (inferred from fn type signature)
            .collect();
        let mut sum = 0;
        for m in matches {
            let split = m.split(",");
            let numbers = split
                .map(|ns| {
                    let number = ns.chars().filter(|c| c.is_numeric()).collect::<String>();
                    number.parse::<i32>().unwrap()
                })
                .collect::<Vec<i32>>();
            sum += numbers[0] * numbers[1];
        }
        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let content = fs::read_to_string(input).expect("Should have been able to read the file");

        let mut sum = 0;
        for part in content.split("do()") {
            let split_donts = part.split("don't()").collect::<Vec<&str>>();

            let dos = split_donts.first().unwrap();

            let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
            let matches: Vec<&str> = re
                .find_iter(dos)
                // try to parse the string matches as i64 (inferred from fn type signature)
                // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
                .filter_map(|digits| Option::from(digits.as_str()))
                // collect the results in to a Vec<i64> (inferred from fn type signature)
                .collect();
            for m in matches {
                let split = m.split(",");
                let numbers = split
                    .map(|ns| {
                        let number = ns.chars().filter(|c| c.is_numeric()).collect::<String>();
                        number.parse::<i32>().unwrap()
                    })
                    .collect::<Vec<i32>>();
                dbg!(&numbers);
                sum += numbers[0] * numbers[1];
            }
        }

        format!("{}", sum)
    }
}
