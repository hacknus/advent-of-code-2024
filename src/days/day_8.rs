use crate::io::read_file_lines;
use crate::problem::Problem;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub struct DayEight {}

impl Problem for DayEight {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let width = content[0].len() as isize;
        let height = content.len() as isize;

        let mut antennas: HashMap<char, Vec<[isize; 2]>> = HashMap::new();

        for (x, line) in content.iter().enumerate() {
            for (y, c) in line.char_indices() {
                if c != '.' {
                    // found antenna
                    if let Some(antenna) = antennas.get_mut(&c) {
                        antenna.push([x as isize, y as isize]);
                    } else {
                        antennas.insert(c, vec![[x as isize, y as isize]]);
                    }
                }
            }
        }

        let mut antinode_positions = HashSet::new();

        for (antenna, positions) in antennas.iter() {
            for [x, y] in positions.iter() {
                for [xj, yj] in positions.iter() {
                    if x == xj && y == yj {
                        continue;
                    }

                    let dx = (x - xj).abs();
                    let dy = (y - yj).abs();

                    let mut antinode_a = [*x, *y];
                    let mut antinode_b = [*xj, *yj];
                    if antinode_a[0] < antinode_b[0] {
                        antinode_a[0] -= dx;
                        antinode_b[0] += dx;
                    } else {
                        antinode_b[0] -= dx;
                        antinode_a[0] += dx;
                    }
                    if antinode_a[1] < antinode_b[1] {
                        antinode_a[1] -= dy;
                        antinode_b[1] += dy;
                    } else {
                        antinode_b[1] -= dy;
                        antinode_a[1] += dy;
                    }
                    let antinodes = [antinode_a, antinode_b];

                    for antinode in antinodes {
                        if antinode[0] >= 0
                            && antinode[1] >= 0
                            && antinode[0] < width
                            && antinode[1] < height
                        {
                            antinode_positions.insert(antinode);
                        }
                    }
                }
            }
        }

        format!("{}", antinode_positions.len())
    }

    fn part_two(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let width = content[0].len() as isize;
        let height = content.len() as isize;

        let mut antenna_positions = Vec::new();
        let mut antenna_names = Vec::new();

        for (x, line) in content.iter().enumerate() {
            for (y, c) in line.char_indices() {
                if c != '.' {
                    // found antenna
                    antenna_positions.push([x as isize, y as isize]);
                    antenna_names.push(c)
                }
            }
        }

        let mut antinode_positions = HashSet::new();

        let n = antenna_positions.len();
        for i in 0..n {
            let x = antenna_positions[i][0];
            let y = antenna_positions[i][1];
            let ant_i = &antenna_names[i];
            for j in 0..n {
                let xj = antenna_positions[j][0];
                let yj = antenna_positions[j][1];
                let ant_j = &antenna_names[j];

                if ant_i != ant_j || i == j {
                    continue;
                }

                let dx = (x - xj).abs();
                let dy = (y - yj).abs();
                let mut out_of_bounds_a = false;
                let mut out_of_bounds_b = false;
                let mut antinode_a = [x, y];
                let mut antinode_b = [xj, yj];
                antinode_positions.insert(antinode_a);
                antinode_positions.insert(antinode_b);

                loop {
                    if antinode_a[0] < antinode_b[0] {
                        antinode_a[0] -= dx;
                        antinode_b[0] += dx;
                    } else {
                        antinode_b[0] -= dx;
                        antinode_a[0] += dx;
                    }
                    if antinode_a[1] < antinode_b[1] {
                        antinode_a[1] -= dy;
                        antinode_b[1] += dy;
                    } else {
                        antinode_b[1] -= dy;
                        antinode_a[1] += dy;
                    }

                    if !out_of_bounds_a
                        && antinode_a[0] >= 0
                        && antinode_a[1] >= 0
                        && antinode_a[0] < width
                        && antinode_a[1] < height
                    {
                        antinode_positions.insert(antinode_a);
                    } else {
                        out_of_bounds_a = true
                    }
                    if !out_of_bounds_b
                        && antinode_b[0] >= 0
                        && antinode_b[1] >= 0
                        && antinode_b[0] < width
                        && antinode_b[1] < height
                    {
                        antinode_positions.insert(antinode_b);
                    } else {
                        out_of_bounds_b = true
                    }
                    if out_of_bounds_a && out_of_bounds_b {
                        break;
                    }
                }
            }
        }

        format!("{}", antinode_positions.len())
    }
}
