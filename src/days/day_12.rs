use crate::io::read_file_lines;
use crate::problem::Problem;
use ndarray::{s, Array2};
use std::collections::HashSet;
use std::path::Path;

pub struct DayTwelve {}

fn differentiate_x(grid: &Array2<isize>) -> Array2<isize> {
    let slice_x = grid.slice(s![.., 1..]); // Columns 1 to end
    let slice_x_prev = grid.slice(s![.., ..-1]); // Columns 0 to end-1
    &slice_x - &slice_x_prev
}

fn differentiate_y(grid: &Array2<isize>) -> Array2<isize> {
    let slice_y = grid.slice(s![1.., ..]); // Rows 1 to end
    let slice_y_prev = grid.slice(s![..-1, ..]); // Rows 0 to end-1
    &slice_y - &slice_y_prev
}

pub fn look_around(
    x: isize,
    y: isize,
    c: char,
    map: &Vec<Vec<char>>,
    plants: &mut HashSet<[isize; 2]>,
    nonce: &mut HashSet<[isize; 2]>,
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
            if y == map.len() as isize - 1 && dy == 1 {
                continue;
            }
            if x == map[0].len() as isize - 1 && dx == 1 {
                continue;
            }
            let xi = x + dx;
            let yi = y + dy;

            if c == map[yi as usize][xi as usize] && !plants.contains(&[xi, yi]) {
                plants.insert([xi, yi]);
                nonce.insert([xi, yi]);
                look_around(xi, yi, c, map, plants, nonce);
            }
        }
    }
}

impl Problem for DayTwelve {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let width = content[0].len();
        let height = content.len();

        let mut plants: Vec<HashSet<[isize; 2]>> = vec![];

        let map = content
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut nonce = HashSet::new();
        for y in 0..height {
            'x_loop: for x in 0..width {
                // this is a start point for the plant
                // check if it is already corresponding to a plant set
                if nonce.contains(&[x as isize, y as isize]) {
                    continue 'x_loop;
                }
                let mut plant_set = HashSet::new();
                plant_set.insert([x as isize, y as isize]);
                look_around(
                    x as isize,
                    y as isize,
                    map[y][x],
                    &map,
                    &mut plant_set,
                    &mut nonce,
                );
                plants.push(plant_set);
            }
        }

        // calculate perimeter
        let mut sum = 0;
        for plant in plants.iter() {
            let mut num_sides = 0;
            for plant_location in plant.iter() {
                let x = plant_location[0];
                let y = plant_location[1];
                let mut num_sides_i = 4;
                for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                    let xi = x + dx;
                    let yi = y + dy;
                    if plant.contains(&[xi, yi]) {
                        num_sides_i -= 1;
                    }
                }
                num_sides += num_sides_i;
            }
            sum += num_sides * plant.len();
        }
        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let width = content[0].len();
        let height = content.len();

        let mut plants: Vec<HashSet<[isize; 2]>> = vec![];

        let map = content
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut nonce = HashSet::new();
        for y in 0..height {
            'x_loop: for x in 0..width {
                // this is a start point for the plant
                // check if it is already corresponding to a plant set
                if nonce.contains(&[x as isize, y as isize]) {
                    continue 'x_loop;
                }
                let mut plant_set = HashSet::new();
                plant_set.insert([x as isize, y as isize]);
                look_around(
                    x as isize,
                    y as isize,
                    map[y][x],
                    &map,
                    &mut plant_set,
                    &mut nonce,
                );
                plants.push(plant_set);
            }
        }

        // make grid
        let mut sum = 0;
        for plant in plants.iter() {
            let mut min = [usize::MAX, usize::MAX];
            let mut max = [0, 0];
            // TODO: this should be simplified
            for plant_location in plant.iter() {
                let x = plant_location[0];
                let y = plant_location[1];
                min[0] = min[0].min(x as usize);
                min[1] = min[1].min(y as usize);
                max[0] = max[0].max(x as usize);
                max[1] = max[1].max(y as usize);
            }

            // make grid for this plant, pad it on the edges
            let width = max[0] - min[0] + 1 + 2;
            let height = max[1] - min[1] + 1 + 2;
            let mut grid = Array2::zeros((width, height));

            for plant_location in plant.iter() {
                *grid
                    .get_mut((
                        plant_location[0] as usize + 1 - min[0],
                        plant_location[1] as usize + 1 - min[1],
                    ))
                    .unwrap() = 1_isize;
            }

            let mut num_sides = 0;
            // differentiate in x direction
            let diff_x = differentiate_x(&grid);
            let diff_x_diff_y = differentiate_y(&diff_x);
            num_sides += &diff_x_diff_y
                .iter()
                .map(|x| x.abs() as usize)
                .sum::<usize>()
                / 2;

            let diff_y = differentiate_y(&grid);
            let diff_y_diff_x = differentiate_x(&diff_y);
            num_sides += &diff_y_diff_x
                .iter()
                .map(|x| x.abs() as usize)
                .sum::<usize>()
                / 2;

            sum += num_sides * plant.len();
        }

        format!("{}", sum)
    }
}
