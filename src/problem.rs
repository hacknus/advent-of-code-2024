use std::path::Path;

pub trait Problem {
    fn part_one(&self, input: &Path) -> String;
    fn part_two(&self, input: &Path) -> String;
}
