use crate::search_monitor::SearchMonitor;
use crate::utils;
use crate::utils::basic_evaluate;
use crate::utils::swap_delta;
use crate::QapInstance;
use crate::QapSolution;
use crate::MAX_INSTANCE_SIZE;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::i32::MAX;
use std::time;

fn naive_steepest_best_swap(instance: &QapInstance, solution: &QapSolution) -> (usize, usize, i32) {
    let mut best_delta: i32 = MAX;
    let mut neighbors_with_best_delta: usize = 0;
    let mut best_swaps: [[usize; 2]; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE] =
        [[0, 0]; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE];
    let original_eval = utils::basic_evaluate(&instance, &solution) as i32;
    for i in 0..(instance.instance_size - 1) {
        for j in i + 1..(instance.instance_size) {
            let mut swapped_order = solution.assignments.clone();
            swapped_order.swap(i, j);
            let new_solution = QapSolution {
                instance_size: instance.instance_size,
                assignments: swapped_order,
            };
            let new_eval = utils::basic_evaluate(&instance, &new_solution) as i32;
            let delta = new_eval - original_eval;
            if delta == best_delta {
                best_swaps[neighbors_with_best_delta][0] = i;
                best_swaps[neighbors_with_best_delta][1] = j;
                neighbors_with_best_delta += 1;
            }
            if delta < best_delta {
                best_swaps[0][0] = i;
                best_swaps[0][1] = j;
                best_delta = delta;
                neighbors_with_best_delta = 1;
            }
        }
    }
    let mut rng: ThreadRng = rand::thread_rng();
    let best_swap: [usize; 2] = best_swaps[rng.gen_range(0..neighbors_with_best_delta)];
    (best_swap[0], best_swap[1], best_delta)
}

pub fn simplest_steepest_local_search(instance: &QapInstance) -> QapSolution {
    let mut starting_solution: QapSolution = QapSolution::random_solution(instance.instance_size);
    loop {
        let (a, b, delta) = naive_steepest_best_swap(&instance, &starting_solution);
        if delta >= 0 {
            break;
        }
        starting_solution.assignments.swap(a, b);
    }
    starting_solution
}

/// Finds the swap which maximizes the improvement
///
/// Returns: swap_idx1, swap_idx2, delta
fn delta_steepest_best_swap(
    instance: &QapInstance,
    solution: &QapSolution,
    monitor: &mut SearchMonitor,
) -> (usize, usize, i32) {
    let mut best_delta: i32 = MAX;
    let mut neighbors_with_best_delta: usize = 0;
    let mut best_swaps: [[usize; 2]; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE] =
        [[0, 0]; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE];
    for i in 0..(instance.instance_size - 1) {
        for j in i + 1..(instance.instance_size) {
            let delta: i32 = swap_delta(instance, solution, i, j);
            monitor.num_evaluations += 1;
            if delta == best_delta {
                best_swaps[neighbors_with_best_delta][0] = i;
                best_swaps[neighbors_with_best_delta][1] = j;
                neighbors_with_best_delta += 1;
            }
            if delta < best_delta {
                best_swaps[0][0] = i;
                best_swaps[0][1] = j;
                best_delta = delta;
                neighbors_with_best_delta = 1;
            }
        }
    }
    let mut rng: ThreadRng = rand::thread_rng();
    let best_swap: [usize; 2] = best_swaps[rng.gen_range(0..neighbors_with_best_delta)];
    (best_swap[0], best_swap[1], best_delta)
}

pub fn deltas_steepest_local_search(
    instance: &QapInstance,
    run_id: u32,
    instance_name: &str,
) -> QapSolution {
    let mut monitor: SearchMonitor = SearchMonitor {
        run_id,
        instance_name: instance_name.to_string(),
        search_type: "steepest".to_string(),
        instance_size: instance.instance_size,
        num_visited_solutions: 1,
        num_evaluations: 1,
        running_time_micros: 0,
        best_assignments: [0; MAX_INSTANCE_SIZE],
        cost_history: vec![],
    };
    let start: time::Instant = time::Instant::now();
    let mut starting_solution: QapSolution = QapSolution::random_solution(instance.instance_size);
    let mut cost: u32 = basic_evaluate(&instance, &starting_solution);
    monitor.cost_history.push(cost);
    loop {
        let (idx_a, idx_b, delta) =
            delta_steepest_best_swap(&instance, &starting_solution, &mut monitor);
        if delta >= 0 {
            break;
        }
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
