struct Partition {
    pub data: HashMap<String, String>,
}

pub struct Store {
    partitions: Vec<Partition>,
}

impl Store {
    fn new(num_partitions: usize) -> Store {
        let mut partitions = Vec::new();

        for _ in 0..num_partitions {
            partitions.push(Partition {
                data: HashMap::new(),
            });
        }

        Store { partitions }
    }
}
