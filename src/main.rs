pub mod qap_instance;
use qap_instance::QapInstance;
pub mod qap_solution;
use qap_solution::QapSolution;


pub const MAX_INSTANCE_SIZE: usize = 256;

fn main() {
    const PATH: &str = "./qap_data/tai10a.dat";
    let example_instance: QapInstance = QapInstance::instance_from_file(PATH);
    println!("{}", example_instance);
    let random_solution: QapSolution = QapSolution::random_solution(10);
    println!("{:?}", random_solution)
}
