struct Partition {
    pub data: HashMap<String, String>,
}

pub struct Store {
    partitions: Vec<Partition>,
    num_partitions: usize,
}

impl Partition {
    pub fn new() -> Partition {
        Partition {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.data.get(&key)
    }

    pub fn del(&mut self, key: String) {
        self.data.remove(&key);
    }

    pub fn flush(&mut self) {
        self.data.clear();
    }
}

impl Store {
    pub fn new(n: Option<usize>) -> Store {
        let num_partitions = match n {
            Some(num) => num,
            None => 1,
        };

        let mut partitions = Vec::new();
        for _ in 0..num_partitions {
            partitions.push(Partition::new());
        }

        Store {
            partitions,
            num_partitions,
        }
    }
}
