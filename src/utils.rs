use crate::QapInstance;
use crate::QapSolution;

pub fn basic_evaluate(instance: &QapInstance, solution: &QapSolution) -> u32 {
    let mut cost: u32 = 0;
    for row in 0..instance.instance_size {
        for col in 0..instance.instance_size {
            cost += instance.costs[row][col]
                * instance.interactions[solution.assignments[row]][solution.assignments[col]];
        }
    }
    cost
}

pub fn swap_delta(
    instance: &QapInstance,
    solution: &QapSolution,
    idx_a: usize,
    idx_b: usize,
) -> i32 {
    // likely to be the hottest function, optimise carefully!
    let mut old_subcost: u32 = 0;
    let mut new_subcost: u32 = 0;
    // immutable reference
    let sol: &[usize] = &solution.assignments[..instance.instance_size];
    // This cloning operation might take a while, but it makes
    // the code easier to follow (and within my ability to write it)
    let mut sol2: [usize; 256] = solution.assignments.clone();
    sol2.swap(idx_a, idx_b);
    // Impacts only rows and columns idx_a and idx_b of A
    // the rows are multiplied by B in the order of teh permutation
    for i in 0..instance.instance_size {
        // This block handles columns of the "swapped rows"
        old_subcost += instance.costs[idx_a][i] * instance.interactions[sol[idx_a]][sol[i]];
        old_subcost += instance.costs[idx_b][i] * instance.interactions[sol[idx_b]][sol[i]];
        new_subcost += instance.costs[idx_a][i] * instance.interactions[sol2[idx_a]][sol2[i]];
        new_subcost += instance.costs[idx_b][i] * instance.interactions[sol2[idx_b]][sol2[i]];

        // This block handles rows of the "swapped columns"
        old_subcost += instance.costs[i][idx_a] * instance.interactions[sol[i]][sol[idx_a]];
        old_subcost += instance.costs[i][idx_b] * instance.interactions[sol[i]][sol[idx_b]];

        new_subcost += instance.costs[i][idx_a] * instance.interactions[sol2[i]][sol2[idx_a]];
        new_subcost += instance.costs[i][idx_b] * instance.interactions[sol2[i]][sol2[idx_b]];
    }
    // the multiplications at the intersections of the "swapped rows and columns"
    // are counted twice. This is corrected here
    old_subcost -= instance.costs[idx_a][idx_a] * instance.interactions[sol[idx_a]][sol[idx_a]];
    old_subcost -= instance.costs[idx_a][idx_b] * instance.interactions[sol[idx_a]][sol[idx_b]];
    old_subcost -= instance.costs[idx_b][idx_a] * instance.interactions[sol[idx_b]][sol[idx_a]];
    old_subcost -= instance.costs[idx_b][idx_b] * instance.interactions[sol[idx_b]][sol[idx_b]];

    new_subcost -= instance.costs[idx_a][idx_a] * instance.interactions[sol2[idx_a]][sol2[idx_a]];
    new_subcost -= instance.costs[idx_a][idx_b] * instance.interactions[sol2[idx_a]][sol2[idx_b]];
    new_subcost -= instance.costs[idx_b][idx_a] * instance.interactions[sol2[idx_b]][sol2[idx_a]];
    new_subcost -= instance.costs[idx_b][idx_b] * instance.interactions[sol2[idx_b]][sol2[idx_b]];

    new_subcost as i32 - old_subcost as i32
}

/// Taken from: https://stackoverflow.com/questions/69764050/how-to-get-the-indices-that-would-sort-a-vec
pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}
