use crate::io::read_file_lines;
use crate::problem::Problem;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_val(&self) -> [isize; 2] {
        match self {
            Direction::Up => [0, -1],
            Direction::Down => [0, 1],
            Direction::Left => [-1, 0],
            Direction::Right => [1, 0],
        }
    }

    pub fn turn_right(&mut self) {
        match self {
            Direction::Up => {
                *self = Direction::Right;
            }
            Direction::Down => {
                *self = Direction::Left;
            }
            Direction::Left => {
                *self = Direction::Up;
            }
            Direction::Right => {
                *self = Direction::Down;
            }
        }
    }
}

pub struct DaySix {}

impl Problem for DaySix {
    fn part_one(&self, input: &Path) -> String {
        let content: Vec<Vec<String>> = read_file_lines(input)
            .iter()
            .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
            .collect();
        let width = content[0].len() as isize;
        let height = content.len() as isize;
        let mut guard = [0, 0];
        let mut dir = Direction::Up;
        for x in 0..width {
            for y in 0..height {
                if content[y as usize][x as usize] == "^" {
                    guard[0] = x;
                    guard[1] = y;
                }
            }
        }
        let mut visited = HashSet::new();
        visited.insert(guard);
        loop {
            let temp_x = guard[0] + dir.to_val()[0];
            let temp_y = guard[1] + dir.to_val()[1];

            if temp_x < 0 || temp_y < 0 || temp_y >= height || temp_x >= width {
                break;
            }

            if content[temp_y as usize][temp_x as usize] == "#" {
                dir.turn_right();
            } else {
                guard[0] = temp_x;
                guard[1] = temp_y;
                visited.insert(guard);
            }
        }
        format!("{}", visited.len())
    }

    fn part_two(&self, input: &Path) -> String {
        let content: Vec<Vec<String>> = read_file_lines(input)
            .iter()
            .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
            .collect();
        let width = content[0].len() as isize;
        let height = content.len() as isize;
        let mut guard = [0, 0];
        let mut dir = Direction::Up;
        for x in 0..width {
            for y in 0..height {
                if content[y as usize][x as usize] == "^" {
                    guard[0] = x;
                    guard[1] = y;
                }
            }
        }
        let init_guard = guard;
        let mut sum = 0;
        let mut content_option = content.clone();

        for x in 0..width {
            'options_loop: for y in 0..height {
                guard = init_guard;
                dir = Direction::Up;
                if content[y as usize][x as usize] != "^" && content[y as usize][x as usize] != "#"
                {
                    content_option = content.clone();
                    content_option[y as usize][x as usize] = "#".to_string();
                } else {
                    continue 'options_loop;
                }

                let mut history = Vec::new();
                loop {
                    let temp_x = guard[0] + dir.to_val()[0];
                    let temp_y = guard[1] + dir.to_val()[1];

                    if temp_x < 0 || temp_y < 0 || temp_y >= height || temp_x >= width {
                        break;
                    }

                    if content_option[temp_y as usize][temp_x as usize] == "#" {
                        dir.turn_right();
                        history.push(guard);
                    } else {
                        guard[0] = temp_x;
                        guard[1] = temp_y;
                        if history.iter().filter(|&n| *n == guard).count() >= 2 {
                            // detected loop
                            sum += 1;
                            continue 'options_loop;
                        }
                    }
                }
            }
        }
        format!("{}", sum)
    }
}
