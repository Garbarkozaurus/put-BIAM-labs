use std::time;

use crate::{
    qap_instance::QapInstance, qap_solution::QapSolution, search_monitor::SearchMonitor,
    utils::basic_evaluate, MAX_INSTANCE_SIZE,
};

pub fn random_search(
    instance: &QapInstance,
    run_id: u32,
    instance_name: &str,
    time_limit_micros: u32,
) -> QapSolution {
    let mut monitor: SearchMonitor = SearchMonitor {
        run_id,
        instance_name: instance_name.to_string(),
        search_type: "random_search".to_string(),
        instance_size: instance.instance_size,
        num_visited_solutions: 1,
        num_evaluations: 1,
        running_time_micros: 0,
        best_assignments: [0; MAX_INSTANCE_SIZE],
        cost_history: vec![],
        cost_updates_evals: vec![0],
    };
    let start: time::Instant = time::Instant::now();
    let mut best_solution: QapSolution = QapSolution::random_solution(instance.instance_size);
    let mut best_cost: u32 = basic_evaluate(&instance, &best_solution);
    monitor.cost_history.push(best_cost);
    while (start.elapsed().as_micros() as u32) < time_limit_micros {
        let solution: QapSolution = QapSolution::random_solution(instance.instance_size);
        let cost: u32 = basic_evaluate(&instance, &solution);
        if cost < best_cost {
            best_cost = cost;
            best_solution = solution;
            monitor.cost_history.push(best_cost);
            monitor.cost_updates_evals.push(monitor.num_evaluations);
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
