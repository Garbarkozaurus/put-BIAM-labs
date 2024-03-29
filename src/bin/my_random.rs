use rand::prelude::*;
// use rand::{Rng, SeedableRng};

pub fn random_permutation<const LENGTH: usize>() -> [i32; LENGTH] {
    let mut array: [i32; LENGTH] = [0; LENGTH];
    let mut rng: ThreadRng = rand::thread_rng();

    for i in 0..LENGTH {
        let val: i32 = i as i32;
        array[i] = val;
    }
    for i in 0..LENGTH {
        let src_idx: usize = rng.gen_range(0..LENGTH - i);
        let target_idx: usize = LENGTH - i - 1;
        let tmp: i32 = array[src_idx];
        array[src_idx] = array[target_idx];
        array[target_idx] = tmp;
    }
    return array;
}

fn main() {
    const LENGTH: usize = 100;
    // let mut rng = seed_from_u64(10);
    // println!("Random i32: {}", rng.gen::<i32>(10));
    let arr = random_permutation::<LENGTH>();
    for val in arr.iter() {
        println!("{}", val);
    }
}
