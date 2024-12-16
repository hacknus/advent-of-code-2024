use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::path::Path;

pub struct DaySixteen {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

pub fn walk(
    score: usize,
    x: isize,
    y: isize,
    dir: Direction,
    content: &Vec<String>,
    mut history: HashSet<[isize; 2]>,
) -> usize {
    if score > 99432 {
        return usize::MAX;
    }

    let mut sum = usize::MAX;
    for (dx, dy, new_dir) in [
        (-1, 0, Direction::Left),
        (1, 0, Direction::Right),
        (0, 1, Direction::Down),
        (0, -1, Direction::Up),
    ] {
        // bounds check
        if x == 0 && dx == -1 {
            continue;
        }
        if y == 0 && dy == -1 {
            continue;
        }
        if y == content.len() as isize - 1 && dy == 1 {
            continue;
        }
        if x == content[0].len() as isize - 1 && dx == 1 {
            continue;
        }

        let xi = x + dx;
        let yi = y + dy;

        if history.contains(&[xi, yi]) {
            // already was there with this branch
            continue;
        }

        history.insert([xi, yi]);

        // TODO: probably faster with ndarray
        if content
            .get(yi as usize)
            .unwrap()
            .chars()
            .nth(xi as usize)
            .unwrap()
            == '.'
        {
            if new_dir == dir {
                sum = walk(score + 1, xi, yi, new_dir, content, history.clone()).min(sum);
            } else {
                sum = walk(score + 1001, xi, yi, new_dir, content, history.clone()).min(sum);
            }
        } else if content
            .get(yi as usize)
            .unwrap()
            .chars()
            .nth(xi as usize)
            .unwrap()
            == 'E'
        {
            return if new_dir == dir {
                score + 1
            } else {
                score + 1001
            };
        }
    }
    sum
}

impl Problem for DaySixteen {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let history = HashSet::new();

        let score = walk(
            0,
            1,
            content.len() as isize - 2,
            Direction::Right,
            &content,
            history,
        );

        // 105448, too high
        // not 101444
        // not 99432

        format!("{}", score)
    }

    fn part_two(&self, input: &Path) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}
