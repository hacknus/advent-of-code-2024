use crate::problem::Problem;
use std::fs;
use std::path::Path;

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
        'update_loop: for update in updates {
            let mut numbers = update
                .split(",")
                .map(|ni| ni.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let n = numbers.len();

            let mut order_ok = true;
            'check_loop: for i in 0..n {
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
                    if number == n_a && numbers_before.contains(&n_b) {
                        // order not ok
                        order_ok = false;
                        break 'check_loop;
                    }
                }
            }
            if !order_ok {
                // let's re-order
                let mut old_numbers = vec![];
                while old_numbers != numbers {
                    old_numbers = numbers.clone();
                    for i in 1..n {
                        let number1 = numbers[i - 1];
                        let number2 = numbers[i];
                        for rule in page_ordering_rules.iter() {
                            let rule_numbers = rule
                                .split("|")
                                .map(|ni| ni.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>();
                            let n_a = rule_numbers[0];
                            let n_b = rule_numbers[1];
                            if number1 == n_b && number2 == n_a {
                                numbers.swap(i - 1, i);
                            }
                        }
                    }
                }
                let middle_page = numbers[n / 2];
                sum += middle_page;
            }
        }

        // not 6126, too low
        // not 6925, too high
        format!("{}", sum)
    }
}
