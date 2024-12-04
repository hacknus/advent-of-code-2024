use crate::io::read_file_lines;
use crate::problem::Problem;
use std::path::Path;

pub struct DayFour {}

impl Problem for DayFour {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let mut sum = 0;
        let width = content[0].chars().count();
        let content = content.into_iter().collect::<String>();
        let n = content.len();

        for i in 0..n {
            // look around
            let j = 4;

            // only look in front, if we are at least 4 away from the edge
            if i % width < width - j + 1 {
                let horizontal = content.chars().skip(i).take(j).collect::<String>();
                if &horizontal == "XMAS" {
                    // forward
                    sum += 1;
                }
                if &horizontal == "SAMX" {
                    // backward
                    sum += 1;
                }
                let diag = content
                    .chars()
                    .skip(i)
                    .step_by(width + 1)
                    .take(j)
                    .collect::<String>();
                if &diag == "XMAS" {
                    // diag forward
                    sum += 1;
                }

                if &diag == "SAMX" {
                    // diag backward
                    sum += 1;
                }
            }

            // only look behind, if we are at least 4 away from the edge
            if i % width >= j - 1 {
                let diag = content
                    .chars()
                    .skip(i)
                    .step_by(width - 1)
                    .take(j)
                    .collect::<String>();
                if &diag == "XMAS" {
                    // -diag forward
                    sum += 1;
                }
                if &diag == "SAMX" {
                    // -diag backward
                    sum += 1;
                }
            }

            let vertical = content
                .chars()
                .skip(i)
                .step_by(width)
                .take(j)
                .collect::<String>();
            if &vertical == "XMAS" {
                // downwards forward
                sum += 1;
            }
            if &vertical == "SAMX" {
                // downwards backward
                sum += 1;
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let mut sum = 0;
        let width = content[0].chars().count();
        let content = content.into_iter().collect::<String>();
        let n = content.len();

        for i in 0..n {
            // look around
            let j = 3;

            if i % width < width - 1
                && i % width >= 1
                && i > width + 1
                && content.chars().skip(i).take(1).collect::<String>() == "A"
            {
                let diag1 = content
                    .chars()
                    .skip(i - 1 - width)
                    .step_by(width + 1)
                    .take(j)
                    .collect::<String>();
                let diag2 = content
                    .chars()
                    .skip(i + 1 - width)
                    .step_by(width - 1)
                    .take(j)
                    .collect::<String>();
                if (&diag1 == "MAS" || &diag1 == "SAM") && (&diag2 == "SAM" || &diag2 == "MAS") {
                    sum += 1;
                }
            }
        }

        format!("{}", sum)
    }
}
