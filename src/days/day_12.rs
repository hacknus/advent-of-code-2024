use crate::io::read_file_lines;
use crate::problem::Problem;
use std::collections::HashSet;
use std::path::Path;

pub struct DayTwelve {}

pub fn look_around(
    x: usize,
    y: usize,
    c: char,
    map: &Vec<Vec<char>>,
    plants: &mut HashSet<[usize; 2]>,
    nonce: &mut HashSet<[usize; 2]>,
) {
    for dx in -1..=1_isize {
        for dy in -1..=1_isize {
            if dx == 0 && dy == 0 {
                continue;
            }
            if dx.abs() == 1 && dy.abs() == 1 {
                continue;
            }
            if x == 0 && dx == -1 {
                continue;
            }
            if y == 0 && dy == -1 {
                continue;
            }
            if y == map.len() - 1 && dy == 1 {
                continue;
            }
            if x == map[0].len() - 1 && dx == 1 {
                continue;
            }
            let xi = (x as isize + dx) as usize;
            let yi = (y as isize + dy) as usize;

            if c == map[yi][xi] && !plants.contains(&[xi, yi]) {
                plants.insert([xi, yi]);
                nonce.insert([xi, yi]);
                look_around(xi, yi, c, map, plants, nonce);
            }
        }
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn to_delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

pub fn look_in_front(
    x: usize,
    y: usize,
    direction: Direction,
    plants: &mut HashSet<[usize; 2]>,
) -> (Direction, [usize; 2]) {
    let delta = direction.to_delta();
    let position = [x as isize + delta.0, y as isize + delta.1];

    // if !plants.contains(&[x, y - 1]) {
    //     // the one up is a legit  tile, we do not change direction
    //     if plants.contains(&[x + 1, y - 1]) {
    //         // is an inside tile
    //         if !plants.contains(&[x + 1, y]) {
    //             // move right
    //             x += 1;
    //             direction_change_counter += 1;
    //         }
    //     } else if plants.contains(&[x - 1, y - 1]) {
    //         if !plants.contains(&[x - 1, y]) {
    //             // move left
    //             x -= 1;
    //             direction_change_counter += 1;
    //         }
    //     } else {
    //         // move up
    //         y -= 1;
    //     }
    // }
    todo!()
}

pub fn count_sides(
    mut x: usize,
    mut y: usize,
    mut direction_change_counter: usize,
    dir: Direction,
    map: &Vec<Vec<char>>,
    plants: &mut HashSet<[usize; 2]>,
) {
    // let's go clockwise
    match dir {
        Direction::Up => {
            // look around for tiles
            if !plants.contains(&[x, y - 1]) {
                // the one up is a legit  tile, we do not change direction
                if plants.contains(&[x + 1, y - 1]) {
                    // is an inside tile
                    if !plants.contains(&[x + 1, y]) {
                        // move right
                        x += 1;
                        direction_change_counter += 1;
                    }
                } else if plants.contains(&[x - 1, y - 1]) {
                    if !plants.contains(&[x - 1, y]) {
                        // move left
                        x -= 1;
                        direction_change_counter += 1;
                    }
                } else {
                    // move up
                    y -= 1;
                }
            } else if !plants.contains(&[x + 1, y]) {
                // the one right is a legit  tile, we do not change direction
                if plants.contains(&[x + 1, y + 1]) {
                    // is an inside tile
                    if !plants.contains(&[x, y + 1]) {
                        // move down
                    }
                } else if plants.contains(&[x + 1, y - 1]) {
                    if !plants.contains(&[x, y - 1]) {
                        // move up
                        y -= 1;
                    }
                } else {
                    // move right
                    x += 1;
                }
            } else if !plants.contains(&[x - 1, y]) {
                // the one left is a legit  tile, we do not change direction
                if plants.contains(&[x - 1, y + 1]) {
                    // is an inside tile
                    if !plants.contains(&[x, y + 1]) {
                        // move down
                        y += 1;
                    }
                } else if plants.contains(&[x - 1, y - 1]) {
                    if !plants.contains(&[x, y - 1]) {
                        // move up
                        y -= 1;
                    }
                } else {
                    // move left
                    x -= 1;
                }
            } else {
                unreachable!()
            }
        }
        Direction::Right => {
            if !plants.contains(&[x + 1, y]) {
                // the one right is a legit  tile, we do not change direction
                if plants.contains(&[x + 1, y + 1]) {
                    // is an inside tile
                    if !plants.contains(&[x, y + 1]) {
                        // move down
                        y += 1;
                    }
                } else if plants.contains(&[x + 1, y - 1]) {
                    if !plants.contains(&[x, y - 1]) {
                        // move up
                        y -= 1;
                    }
                } else {
                    // move right
                    x += 1;
                }
            } else if !plants.contains(&[x, y + 1]) {
                // the one down is a legit  tile, we do not change direction
                if plants.contains(&[x + 1, y + 1]) {
                    // is an inside tile
                    if !plants.contains(&[x + 1, y]) {
                        // move right
                        x += 1;
                    }
                } else if plants.contains(&[x - 1, y + 1]) {
                    if !plants.contains(&[x - 1, y]) {
                        // move left
                        x -= 1;
                    }
                } else {
                    // move down
                    y += 1;
                }
            } else if !plants.contains(&[x, y - 1]) {
                // the one up is a legit  tile, we do not change direction
                if plants.contains(&[x + 1, y - 1]) {
                    // is an inside tile
                    if !plants.contains(&[x + 1, y]) {
                        // move right
                        x += 1;
                    }
                } else if plants.contains(&[x - 1, y - 1]) {
                    if !plants.contains(&[x - 1, y]) {
                        // move left
                        x -= 1;
                    }
                } else {
                    // move up
                    y -= 1;
                }
            } else {
                unreachable!()
            }
        }
        Direction::Down => {
            if !plants.contains(&[x, y + 1]) {
                // the one down is a legit  tile, we do not change direction
                if plants.contains(&[x + 1, y + 1]) {
                    // is an inside tile
                    if !plants.contains(&[x + 1, y]) {
                        // move right
                        x += 1;
                    }
                } else if plants.contains(&[x - 1, y + 1]) {
                    if !plants.contains(&[x - 1, y]) {
                        // move left
                        x -= 1;
                    }
                } else {
                    // move down
                    y += 1;
                }
            } else if !plants.contains(&[x + 1, y]) {
                // the one right is a legit  tile, we do not change direction
                if plants.contains(&[x + 1, y + 1]) {
                    // is an inside tile
                    if !plants.contains(&[x, y + 1]) {
                        // move down
                        y += 1;
                    }
                } else if plants.contains(&[x + 1, y - 1]) {
                    if !plants.contains(&[x, y - 1]) {
                        // move up
                        y -= 1;
                    }
                } else {
                    // move right
                    x += 1;
                }
            } else if !plants.contains(&[x - 1, y]) {
                // the one left is a legit  tile, we do not change direction
                if plants.contains(&[x - 1, y + 1]) {
                    // is an inside tile
                    if !plants.contains(&[x, y + 1]) {
                        // move down
                        y += 1;
                    }
                } else if plants.contains(&[x - 1, y - 1]) {
                    if !plants.contains(&[x, y - 1]) {
                        // move up
                        y -= 1;
                    }
                } else {
                    // move left
                    x -= 1;
                }
            } else {
                unreachable!()
            }
        }
        Direction::Left => {
            if !plants.contains(&[x - 1, y]) {
                // the one left is a legit  tile, we do not change direction
                if plants.contains(&[x - 1, y + 1]) {
                    // is an inside tile
                    if !plants.contains(&[x, y + 1]) {
                        // move down
                        y += 1;
                    }
                } else if plants.contains(&[x - 1, y - 1]) {
                    if !plants.contains(&[x, y - 1]) {
                        // move up
                        y -= 1;
                    }
                } else {
                    // move left
                    x -= 1;
                }
            } else if !plants.contains(&[x, y + 1]) {
                // the one down is a legit  tile, we do not change direction
                if plants.contains(&[x + 1, y + 1]) {
                    // is an inside tile
                    if !plants.contains(&[x + 1, y]) {
                        // move right
                        x += 1;
                    }
                } else if plants.contains(&[x - 1, y + 1]) {
                    if !plants.contains(&[x - 1, y]) {
                        // move left
                        x -= 1;
                    }
                } else {
                    // move down
                    y += 1;
                }
            } else if !plants.contains(&[x, y - 1]) {
                // the one up is a legit  tile, we do not change direction
                if plants.contains(&[x + 1, y - 1]) {
                    // is an inside tile
                    if !plants.contains(&[x + 1, y]) {
                        // move right
                        x += 1;
                    }
                } else if plants.contains(&[x - 1, y - 1]) {
                    if !plants.contains(&[x - 1, y]) {
                        // move left
                        x -= 1;
                    }
                } else {
                    // move up
                    y -= 1;
                }
            } else {
                unreachable!()
            }
        }
    }
}
impl Problem for DayTwelve {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let width = content[0].len();
        let height = content.len();

        let mut plants: Vec<HashSet<[usize; 2]>> = vec![];

        let map = content
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut nonce = HashSet::new();
        for y in 0..height {
            'x_loop: for x in 0..width {
                // this is a start point for the plant
                // check if it is already corresponding to a plant set
                if nonce.contains(&[x, y]) {
                    continue 'x_loop;
                }
                let mut plant_set = HashSet::new();
                plant_set.insert([x, y]);
                look_around(x, y, map[y][x], &map, &mut plant_set, &mut nonce);
                plants.push(plant_set);
            }
        }

        // calculate perimeter
        let mut sum = 0;
        for plant in plants.iter() {
            let mut perimeter = 0;
            for plant_location in plant.iter() {
                let x = plant_location[0];
                let y = plant_location[1];
                for dx in -1..=1_isize {
                    for dy in -1..=1_isize {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if dx.abs() == 1 && dy.abs() == 1 {
                            continue;
                        }
                        let xi = (x as isize + dx) as usize;
                        let yi = (y as isize + dy) as usize;

                        if !plant.contains(&[xi, yi]) {
                            perimeter += 1;
                        }
                    }
                }
            }
            sum += perimeter * plant.len();
        }

        dbg!(&plants.len());
        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}
