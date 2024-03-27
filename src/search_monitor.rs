use std::{fs::OpenOptions, io::Write};

use crate::MAX_INSTANCE_SIZE;

pub struct SearchMonitor {
    pub run_id: u32,
    pub instance_name: String,
    pub search_type: String,
    pub instance_size: usize,
    pub num_visited_solutions: u32,
    pub num_evaluations: u32,
    pub running_time_ms: u32,
    pub best_assignments: [usize; MAX_INSTANCE_SIZE],
    pub cost_history: Vec<u32>,
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
                + &self.running_time_ms.to_string()
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
            Err(why) => panic!("couldn't create {}: {}", file_name, why),
            Ok(file) => file,
        };
        let _ = history_file.write((self.run_id.to_string() + ";").as_bytes());
        let history_string_vector: Vec<String> = self
            .cost_history
            .iter()
            .map(|x: &u32| x.to_string())
            .collect();
        let history_string: String = history_string_vector.join(",");
        let _ = history_file.write((history_string + "\n").as_bytes());
    }
}
