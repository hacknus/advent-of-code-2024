use crate::io::read_file_lines;
use crate::problem::Problem;
use std::path::Path;

pub struct DayFour {}

impl Problem for DayFour {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let mut sum = 0;
        let width = content[0].len();
        let content = content.into_iter().collect::<String>();
        let n = content.len();

        for i in 0..n {
            // look around
            let j = 4;

            // only look in front, if we are at least 4 away from the edge
            if i % width < width - j + 1 {
                if &content.chars().skip(i).take(j).collect::<String>() == "XMAS" {
                    // forward
                    sum += 1;
                }
                if &content.chars().skip(i).take(j).collect::<String>() == "SAMX" {
                    // backward
                    sum += 1;
                }
                if &content
                    .chars()
                    .skip(i)
                    .step_by(width + 1)
                    .take(j)
                    .collect::<String>()
                    == "XMAS"
                {
                    // diag forward
                    sum += 1;
                }

                if &content
                    .chars()
                    .skip(i)
                    .step_by(width + 1)
                    .take(j)
                    .collect::<String>()
                    == "SAMX"
                {
                    // diag backward
                    sum += 1;
                }
            }

            // only look behind, if we are at least 4 away from the edge
            if i % width >= j - 1 {
                if &content
                    .chars()
                    .skip(i)
                    .step_by(width - 1)
                    .take(j)
                    .collect::<String>()
                    == "XMAS"
                {
                    // -diag forward
                    sum += 1;
                }
                if &content
                    .chars()
                    .skip(i)
                    .step_by(width - 1)
                    .take(j)
                    .collect::<String>()
                    == "SAMX"
                {
                    // -diag backward
                    sum += 1;
                }
            }
            if &content
                .chars()
                .skip(i)
                .step_by(width)
                .take(j)
                .collect::<String>()
                == "XMAS"
            {
                // downwards forward
                sum += 1;
            }
            if &content
                .chars()
                .skip(i)
                .step_by(width)
                .take(j)
                .collect::<String>()
                == "SAMX"
            {
                // downwards backward
                sum += 1;
            }
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let mut sum = 0;
        let width = content[0].len();
        let content = content.into_iter().collect::<String>();
        let n = content.len();

        for i in 0..n {
            // look around
            let j = 3;

            // only look in front, if we are at least 4 away from the edge
            if i % width < width - 1
                && i % width >= 1
                && i > width + 1
                && &content.chars().skip(i).take(1).collect::<String>() == "A"
                && (&content
                    .chars()
                    .skip(i - 1 - width)
                    .step_by(width + 1)
                    .take(j)
                    .collect::<String>()
                    == "MAS"
                    || &content
                        .chars()
                        .skip(i - 1 - width)
                        .step_by(width + 1)
                        .take(j)
                        .collect::<String>()
                        == "SAM")
                && (&content
                    .chars()
                    .skip(i + 1 - width)
                    .step_by(width - 1)
                    .take(j)
                    .collect::<String>()
                    == "SAM"
                    || &content
                        .chars()
                        .skip(i + 1 - width)
                        .step_by(width - 1)
                        .take(j)
                        .collect::<String>()
                        == "MAS")
            {
                sum += 1;
            }
        }

        format!("{}", sum)
    }
}
