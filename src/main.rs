pub mod qap_instance;
pub mod search_monitor;
use qap_instance::QapInstance;
pub mod qap_solution;
use qap_solution::QapSolution;
pub mod steepest;
pub mod utils;
use std::env;

pub const MAX_INSTANCE_SIZE: usize = 256;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    const INSTANCE_NAME: &str = "tai256c";
    let path: String = format!("qap_data/{INSTANCE_NAME}.dat");
    let example_instance: QapInstance = QapInstance::instance_from_file(&path);
    steepest::deltas_steepest_local_search(&example_instance, 0, INSTANCE_NAME);
}
