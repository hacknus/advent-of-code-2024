use crate::io::read_file_lines;
use crate::problem::Problem;
use std::path::Path;

pub struct DayThirteen {}

pub enum ParsingState {
    A,
    B,
    Prize,
}

fn extract_coordiantes(line: &str, parser_state: &ParsingState) -> (f64, f64) {
    let pos = line.split(",").collect::<Vec<&str>>();
    match parser_state {
        ParsingState::A | ParsingState::B => (
            pos[0].split("+").nth(1).unwrap().parse::<f64>().unwrap(),
            pos[1].split("+").nth(1).unwrap().parse::<f64>().unwrap(),
        ),
        ParsingState::Prize => (
            pos[0].split("=").nth(1).unwrap().parse::<f64>().unwrap(),
            pos[1].split("=").nth(1).unwrap().parse::<f64>().unwrap(),
        ),
    }
}
impl Problem for DayThirteen {
    fn part_one(&self, input: &Path) -> String {
        let mut content = read_file_lines(input);
        // add empty line for calculation step below
        content.push("\n".to_string());

        let mut sum = 0.0;

        let mut x_a = 0.0;
        let mut y_a = 0.0;
        let mut x_b = 0.0;
        let mut y_b = 0.0;
        let mut prize_x = 0.0;
        let mut prize_y = 0.0;

        for line in content.iter() {
            if line.contains("Button A:") {
                (x_a, y_a) = extract_coordiantes(line, &ParsingState::A);
            } else if line.contains("Button B:") {
                (x_b, y_b) = extract_coordiantes(line, &ParsingState::B);
            } else if line.contains("Prize:") {
                (prize_x, prize_y) = extract_coordiantes(line, &ParsingState::Prize);
            } else {
                // empty line, let's calculate here

                let n_a = (prize_y / y_b - prize_x / x_b) / (y_a / y_b - x_a / x_b);
                let n_b = (prize_y - y_a * n_a) / y_b;

                if (n_a * 100.0).round() / 100.0 == n_a.round()
                    && (n_b * 100.0).round() / 100.0 == n_b.round()
                {
                    sum += n_a * 3.0 + n_b;
                }
            }
        }

        format!("{}", sum as usize)
    }

    fn part_two(&self, input: &Path) -> String {
        let mut content = read_file_lines(input);
        // add empty line for calculation step below
        content.push("\n".to_string());

        let mut sum = 0.0;

        let mut x_a = 0.0;
        let mut y_a = 0.0;
        let mut x_b = 0.0;
        let mut y_b = 0.0;
        let mut prize_x = 0.0;
        let mut prize_y = 0.0;

        for line in content.iter() {
            if line.contains("Button A:") {
                (x_a, y_a) = extract_coordiantes(line, &ParsingState::A);
            } else if line.contains("Button B:") {
                (x_b, y_b) = extract_coordiantes(line, &ParsingState::B);
            } else if line.contains("Prize:") {
                (prize_x, prize_y) = extract_coordiantes(line, &ParsingState::Prize);
                // add offset for ex 2
                prize_x += 10000000000000.0;
                prize_y += 10000000000000.0;
            } else {
                // empty line, let's calculate here

                let n_a = (prize_y / y_b - prize_x / x_b) / (y_a / y_b - x_a / x_b);
                let n_b = (prize_y - y_a * n_a) / y_b;

                if (n_a * 100.0).round() / 100.0 == n_a.round()
                    && (n_b * 100.0).round() / 100.0 == n_b.round()
                {
                    sum += n_a * 3.0 + n_b;
                }
            }
        }
        format!("{}", sum as usize)
    }
}
