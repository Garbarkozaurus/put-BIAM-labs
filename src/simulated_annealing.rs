use crate::search_monitor::SearchMonitor;
use crate::utils::basic_evaluate;
use crate::utils::swap_delta;
use crate::QapInstance;
use crate::QapSolution;
use crate::MAX_INSTANCE_SIZE;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::time;

fn deltas_simulated_annealing_select_swap(
    instance: &QapInstance,
    solution: &QapSolution,
    neighborhood_size: usize,
    temperature: f32,
    monitor: &mut SearchMonitor,
    rng: &mut ThreadRng,
) -> (usize, usize, i32) {
    let rand_01: Uniform<f32> = Uniform::from(0.0..1.0);
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
        let f_delta: f32 = (-1 * delta) as f32;
        let rand_val: f32 = rand_01.sample(rng);
        if (f_delta / temperature).exp() > rand_val {
            // println!("{} > {} (delta: {delta}, temperature: {temperature})", (f_delta.exp() / temperature), rand_val);
            return (row, column, delta);
        }
    }
    (0, 0, 1)
}

fn find_initial_temperature(
    instance: &QapInstance,
    solution: &QapSolution,
    neighborhood_size: usize,
    acceptance_prob: f32,
) -> f32 {
    let mut deltas: Vec<i32> = vec![];
    for i in 0..neighborhood_size {
        let row: usize = i / instance.instance_size;
        let column: usize = i % instance.instance_size;
        // an ugly workaround to prevent calculating the same deltas twice
        if row >= column {
            continue;
        }
        let delta: i32 = swap_delta(&instance, &solution, row, column);
        deltas.push(delta);
    }
    deltas.sort_unstable();
    let len_deltas: usize = deltas.len();
    let idx: usize = ((len_deltas as f32) * acceptance_prob).ceil() as usize;
    // starting temperature "allows to accept 95% of moves"
    // 0.95 to accept the worst, or >50% to accept the 5th percentile?
    // maybe so that the average probability of accepting a move is 0.95?
    // maybe make this average weighted by deltas?
    let boundary_delta: i32 = deltas[idx];
    // e^(-delta/T) = prob
    // T = -delta/ln(prob)
    -(boundary_delta as f32) / acceptance_prob.ln()
}

pub fn deltas_simulated_annealing(
    instance: &QapInstance,
    run_id: u32,
    instance_name: &str,
) -> QapSolution {
    let mut monitor: SearchMonitor = SearchMonitor {
        run_id,
        instance_name: instance_name.to_string(),
        search_type: "simulated_annealing".to_string(),
        instance_size: instance.instance_size,
        num_visited_solutions: 1,
        num_evaluations: 1,
        running_time_micros: 0,
        best_assignments: [0; MAX_INSTANCE_SIZE],
        cost_history: vec![],
        cost_updates_evals: vec![0],
    };
    let start: time::Instant = time::Instant::now();
    // local search variables
    let mut starting_solution: QapSolution = QapSolution::random_solution(instance.instance_size);
    let neighborhood_size: usize = instance.instance_size * instance.instance_size;
    let mut cost: u32 = basic_evaluate(&instance, &starting_solution);
    monitor.cost_history.push(cost);
    let mut rng: ThreadRng = rand::thread_rng();

    // simulated annealing configuration
    let init_accept_prob: f32 = 0.95;
    let final_accept_prob: f32 = 0.01;
    let cooling_constant: f32 = 0.9;
    let markov_chain_length: u32 = (0.125 * neighborhood_size as f32) as u32;
    let no_improvement_cap: u32 = 10 * markov_chain_length;
    let global_no_improvement_cap: u32 = 2 * no_improvement_cap;
    // e^(-delta/T) = prob
    // Minimum deteriorating delta is 1
    // the final temperature can be calculated using -1/ln(acceptance_prob)
    let final_temperature: f32 = -1.0 / final_accept_prob.ln();
    let mut best_cost: u32 = cost;
    let mut temperature: f32 = find_initial_temperature(
        &instance,
        &starting_solution,
        neighborhood_size,
        init_accept_prob,
    );

    // loop helper variables
    let mut iter_without_improvement: u32 = 0;
    let mut iter_in_chain: u32 = 0;
    let mut iter_without_global_improvement: u32 = 0;
    // main search loop
    loop {
        let (idx_a, idx_b, delta) = deltas_simulated_annealing_select_swap(
            &instance,
            &starting_solution,
            neighborhood_size,
            temperature,
            &mut monitor,
            &mut rng,
        );
        iter_in_chain += 1;
        if iter_in_chain >= markov_chain_length {
            temperature *= cooling_constant;
            iter_in_chain = 0;
        }
        if delta >= 0 {
            iter_without_improvement += 1;
        } else {
            iter_without_improvement = 0;
        }
        // (0, 0, 1 is a special case - when no improvement was found, and no deterioration was accepted)
        // it can be handled in at least two ways:
        // - terminate the algorithm
        // - give the neighborhood another chance - maybe next time some deterioration will be accepted
        // the second approach is chosen here. Only some monitoring operations are omitted
        if (idx_a, idx_b, delta) == (0, 0, 1) {
            iter_without_global_improvement += 1;
            if (iter_without_improvement >= no_improvement_cap && temperature <= final_temperature)
                || iter_without_global_improvement >= global_no_improvement_cap
            {
                break;
            }
            continue;
        }
        // num_evaluations is updated within the swap-finding function
        monitor.cost_updates_evals.push(monitor.num_evaluations);
        starting_solution.assignments.swap(idx_a, idx_b);
        monitor.num_visited_solutions += 1;
        cost = ((cost as i32) + delta) as u32;
        monitor.cost_history.push(cost);
        if cost < best_cost {
            best_cost = cost;
            monitor.best_assignments = starting_solution.assignments;
            iter_without_global_improvement = 0;
        } else {
            iter_without_global_improvement += 1;
        }
        if (iter_without_improvement >= no_improvement_cap && temperature <= final_temperature)
            || iter_without_global_improvement >= global_no_improvement_cap
        {
            break;
        }
    }
    let duration: time::Duration = start.elapsed();
    monitor.running_time_micros = duration.as_micros() as u32;
    // monitor.export_to_files();
    starting_solution
}
