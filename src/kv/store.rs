use std::collections::HashMap;

struct Partition {
    pub data: HashMap<String, String>,
}

pub struct Opts {
    pub num_partitions: Option<usize>,
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
    pub fn new(opts: Opts) -> Result<Store, String> {
        let num_partitions = match opts.num_partitions {
            Some(num) => {
                if num > 0 {
                    num
                } else {
                    return Err(String::from("number of partitions must be greater than 0"));
                }
            }
            None => 1,
        };

        let mut partitions = Vec::new();
        for _ in 0..num_partitions {
            partitions.push(Partition::new());
        }

        Ok(Store {
            partitions,
            num_partitions,
        })
    }
}
