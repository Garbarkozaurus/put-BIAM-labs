pub mod greedy;
pub mod heuristic;
pub mod qap_instance;
pub mod random_search;
pub mod random_walk;
pub mod search_monitor;
use qap_instance::QapInstance;
pub mod qap_solution;
use qap_solution::QapSolution;
pub mod steepest;
pub mod utils;

pub const MAX_INSTANCE_SIZE: usize = 256;

fn main() {
    let instance_names: [&str; 12] = [
        "tai10a", "tai15a", "tai20a", "tai25b", "tai30b", "tai35b", "tai40b", "tai50b", "tai60a",
        "tai80a", "tai100b", "tai256c",
    ];
    let timeouts: [u32; 12] = [
        100, 150, 200, 700, 1600, 2600, 5200, 12500, 13000, 35000, 240000, 1500000,
    ];
    for num in 0..12 {
        let instance_name: &str = instance_names[num];
        let timeout: u32 = timeouts[num];
        let path: String = format!("qap_data/{instance_name}.dat");
        let example_instance: QapInstance = QapInstance::instance_from_file(&path);
        for i in 0..100 {
            steepest::deltas_steepest_local_search(&example_instance, i, instance_name);
            greedy::deltas_greedy_local_search(&example_instance, i, instance_name);
            random_search::random_search(&example_instance, i, instance_name, timeout);
            random_walk::random_walk(&example_instance, i, instance_name, timeout);
        }
    }
}
