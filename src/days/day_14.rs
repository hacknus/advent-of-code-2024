use crate::io::read_file_lines;
use crate::problem::Problem;
use ndarray::{s, Array2, Array3};
use regex::Regex;
use std::borrow::Cow;
use std::fs::File;
use std::path::Path;
use video_rs::encode::{Encoder, Settings};
use video_rs::frame::PixelFormat;
use video_rs::time::Time;
use video_rs::Options;

pub struct DayFourteen {}

impl Problem for DayFourteen {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let re = Regex::new(r"p=(-?\d*),(-?\d*) v=(-?\d*),(-?\d*)").unwrap();

        let mut robot_positions = vec![];
        let mut robot_velocities = vec![];
        for line in content {
            let captures = re.captures(&line).unwrap();

            let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let vx = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let vy = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
            robot_positions.push([x, y]);
            robot_velocities.push([vx, vy]);
        }

        let width = 101;
        let height = 103;

        // iterate through 100 steps
        for _t in 0..100 {
            for (pos, vel) in robot_positions.iter_mut().zip(robot_velocities.iter()) {
                pos[0] += vel[0];
                pos[1] += vel[1];
                if pos[0] < 0 {
                    pos[0] += width;
                }
                if pos[1] < 0 {
                    pos[1] += height;
                }
                pos[0] %= width;
                pos[1] %= height;
            }
        }

        let mut quadrants = [0; 4];
        for pos in robot_positions.iter() {
            if pos[0] < width / 2 && pos[1] < height / 2 {
                // first quadrant
                quadrants[0] += 1;
            } else if pos[0] > width / 2 && pos[1] < height / 2 {
                // second quadrant
                quadrants[1] += 1;
            } else if pos[0] < width / 2 && pos[1] > height / 2 {
                // third quadrant
                quadrants[2] += 1;
            } else if pos[0] > width / 2 && pos[1] > height / 2 {
                // fourth quadrant
                quadrants[3] += 1;
            }
        }
        format!("{}", quadrants.iter().product::<i32>())
    }

    fn part_two(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        let re = Regex::new(r"p=(-?\d*),(-?\d*) v=(-?\d*),(-?\d*)").unwrap();

        let mut robot_positions = vec![];
        let mut robot_velocities = vec![];
        for line in content {
            let captures = re.captures(&line).unwrap();

            let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let vx = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let vy = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
            robot_positions.push([x, y]);
            robot_velocities.push([vx, vy]);
        }

        let width = 101;
        let height = 103;

        // iterate through 100 steps
        let mut t = 0;

        video_rs::init().unwrap();

        let settings = Settings::preset_h264_yuv420p(1280 / 4 as usize, 720 / 4 as usize, false);
        let mut encoder =
            Encoder::new(Path::new("day_14.mp4"), settings).expect("failed to create encoder");

        let duration: Time = Time::from_nth_of_a_second(12);
        let mut position = Time::zero();

        let mut grid = Array3::zeros((720 / 4 as usize, 1280 / 4 as usize, 3));

        loop {
            grid = Array3::zeros((720 / 4 as usize, 1280 / 4 as usize, 3));
            for (pos, vel) in robot_positions.iter_mut().zip(robot_velocities.iter()) {
                pos[0] += vel[0];
                pos[1] += vel[1];
                if pos[0] < 0 {
                    pos[0] += width;
                }
                if pos[1] < 0 {
                    pos[1] += height;
                }
                pos[0] %= width;
                pos[1] %= height;
                *grid.get_mut((pos[0] as usize, pos[1] as usize, 0)).unwrap() = 255_u8;
            }
            t += 1;

            encoder
                .encode(&grid, position)
                .expect("failed to encode frame");

            // Update the current position and add the inter-frame duration to it.
            position = position.aligned_with(duration).add();

            if t >= 100 {
                break;
            }
        }
        encoder.finish().expect("failed to finish encoder");

        format!("{}", t)
    }
}
