use crate::search_monitor::SearchMonitor;
use crate::utils;
use crate::utils::basic_evaluate;
use crate::utils::swap_delta;
use crate::QapInstance;
use crate::QapSolution;
use crate::MAX_INSTANCE_SIZE;
use std::cmp::max;
use std::cmp::min;
use std::i32::MAX;
use std::time;

const NUM_CANDIDATES: usize = 10;

/// Returns: list of candidate swaps - NODE IDs, idx_a_best_swap, idx_b_best_swap, best delta, worst delta
fn find_candidates(
    instance: &QapInstance,
    solution: &QapSolution,
    monitor: &mut SearchMonitor,
) -> ([[usize; 2]; NUM_CANDIDATES], usize, usize, i32, i32) {
    // need only the best delta (it will be accepted by the search)
    // and the worst delta (will be used to set the threshold)
    let mut all_swaps: [[usize; 2]; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE] =
        [[0, 0]; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE];
    let mut all_deltas: [i32; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE] =
        [MAX; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE];
    let mut idx: usize = 0;
    let mut best_delta: i32 = MAX;
    let mut best_swap_idx_a: usize = 0;
    let mut best_swap_idx_b: usize = 0;
    for i in 0..(instance.instance_size - 1) {
        for j in i + 1..(instance.instance_size) {
            let delta: i32 = swap_delta(instance, solution, i, j);
            all_deltas[idx] = delta;
            all_swaps[idx] = [
                min(solution.assignments[i], solution.assignments[j]),
                max(solution.assignments[i], solution.assignments[j]),
            ];
            if delta < best_delta {
                best_delta = delta;
                best_swap_idx_a = i;
                best_swap_idx_b = j;
            }
            idx += 1;
            monitor.num_evaluations += 1;
        }
    }
    let mut order: Vec<usize> = utils::argsort(&all_deltas);
    order.truncate(NUM_CANDIDATES);
    let deltas: [i32; NUM_CANDIDATES] = order
        .iter()
        .map(|x: &usize| all_deltas[*x])
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();
    let swaps: [[usize; 2]; NUM_CANDIDATES] = order
        .iter()
        .map(|x: &usize| all_swaps[*x])
        .collect::<Vec<[usize; 2]>>()
        .try_into()
        .unwrap();
    // all swaps IDS, idx_a_best_swap, idx_b_best_swap, best delta, worst delta
    (
        swaps,
        best_swap_idx_a,
        best_swap_idx_b,
        deltas[0],
        deltas[NUM_CANDIDATES - 1],
    )
}

