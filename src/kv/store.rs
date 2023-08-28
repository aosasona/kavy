use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

    // this is used to determine which partition to use to store or retrieve a key
    fn hash(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.num_partitions
    }

    pub fn set(&mut self, key: String, value: String) {
        let partition = self.hash(&key);
        self.partitions[partition].set(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        let partition = self.hash(&key);
        self.partitions[partition].get(key)
    }

    pub fn del(&mut self, key: String) {
        let partition = self.hash(&key);
        self.partitions[partition].del(key);
    }

    pub fn flush(&mut self) {
        for partition in &mut self.partitions {
            partition.flush();
        }
    }
}
