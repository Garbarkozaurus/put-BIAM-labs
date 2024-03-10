use std::i32::MAX;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::utils::basic_evaluate;
use crate::QapInstance;
use crate::QapSolution;
use crate::MAX_INSTANCE_SIZE;


fn naive_steepest_best_swap(instance: &QapInstance, solution: &QapSolution) -> (usize, usize, i32) {
    let mut best_delta: i32 = MAX;
    let mut neighbors_with_best_delta: usize = 0;
    let mut best_swaps: [[usize; 2]; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE] =
        [[0, 0]; MAX_INSTANCE_SIZE * MAX_INSTANCE_SIZE];
    let original_eval = basic_evaluate(&instance, &solution) as i32;
    for i in 0..(instance.instance_size as usize) - 1 {
        for j in i + 1..(instance.instance_size as usize) {
            let mut swapped_order = solution.assignments.clone();
            swapped_order.swap(i, j);
            let new_solution = QapSolution {
                instance_size: instance.instance_size,
                assignments: swapped_order,
            };
            let new_eval = basic_evaluate(&instance, &new_solution) as i32;
            let delta = new_eval - original_eval;
            if delta == best_delta {
                best_swaps[neighbors_with_best_delta][0] = i;
                best_swaps[neighbors_with_best_delta][1] = j;
                neighbors_with_best_delta+=1;
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