fn is_candidate_good_delta(
    instance: &QapInstance,
    solution: &QapSolution,
    tabu_list: &[[u32; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE],
    swapped_nodes: &[usize; 2],
    aspiration_delta: i32,
    acceptance_delta: i32,
) -> (bool, usize, usize, i32) {
    let idx_a: usize = solution
        .assignments
        .iter()
        .position(|&x| x == swapped_nodes[0])
        .unwrap();
    let idx_b: usize = solution
        .assignments
        .iter()
        .position(|&x| x == swapped_nodes[1])
        .unwrap();
    let delta: i32 = utils::swap_delta(&instance, &solution, idx_a, idx_b);
    // move is too bad to be accepted
    if delta >= acceptance_delta {
        return (false, idx_a, idx_b, delta);
    }
    // move is so good that it must be accepted
    if delta <= aspiration_delta {
        return (true, idx_a, idx_b, delta);
    }
    // reject the move if it is tabu
    if tabu_list[swapped_nodes[0]][swapped_nodes[1]] > 0 {
        return (false, idx_a, idx_b, delta);
    }
    // accept a decent, not-tabu move
    (true, idx_a, idx_b, delta)
}

/// Returns: swap_idx1, swap_idx2, delta
fn tabu_best_swap(
    instance: &QapInstance,
    solution: &QapSolution,
    tabu_list: &[[u32; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE],
    best_cost: u32,
    current_cost: u32,
    cost_threshold: u32,
    candidate_swaps: &mut [[usize; 2]; NUM_CANDIDATES],
    good_candidates: &mut usize,
    monitor: &mut SearchMonitor,
) -> (usize, usize, i32) {
    let opt_required_delta: i32 = best_cost as i32 - current_cost as i32 - 1;
    let threshold_required_delta: i32 = cost_threshold as i32 - current_cost as i32;
    let mut best_candidate_delta: i32 = MAX;
    let mut bad_candidate_count: usize = 0;
    let mut chosen_candidate: usize = 0;
    let mut good_candidates_found: usize = 0;
    let mut swap_locations: [[usize; 2]; NUM_CANDIDATES] = [[0; 2]; NUM_CANDIDATES];
    for i in 0..*good_candidates {
        let (is_good, idx_a, idx_b, delta): (bool, usize, usize, i32) = is_candidate_good_delta(
            instance,
            solution,
            tabu_list,
            &candidate_swaps[i],
            opt_required_delta,
            threshold_required_delta,
        );
        // the swap is evaluated inside is_candidate_good()
        swap_locations[i] = [idx_a, idx_b];
        monitor.num_evaluations += 1;
        if !is_good {
            // move the swap to the first position available for bad candidates
            candidate_swaps.swap(i, *good_candidates - bad_candidate_count);
            bad_candidate_count += 1;
        } else {
            // make sure that good candidates are towards the beginning
            candidate_swaps.swap(good_candidates_found, i);
            if delta < best_candidate_delta {
                chosen_candidate = i;
                best_candidate_delta = delta;
            }
            good_candidates_found += 1;
        }
    }
    *good_candidates -= bad_candidate_count;
    if *good_candidates == 0 {
        // special case, handled in the search function
        return (0, 0, 1);
    }
    *good_candidates -= 1;
    (
        swap_locations[chosen_candidate][0],
        swap_locations[chosen_candidate][1],
        best_candidate_delta,
    )
}

fn update_tabu_list(
    instance_size: usize,
    id_a: usize,
    id_b: usize,
    tenure: u32,
    tabu_list: &mut [[u32; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE],
) {
    for i in 0..instance_size - 1 {
        for j in i + 1..instance_size {
            tabu_list[i][j] = max(1, tabu_list[i][j]) - 1;
        }
    }
    tabu_list[id_a][id_b] = tenure;
}

pub fn deltas_tabu_search(instance: &QapInstance, run_id: u32, instance_name: &str) -> QapSolution {
    let mut monitor: SearchMonitor = SearchMonitor {
        run_id,
        instance_name: instance_name.to_string(),
        search_type: "tabu_search".to_string(),
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
    let mut current_cost: u32 = basic_evaluate(&instance, &starting_solution);
    let mut best_cost: u32 = current_cost;
    monitor.cost_history.push(best_cost);
    // tabu search specific variables
    let tenure: u32 = (instance.instance_size / 4) as u32;
    let mut tabu_list: [[u32; 256]; 256] = [[0; MAX_INSTANCE_SIZE]; MAX_INSTANCE_SIZE];
    // store node ids
    let mut candidate_swaps: [[usize; 2]; NUM_CANDIDATES] = [[0; 2]; NUM_CANDIDATES];
    let mut best_delta: i32;
    let mut worst_delta: i32;
    let mut good_candidates: usize = 0;
    let mut cost_threshold: u32 = 4_000_000_000;

    // loop helper variables
    let mut iter_without_global_improvement: u32 = 0;
    let no_improvement_cap: u32 = instance.instance_size as u32;
    loop {
        let delta: i32;
        let idx_a: usize;
        let idx_b: usize;
        if good_candidates == 0 {
            (candidate_swaps, idx_a, idx_b, best_delta, worst_delta) =
                find_candidates(instance, &starting_solution, &mut monitor);
            cost_threshold = ((current_cost as i32) + worst_delta) as u32;
            good_candidates = NUM_CANDIDATES - 1;
            if best_delta > 0 {
                break;
            }
            delta = best_delta;
        } else {
            (idx_a, idx_b, delta) = tabu_best_swap(
                &instance,
                &starting_solution,
                &tabu_list,
                best_cost,
                current_cost,
                cost_threshold,
                &mut candidate_swaps,
                &mut good_candidates,
                &mut monitor,
            );
            if (idx_a, idx_b, delta) == (0, 0, 1) {
                iter_without_global_improvement += 1;
                if iter_without_global_improvement == no_improvement_cap {
                    break;
                }
                continue;
            }
        }
        update_tabu_list(
            instance.instance_size,
            starting_solution.assignments[idx_a],
            starting_solution.assignments[idx_b],
            tenure,
            &mut tabu_list,
        );
        // num_evaluations is updated within the swap-finding function
        starting_solution.assignments.swap(idx_a, idx_b);
        monitor.num_visited_solutions += 1;
        current_cost = ((current_cost as i32) + delta) as u32;
        if current_cost < best_cost {
            best_cost = current_cost;
            iter_without_global_improvement = 0;
            monitor.cost_updates_evals.push(monitor.num_evaluations);
            monitor.cost_history.push(current_cost);
            monitor.best_assignments = starting_solution.assignments;
        } else {
            iter_without_global_improvement += 1;
        }
        if iter_without_global_improvement == no_improvement_cap {
            break;
        }
    }
    let duration: time::Duration = start.elapsed();
    monitor.running_time_micros = duration.as_micros() as u32;
    monitor.export_to_files();
    starting_solution
}
