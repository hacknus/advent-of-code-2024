use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayOne {}

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let content = read_file_lines(input);
        let mut historian_a = vec![];
        let mut historian_b = vec![];
        for line in content.iter() {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            historian_a.push(split[0].parse::<i32>().unwrap());
            historian_b.push(split[1].parse::<i32>().unwrap());
        }

        historian_a.sort();
        historian_b.sort();

        let mut sum = 0;
        let n = historian_b.len();
        for i in 0..n {
            sum += (historian_a[i] - historian_b[i]).abs();
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let content = read_file_lines(input);
        let mut historian_a = vec![];
        let mut historian_b = vec![];
        for line in content.iter() {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            historian_a.push(split[0].parse::<i32>().unwrap());
            historian_b.push(split[1].parse::<i32>().unwrap());
        }

        let mut sum = 0;
        let n = historian_b.len();
        for i in 0..n {
            sum += (historian_a[i]
                * historian_b.iter().filter(|&n| *n == historian_a[i]).count() as i32)
                .abs();
        }

        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
