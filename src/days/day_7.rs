use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;
use rayon::prelude::*;
use std::path::Path;

pub struct DaySeven {}

fn concat(a: i64, b: i64) -> i64 {
    a * 10i64.pow(b.ilog10() + 1) + b
}
impl Problem for DaySeven {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let mut sum = 0;
        let operators = ["+", "*"];
        'line_loop: for line in content.iter() {
            let splits = line.split(":").collect::<Vec<&str>>();
            let result = splits[0].parse::<i64>().unwrap();
            let parts = splits[1]
                .split_whitespace()
                .map(|p| p.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            if parts.iter().sum::<i64>() == result {
                sum += result;
                continue 'line_loop;
            }
            if parts.iter().product::<i64>() == result {
                sum += result;
                continue 'line_loop;
            }

            let n = parts.len();
            let combinations = (1..n).map(|_| operators).multi_cartesian_product();

            for combination in combinations {
                let mut parts_to_calculate = parts.clone();
                for i in 0..n - 1 {
                    match combination[i] {
                        "*" => parts_to_calculate[i + 1] *= parts_to_calculate[i],
                        "+" => parts_to_calculate[i + 1] += parts_to_calculate[i],
                        _ => {
                            unreachable!()
                        }
                    }
                }
                if parts_to_calculate.last().unwrap() == &result {
                    sum += result;
                    continue 'line_loop;
                }
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let operators = ["+", "*", "|"];
        let sum: i64 = content
            .into_par_iter()
            .map(|line| {
                let splits = line.split(":").collect::<Vec<&str>>();
                let result = splits[0].parse::<i64>().unwrap();
                let parts = splits[1]
                    .split_whitespace()
                    .map(|p| p.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

                let n = parts.len();
                let combinations = (1..n).map(|_| operators).multi_cartesian_product();
                'combination_loop: for combination in combinations {
                    let mut parts_to_calculate = parts.clone();
                    for i in 0..n - 1 {
                        match combination[i] {
                            "*" => parts_to_calculate[i + 1] *= parts_to_calculate[i],
                            "+" => parts_to_calculate[i + 1] += parts_to_calculate[i],
                            "|" => {
                                parts_to_calculate[i + 1] =
                                    concat(parts_to_calculate[i], parts_to_calculate[i + 1])
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                        if parts_to_calculate[i + 1] > result {
                            continue 'combination_loop;
                        }
                    }
                    if parts_to_calculate.last().unwrap() == &result {
                        return result;
                    }
                }
                0
            })
            .sum();
        format!("{}", sum)
    }
}
