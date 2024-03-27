use crate::MAX_INSTANCE_SIZE;
use rand::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct QapSolution {
    pub instance_size: usize,
    pub assignments: [usize; MAX_INSTANCE_SIZE],
}

impl fmt::Display for QapSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instance size: {}\n", self.instance_size).unwrap();
        write!(f, "Assignments\n").unwrap();
        for i in 0..self.instance_size {
            write!(f, "{} ", self.assignments[i]).unwrap();
        }
        write!(f, "\n").unwrap();
        Ok(())
    }
}

impl QapSolution {
    pub fn random_solution(instance_size: usize) -> Self {
        let mut array: [usize; MAX_INSTANCE_SIZE] = [0; MAX_INSTANCE_SIZE];
        let mut rng: ThreadRng = rand::thread_rng();

        for i in 0..instance_size {
            array[i] = i;
        }
        for i in 0..instance_size {
            let src_idx: usize = rng.gen_range(0..instance_size - i);
            let target_idx: usize = instance_size - i - 1;
            let tmp = array[src_idx];
            array[src_idx] = array[target_idx];
            array[target_idx] = tmp;
        }
        QapSolution {
            instance_size,
            assignments: array,
        }
    }
}
