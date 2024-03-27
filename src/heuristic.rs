
use crate::{qap_instance::QapInstance, qap_solution::QapSolution, MAX_INSTANCE_SIZE};

/// Taken from: https://stackoverflow.com/questions/69764050/how-to-get-the-indices-that-would-sort-a-vec
fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

pub fn rank_alignment_solution(instance: &QapInstance) -> QapSolution {
    // feels very "weak" (solutions are quite easily surpassed by random search)
    // but I have no better ideas
    // Maybe it will prove itself as a good starting point for LS?
    let mut cost_sums: Vec<u32> = vec![];
    let mut interaction_sums: Vec<u32> = vec![];
    for i in 0..instance.instance_size {
        let mut cost_sum: u32 = 0;
        let mut interaction_sum: u32 = 0;
        for j in 0..instance.instance_size {
            cost_sum += instance.costs[i][j];
            interaction_sum += instance.interactions[i][j];
        }
        cost_sums.push(cost_sum);
        interaction_sums.push(interaction_sum);
    }
    let cost_order: Vec<usize> = argsort(&cost_sums);
    let interactions_order: Vec<usize> = argsort(&interaction_sums);
    let mut heuristic_assignments: [usize; MAX_INSTANCE_SIZE] = [0; MAX_INSTANCE_SIZE];
    for i in 0..instance.instance_size {
        heuristic_assignments[cost_order[i]] = interactions_order[instance.instance_size-i-1];
    }
    QapSolution{instance_size: instance.instance_size, assignments: heuristic_assignments}
}