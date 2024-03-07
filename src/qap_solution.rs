use crate::MAX_INSTANCE_SIZE;
use rand::prelude::*;

#[derive(Debug)]
pub struct QapSolution {
    instance_size: u32,
    assignments: [u32; MAX_INSTANCE_SIZE],
}

impl QapSolution {
    pub fn random_solution(instance_size: u32) -> Self {
        let mut array: [u32; MAX_INSTANCE_SIZE] = [0; MAX_INSTANCE_SIZE];
        let mut rng: ThreadRng = rand::thread_rng();

        let u_instance_size: usize = instance_size as usize;
        for i in 0..u_instance_size {
            let val: u32 = i as u32;
            array[i] = val;
        }
        for i in 0..u_instance_size {
            let src_idx: usize = rng.gen_range(0..u_instance_size - i);
            let target_idx: usize = u_instance_size - i - 1;
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
