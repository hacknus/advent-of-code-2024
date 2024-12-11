use crate::problem::Problem;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct DayEleven {}

pub fn split_number_in_half(number: usize) -> (usize, usize) {
    let digits = (number as f64).log10() as usize + 1;
    let mut left = number;
    for _ in 0..digits / 2 {
        left /= 10;
    }
    (left, number % left)
}

pub fn branch(
    blink: usize,
    max_blink: usize,
    number: usize,
    states: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blink >= max_blink {
        return 1;
    }
    if let Some(state) = states.get(&(blink, number)) {
        return *state;
    }
    let mut sum = 0;
    if number == 0 {
        sum += branch(blink + 1, max_blink, 1, states);
    } else if number.to_string().len() % 2 == 0 {
        let (left, right) = split_number_in_half(number);
        sum += branch(blink + 1, max_blink, left, states);
        sum += branch(blink + 1, max_blink, right, states);
    } else {
        sum += branch(blink + 1, max_blink, number * 2024, states);
    }
    states.insert((blink, number), sum);
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
        let mut states = HashMap::new();
        for stone in stones {
            sum += branch(0, 25, stone, &mut states);
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
        let mut states = HashMap::new();

        for stone in stones {
            sum += branch(0, 75, stone, &mut states);
        }
        format!("{}", sum)
    }
}
