use crate::QapInstance;
use crate::QapSolution;

pub fn basic_evaluate(instance: QapInstance, solution: QapSolution) -> u32 {
    let mut cost: u32 = 0;
    for i in 0..instance.instance_size {
        for j in 0..instance.instance_size {
            let row: usize = i as usize;
            let col: usize = j as usize;
            cost += instance.costs[row][col]
                * instance.interactions[solution.assignments[row] as usize]
                    [solution.assignments[col] as usize];
        }
    }
    cost
}
