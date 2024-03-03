use std::fs;

#[derive(Debug)]
struct QapInstance {
    instance_size: i32,
    costs: Vec<Vec<i32>>,
    interactions: Vec<Vec<i32>>,
}

impl QapInstance {
    fn instance_from_file(file_path: &str) -> Self {
        let mut fields_complete: u8 = 0;
        let contents: String =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let mut numbers_so_far: Vec<Vec<i32>> = vec![];
        let mut instance_size: i32 = 0;
        let mut costs: Vec<Vec<i32>> = vec![];
        let mut interactions: Vec<Vec<i32>> = vec![];
        for line in contents.split("\n") {
            let str_numbers: Vec<&str> = line.split_whitespace().collect();
            if str_numbers.len() == 0 {
                match fields_complete {
                    0 => {
                        instance_size = numbers_so_far[0][0];
                        numbers_so_far.clear();
                    }
                    1 => {
                        costs = numbers_so_far.clone();
                        numbers_so_far.clear();
                    }
                    2 => {
                        interactions = numbers_so_far.clone();
                        numbers_so_far.clear();
                    }
                    _ => (),
                }
                fields_complete += 1;
                continue;
            }
            numbers_so_far.push(
                str_numbers
                    .iter()
                    .map(|x: &&str| x.parse().expect("Not an integer!"))
                    .collect(),
            );
        }

        QapInstance {
            instance_size,
            costs,
            interactions,
        }
    }
}

fn main() {
    const PATH: &str = "./qap_data/tai10a.dat";
    let a: QapInstance = QapInstance::instance_from_file(PATH);
    println!("{:?}", a);
    println!("{:?}", a.instance_size);
    println!("{:?}", a.costs[0][0]);
    println!("{:?}", a.interactions[0][0]);

}
