use std::time;

use rand::{rngs::ThreadRng, Rng};

use crate::{
    qap_instance::QapInstance, qap_solution::QapSolution, search_monitor::SearchMonitor,
    utils::basic_evaluate, utils::swap_delta, MAX_INSTANCE_SIZE,
};

pub fn random_walk(
    instance: &QapInstance,
    run_id: u32,
    instance_name: &str,
    time_limit_micros: u32,
) -> QapSolution {
    let mut monitor: SearchMonitor = SearchMonitor {
        run_id,
        instance_name: instance_name.to_string(),
        search_type: "random_walk".to_string(),
        instance_size: instance.instance_size,
        num_visited_solutions: 1,
        num_evaluations: 1,
        running_time_micros: 0,
        best_assignments: [0; MAX_INSTANCE_SIZE],
        cost_history: vec![],
    };
    let start: time::Instant = time::Instant::now();
    let mut current_solution: QapSolution = QapSolution::random_solution(instance.instance_size);
    let mut best_solution: QapSolution = QapSolution {
        instance_size: current_solution.instance_size,
        assignments: current_solution.assignments,
    };
    let mut current_cost: u32 = basic_evaluate(&instance, &best_solution);
    let mut best_cost: u32 = current_cost;
    monitor.cost_history.push(best_cost);
    let mut rng: ThreadRng = rand::thread_rng();
    while (start.elapsed().as_micros() as u32) < time_limit_micros {
        let idx_a: usize = rng.gen_range(0..instance.instance_size);
        let idx_b: usize =
            (rng.gen_range(0..instance.instance_size - 1) + idx_a + 1) % instance.instance_size;
        let delta: i32 = swap_delta(&instance, &current_solution, idx_a, idx_b);
        current_solution.assignments.swap(idx_a, idx_b);
        current_cost = ((current_cost as i32) + delta) as u32;
        if current_cost < best_cost {
            best_cost = current_cost;
            best_solution = QapSolution {
                instance_size: current_solution.instance_size,
                assignments: current_solution.assignments,
            };
            monitor.cost_history.push(best_cost);
        }
        monitor.num_evaluations += 1;
        monitor.num_visited_solutions += 1;
    }
    monitor.best_assignments = best_solution.assignments;
    let duration: time::Duration = start.elapsed();
    monitor.running_time_micros = duration.as_micros() as u32;
    monitor.export_to_files();
    best_solution
}
