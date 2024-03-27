use crate::search_monitor::SearchMonitor;
use crate::utils::basic_evaluate;
use crate::utils::swap_delta;
use crate::QapInstance;
use crate::QapSolution;
use crate::MAX_INSTANCE_SIZE;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::time;

fn deltas_greedy_select_swap(
    instance: &QapInstance,
    solution: &QapSolution,
    neighborhood_size: usize,
    monitor: &mut SearchMonitor,
    rng: &mut ThreadRng,
) -> (usize, usize, i32) {
    let neighborhood_start: usize = rng.gen_range(0..neighborhood_size);
    for i in 0..neighborhood_size {
        let chosen_swap = (i + neighborhood_start) % neighborhood_size;
        // row and column refer to the imaginary "neighborhood matrix"
        let row: usize = chosen_swap / instance.instance_size;
        let column: usize = chosen_swap % instance.instance_size;
        // an ugly workaround to prevent calculating the same deltas twice
        if row >= column {
            continue;
        }
        let delta: i32 = swap_delta(&instance, &solution, row, column);
        monitor.num_evaluations += 1;
        if delta < 0 {
            return (row, column, delta);
        }
    }
    // the value returned here doesn't matter as long as delta is non-negative
    (0, 0, 1)
}

pub fn deltas_greedy_local_search(
    instance: &QapInstance,
    run_id: u32,
    instance_name: &str,
) -> QapSolution {
    let mut monitor: SearchMonitor = SearchMonitor {
        run_id,
        instance_name: instance_name.to_string(),
        search_type: "greedy".to_string(),
        instance_size: instance.instance_size,
        num_visited_solutions: 1,
        num_evaluations: 1,
        running_time_micros: 0,
        best_assignments: [0; MAX_INSTANCE_SIZE],
        cost_history: vec![],
        cost_updates_evals: vec![0],
    };
    let start: time::Instant = time::Instant::now();
    let mut starting_solution: QapSolution = QapSolution::random_solution(instance.instance_size);
    let mut cost: u32 = basic_evaluate(&instance, &starting_solution);
    monitor.cost_history.push(cost);
    let mut rng: ThreadRng = rand::thread_rng();
    let neighborhood_size: usize = instance.instance_size * instance.instance_size;
    loop {
        let (idx_a, idx_b, delta) = deltas_greedy_select_swap(
            &instance,
            &starting_solution,
            neighborhood_size,
            &mut monitor,
            &mut rng,
        );
        if delta >= 0 {
            break;
        }
        // num_evaluations is updated within the swap-finding function
        monitor.cost_updates_evals.push(monitor.num_evaluations);
        starting_solution.assignments.swap(idx_a, idx_b);
        monitor.num_visited_solutions += 1;
        cost = ((cost as i32) + delta) as u32;
        monitor.cost_history.push(cost);
    }
    monitor.best_assignments = starting_solution.assignments;
    let duration: time::Duration = start.elapsed();
    monitor.running_time_micros = duration.as_micros() as u32;
    monitor.export_to_files();
    starting_solution
}
