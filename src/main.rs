pub mod qap_instance;
use qap_instance::QapInstance;
pub mod qap_solution;
use qap_solution::QapSolution;
pub mod steepest;
pub mod utils;
use std::env;

use crate::utils::basic_evaluate;

pub const MAX_INSTANCE_SIZE: usize = 256;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    const PATH: &str = "./qap_data/tai10a.dat";
    let example_instance: QapInstance = QapInstance::instance_from_file(PATH);

    let mut count = 0;
    let mut sol: QapSolution = steepest::simplest_steepest_local_search(&example_instance);
    let mut eval: u32 = basic_evaluate(&example_instance, &sol);
    let target = 13557864;
    loop {
        if eval == target {
            break;
        }
        count += 1;
        sol = steepest::simplest_steepest_local_search(&example_instance);
        eval = basic_evaluate(&example_instance, &sol);
        if 0.99 * (eval as f64) <= target as f64 {
            println!("{}. {}", count, eval);
        }
    }
    println!("{}", sol);
}
