use rand::prelude::*;
use std::time;

fn random_permutation<const LENGTH: usize>(
    rng: &mut ThreadRng,
    mut array: [i32; LENGTH],
) -> [i32; LENGTH] {
    for i in 0..LENGTH {
        let val: i32 = i as i32;
        array[i] = val;
    }
    for i in 0..LENGTH {
        let src_idx: usize = rng.gen_range(0..LENGTH - i);
        let target_idx: usize = LENGTH - i - 1;
        let tmp = array[src_idx];
        array[src_idx] = array[target_idx];
        array[target_idx] = tmp;
        array.swap(src_idx, target_idx);
    }
    array
}

fn main() {
    const MAX_TIME: time::Duration = time::Duration::from_millis(1000);
    const LENGTH: usize = 10;
    const MIN_REPEATS: u32 = 1_000_000;
    let mut repeats: u32 = 0;
    let mut rng: ThreadRng = rand::thread_rng();
    let mut array: [i32; 10] = [0; LENGTH];
    let start: time::Instant = time::Instant::now();
    while start.elapsed() < MAX_TIME || repeats < MIN_REPEATS {
        array = random_permutation::<LENGTH>(&mut rng, array);
        repeats += 1;
    }
    let duration: time::Duration = start.elapsed();
    println!("Elapsed time {:?}", duration);
}
