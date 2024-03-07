use std::fmt;
use std::fs;

use crate::MAX_INSTANCE_SIZE;

#[derive(Debug)]
pub struct QapInstance {
    instance_size: u32,
    costs: [[i32; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE],
    interactions: [[i32; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE],
}

impl fmt::Display for QapInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instance size: {}\n", self.instance_size).unwrap();
        write!(f, "Costs\n").unwrap();
        for i in 0..self.instance_size {
            for j in 0..self.instance_size {
                write!(f, "{} ", self.costs[i as usize][j as usize]).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\nInteractions\n").unwrap();
        for i in 0..self.instance_size {
            write!(f, "{}. ", i).unwrap();
            for j in 0..self.instance_size {
                write!(f, "{} ", self.interactions[i as usize][j as usize]).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

impl QapInstance {
    pub fn instance_from_file(file_path: &str) -> Self {
        let contents: String =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let mut numbers_so_far: Vec<Vec<i32>> = vec![];
        let mut instance_size: u32 = 0;
        let mut n_lines_costs: u32 = 0;
        let mut n_lines_interactions: u32 = 0;
        let mut costs: [[i32; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE] =
            [[0; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE];
        let mut interactions: [[i32; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE] =
            [[0; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE];
        let mut lines_read: u32 = 0;
        for line in contents.split("\n") {
            let str_numbers: Vec<&str> = line.split_whitespace().collect();
            if str_numbers.len() == 0 {
                continue;
            }
            numbers_so_far.push(
                str_numbers
                    .iter()
                    .map(|x: &&str| x.parse().expect("Not an integer!"))
                    .collect(),
            );
            lines_read += 1;
            // I really wanted to use match here, but I found no way to compare to a variable
            if lines_read == 1 {
                instance_size = numbers_so_far[0][0] as u32;
                numbers_so_far.clear();
                n_lines_costs = instance_size + 1;
                n_lines_interactions = 2 * instance_size + 1;
            }
            if lines_read == n_lines_costs {
                for (row, numbers) in numbers_so_far.iter().enumerate() {
                    // costs[row] = numbers_so_far[row].clone().into();
                    for (column, number) in numbers.iter().enumerate() {
                        costs[row][column] = *number;
                    }
                }
                numbers_so_far.clear();
            }
            if lines_read == n_lines_interactions {
                for (row, numbers) in numbers_so_far.iter().enumerate() {
                    for (column, number) in numbers.iter().enumerate() {
                        interactions[row][column] = *number;
                    }
                }
                numbers_so_far.clear();
            }
        }

        QapInstance {
            instance_size,
            costs,
            interactions,
        }
    }
}
