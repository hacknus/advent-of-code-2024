use crate::problem::Problem;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::rc::Rc;

pub struct DayEleven {}

pub fn branch(blink: usize, max_blink: usize, number: usize) -> usize {
    if blink >= max_blink {
        return 1;
    }
    let mut sum = 0;
    if number == 0 {
        sum += branch(blink + 1, max_blink, 1);
    } else if number.to_string().len() % 2 == 0 {
        let left = number
            .to_string()
            .chars()
            .take(number.to_string().len() / 2)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let right = number
            .to_string()
            .chars()
            .skip(number.to_string().len() / 2)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        sum += branch(blink + 1, max_blink, left);
        sum += branch(blink + 1, max_blink, right);
    } else {
        sum += branch(blink + 1, max_blink, number * 2024);
    }
    sum
}

impl Problem for DayEleven {
    fn part_one(&self, input: &Path) -> String {
        let mut stones = fs::read_to_string(input)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut sum = 0;
        for stone in stones {
            sum += branch(0, 25, stone);
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let mut stones = fs::read_to_string(input)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut sum = 0;
        for stone in stones {
            sum += branch(0, 75, stone);
        }
        format!("{}", "Part two not yet implemented.")
    }
}
