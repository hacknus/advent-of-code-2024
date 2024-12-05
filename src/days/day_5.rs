use crate::problem::Problem;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Page {
    pub number: usize,
    pub smaller_numbers: Vec<usize>,
}

impl Eq for Page {}

impl PartialEq<Self> for Page {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl PartialOrd<Self> for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.number == other.number {
            Ordering::Equal
        } else if self.smaller_numbers.iter().contains(&other.number) {
            Ordering::Less
        } else if other.smaller_numbers.iter().contains(&self.number) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

pub struct DayFive {}

impl Problem for DayFive {
    fn part_one(&self, input: &Path) -> String {
        let full_content = fs::read_to_string(input).unwrap();
        let full_content_iter = full_content.split("\n\n").collect::<Vec<&str>>();
        let page_ordering_rules = full_content_iter[0].split("\n").collect::<Vec<&str>>();
        let updates = full_content_iter[1].split("\n").collect::<Vec<&str>>();
        let mut sum = 0;
        'update_loop: for update in updates {
            let numbers = update
                .split(",")
                .map(|ni| ni.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let n = numbers.len();
            let middle_page = numbers[n / 2];

            for i in 0..n {
                let number = numbers[i];
                let numbers_before = numbers[0..i].to_vec();
                let numbers_after = numbers[i + 1..n].to_vec();

                for rule in page_ordering_rules.iter() {
                    let rule_numbers = rule
                        .split("|")
                        .map(|ni| ni.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    let n_a = rule_numbers[0];
                    let n_b = rule_numbers[1];
                    if number == n_a {
                        if numbers_after.contains(&n_b) {
                            // order ok
                        }
                        if numbers_before.contains(&n_b) {
                            // order not ok
                            continue 'update_loop;
                        }
                    }
                }
            }
            sum += middle_page;
        }
        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let full_content = fs::read_to_string(input).unwrap();
        let full_content_iter = full_content.split("\n\n").collect::<Vec<&str>>();
        let page_ordering_rules = full_content_iter[0].split("\n").collect::<Vec<&str>>();
        let updates = full_content_iter[1].split("\n").collect::<Vec<&str>>();
        let mut sum = 0;

        let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();

        for rule in page_ordering_rules.iter() {
            let rule_numbers = rule
                .split("|")
                .map(|ni| ni.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            if let Some(mut rule) = rules.get_mut(&rule_numbers[0]) {
                rule.push(rule_numbers[1]);
            } else {
                rules.insert(rule_numbers[0], vec![rule_numbers[1]]);
            }
        }

        'update_loop: for update in updates {
            let mut numbers = update
                .split(",")
                .map(|ni| {
                    let number = ni.parse::<usize>().unwrap();
                    let smaller_numbers = if let Some(rule) = rules.get(&number) {
                        rule.clone()
                    } else {
                        vec![]
                    };
                    Page {
                        number,
                        smaller_numbers,
                    }
                })
                .collect::<Vec<Page>>();
            let n = numbers.len();
            let old_numbers = numbers.clone();
            numbers.sort();
            if old_numbers != numbers {
                let middle_page = numbers[n / 2].clone();
                sum += middle_page.number;
            }
        }
        format!("{}", sum)
    }
}
