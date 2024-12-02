use crate::io::read_file_lines;
use crate::problem::Problem;
use num::Signed;
use std::path::Path;

pub struct DayTwo {}

impl Problem for DayTwo {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);

        let mut sum = 0;
        'outer: for line in content.iter() {
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            let parsed_line = split_line
                .iter()
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let diff: Vec<i32> = parsed_line
                .windows(2)
                .map(|vs| {
                    let [x, y] = vs else { unreachable!() };
                    *y as i32 - *x as i32
                })
                .collect();

            if diff.iter().filter(|&n| *n == 0).count() > 0 {
                continue 'outer;
            }

            let direction = diff[0].signum();
            for difference in diff.iter() {
                if difference.abs() > 3 {
                    continue 'outer;
                }
                if difference.signum() != direction {
                    continue 'outer;
                }
            }
            sum += 1;
        }
        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let content = read_file_lines(input);

        let mut sum = 0;
        'outer: for line in content.iter() {
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            let mut parsed_line = split_line
                .iter()
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let mut safe = false;
            'inner: for i in 0..parsed_line.len() {
                let mut stumpf_je_eine_usekicke = parsed_line.clone();
                stumpf_je_eine_usekicke.remove(i);

                let diff: Vec<i32> = stumpf_je_eine_usekicke
                    .windows(2)
                    .map(|vs| {
                        let [x, y] = vs else { unreachable!() };
                        *y as i32 - *x as i32
                    })
                    .collect();

                if diff.iter().filter(|&n| *n == 0).count() > 0 {
                    continue 'inner;
                }

                let direction = diff[0].signum();
                for difference in diff.iter() {
                    if difference.abs() > 3 {
                        continue 'inner;
                    }
                    if difference.signum() != direction {
                        continue 'inner;
                    }
                }
                safe = true;
                sum += 1;
                continue 'outer;
            }
        }
        format!("{}", sum)
    }
}
