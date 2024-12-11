use crate::problem::Problem;
use std::fs;
use std::path::Path;

pub struct DayNine {}

impl Problem for DayNine {
    fn part_one(&self, input: &Path) -> String {
        let content = fs::read_to_string(input)
            .expect("Failed to read input file")
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as usize)
            .collect::<Vec<_>>();

        let spaces = content.iter().skip(1).step_by(2);
        let files = content.iter().step_by(2);

        let total_files: usize = files.clone().sum();
        let mut file_system = vec![];
        let mut k = files.clone().count() - 1;

        'main_loop: for (i, &file) in files.clone().enumerate() {
            for _ in 0..file {
                file_system.push(i);
                if file_system.len() == total_files {
                    break 'main_loop;
                }
            }

            if let Some(&space) = spaces.clone().nth(i) {
                for _ in 0..space {
                    let mut right_end = *files.clone().nth(k).unwrap(); // Ensure `right_end` is a value, not a reference

                    if right_end > 0 {
                        file_system.push(k);
                    } else {
                        k -= 1;
                        right_end = *files.clone().nth(k).unwrap() - 1; // Correct assignment with dereference
                        file_system.push(k);
                    }
                    right_end -= 1; // Subtract directly as `right_end` is now a `usize`

                    if file_system.len() == total_files {
                        break 'main_loop;
                    }
                }
            }
        }

        println!("{:?}", file_system);

        let checksum: usize = file_system.iter().enumerate().map(|(i, &d)| d * i).sum();
        checksum.to_string()
    }

    fn part_two(&self, input: &Path) -> String {
        let content = fs::read_to_string(input)
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        let spaces = content
            .iter()
            .skip(1)
            .step_by(2)
            .copied()
            .collect::<Vec<usize>>();
        let mut files = content.iter().step_by(2).copied().collect::<Vec<usize>>();

        let mut file_system = vec![];
        for (i, file) in files.iter().enumerate() {
            for _ in 0..*file {
                file_system.push(i as isize);
            }
            if i < spaces.len() - 1 {
                for _ in 0..spaces[i] {
                    file_system.push(-1);
                }
            }
        }

        'files_loop: for (files_index, file_to_move_length) in files.iter().enumerate().rev() {
            let mut file_to_move_index = file_system
                .iter()
                .rposition(|p| *p == files_index as isize)
                .unwrap();
            'file_loop: for i in 0..file_system.len() {
                if file_system[i] == -1 {
                    // space detected
                    let mut space_length = i;
                    while let Some(j) = file_system.get(space_length) {
                        if *j != -1 {
                            space_length -= i;
                            break;
                        }
                        space_length += 1;
                    }

                    if file_system[file_to_move_index] == -1 {
                        break;
                    }
                    if space_length >= *file_to_move_length {
                        if i + space_length < file_system.len() {
                            for j in i..i + space_length {
                                if j - i >= *file_to_move_length {
                                    break;
                                }
                                if j > file_to_move_index {
                                    break 'file_loop;
                                }
                                file_system.swap(j, file_to_move_index);
                                if file_to_move_index > 0 {
                                    file_to_move_index -= 1;
                                } else {
                                    break 'files_loop;
                                }
                            }
                            continue 'files_loop;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        let checksum = file_system
            .iter()
            .enumerate()
            .map(|(i, d)| if *d >= 0 { *d as usize * i } else { 0 })
            .sum::<usize>();
        format!("{}", checksum)
    }
}
