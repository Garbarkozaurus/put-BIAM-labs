pub mod qap_instance;
use qap_instance::QapInstance;
pub mod qap_solution;
use qap_solution::QapSolution;
pub mod steepest;
pub mod utils;
use std::env;

use crate::utils::{basic_evaluate, swap_delta};

pub const MAX_INSTANCE_SIZE: usize = 256;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    const PATH: &str = "./qap_data/tai10a.dat";
    let example_instance: QapInstance = QapInstance::instance_from_file(PATH);

    let mut sol: QapSolution = steepest::simplest_steepest_local_search(&example_instance);
    let old_eval = basic_evaluate(&example_instance, &sol) as i32;
    let a = 0;
    let b = 2;
    let delta = swap_delta(&example_instance, &sol, a, b);
    sol.assignments.swap(a, b);
    let new_eval = basic_evaluate(&example_instance, &sol) as i32;
    println!("OG eval: {}", old_eval);
    println!("Delta: {}", delta);
    println!(
        "Estimated new: {}\tActual new: {}\t Error: {}",
        old_eval + delta,
        new_eval,
        old_eval + delta - new_eval
    )
}
