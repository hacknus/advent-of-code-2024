use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;
use ndarray::Array2;
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
    pub fn to_pos(&self) -> (isize, isize) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

impl Problem for DaySixteen {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);

        let width = content[0].len();
        let height = content.len();

        let mut fuse_grid = vec![vec![None::<u16>; width]; height];
        let mut direction_grid = vec![vec![None::<Direction>; width]; height];
        fuse_grid[height - 2][1] = Some(1);
        direction_grid[height - 2][1] = Some(Direction::Right);

        let mut grid = vec![];
        for line in content.iter() {
            grid.push(line.chars().collect_vec());
        }

        let mut set = HashSet::new();
        let mut history = HashSet::new();
        set.insert((1, height as isize - 2));
        history.insert((1, height as isize - 2));
        let mut t = 0;

        'walk_loop: loop {
            let mut to_delete_set = HashSet::new();
            let mut to_add_set = HashSet::new();
            for position in set.iter() {
                if position.0 == width as isize - 2 && position.1 == 1 {
                    // arrived at target
                    break 'walk_loop;
                }

                if let Some(mut fuse) = fuse_grid[position.1 as usize][position.0 as usize] {
                    if fuse > 1 {
                        fuse_grid[position.1 as usize][position.0 as usize] = Some(fuse - 1);
                        continue;
                    }
                }

                for direction in [
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                ] {
                    let (dx, dy) = direction.to_pos();
                    let xi = position.0 + dx;
                    let yi = position.1 + dy;

                    if xi > width as isize - 1 {
                        continue;
                    }
                    if yi > height as isize - 1 {
                        continue;
                    }

                    if history.contains(&(xi, yi)) {
                        continue;
                    }

                    if grid[yi as usize][xi as usize] == '#' {
                        continue;
                    }

                    if let Some(dir) = direction_grid[position.1 as usize][position.0 as usize] {
                        direction_grid[yi as usize][xi as usize] = Some(direction);
                        if dir == direction {
                            fuse_grid[yi as usize][xi as usize] = Some(1);
                            history.insert((xi, yi));
                            to_add_set.insert((xi, yi));
                        } else {
                            fuse_grid[yi as usize][xi as usize] = Some(1001);
                            history.insert((xi, yi));
                            to_add_set.insert((xi, yi));
                        }
                    }
                }
                to_delete_set.insert(*position);
            }

            for d in to_delete_set.iter() {
                set.remove(d);
            }
            for d in to_add_set {
                set.insert(d);
            }

            t += 1;
        }

        // 105448, too high
        // not 101444
        // not 99432
        // not 89472

        format!("{}", t)
    }

    fn part_two(&self, input: &Path) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}
