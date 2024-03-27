use std::{fs::OpenOptions, io::Write};

use crate::MAX_INSTANCE_SIZE;

pub struct SearchMonitor {
    pub run_id: u32,
    pub instance_name: String,
    pub search_type: String,
    pub instance_size: usize,
    pub num_visited_solutions: u32,
    pub num_evaluations: u32,
    // 4 billion microseconds is still over an hour - way more than enough
    pub running_time_micros: u32,
    pub best_assignments: [usize; MAX_INSTANCE_SIZE],
    pub cost_history: Vec<u32>,
    // How many evaluations it took before the solution at corresponding position
    // in cost_history was found
    // The first element will always be 0 (for the cost of the initial solution)
    // I decided for it to be 0 and not 1, to avoid problems with plotting,
    // and axes starting at 1.
    // Be mindful of 0,46,91,136... in steepest
    pub cost_updates_evals: Vec<u32>,
}

impl SearchMonitor {
    pub fn export_to_files(&self) {
        let file_name: String =
            String::from("saved_results/") + &self.search_type + "/" + &self.instance_name + ".txt";
        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&file_name)
        {
            Err(why) => panic!("couldn't create {}: {}", file_name, why),
            Ok(file) => file,
        };

        let _ = file.write(
            (self.run_id.to_string()
                + ";"
                + &self.num_visited_solutions.to_string()
                + ";"
                + &self.num_evaluations.to_string()
                + ";"
                + &self.running_time_micros.to_string()
                + ";")
                .as_bytes(),
        );
        // export the best solution
        for i in 0..self.instance_size - 1 {
            let _ = file.write((self.best_assignments[i].to_string() + ",").as_bytes());
        }
        let _ = file.write(
            self.best_assignments[self.instance_size - 1]
                .to_string()
                .as_bytes(),
        );
        let _ = file.write("\n".as_bytes());

        // export the history of best solution costs
        let history_file_name: String = String::from("saved_results/")
            + &self.search_type
            + "/histories/"
            + &self.instance_name
            + ".txt";
        let mut history_file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&history_file_name)
        {
            Err(why) => panic!("couldn't create {}: {}", history_file_name, why),
            Ok(file) => file,
        };
        let _ = history_file.write((self.run_id.to_string() + ";").as_bytes());
        let mut history_string_vector: Vec<String> = vec![];
        for (i, eval_count) in self.cost_updates_evals.iter().enumerate() {
            history_string_vector
                .push(eval_count.to_string() + ":" + &self.cost_history[i].to_string());
        }
        let history_string: String = history_string_vector.join(",");
        let _ = history_file.write((history_string + "\n").as_bytes());
    }
}
