use std::sync::{Arc, Mutex, MutexGuard};

use crate::query::{Command, Op};

use super::Store;

pub struct Engine {
    store: Arc<Mutex<Store>>,
}

impl Engine {
    pub fn new(store: Arc<Mutex<Store>>) -> Engine {
        Engine { store }
    }

    fn get_store(&self) -> Result<MutexGuard<Store>, String> {
        match self.store.lock() {
            Ok(store) => Ok(store),
            Err(_) => Err(String::from("failed to acquire lock")),
        }
    }

    pub fn execute(&self, cmd: &Command) -> Result<String, String> {
        match cmd.op {
            Op::Set => self.set(cmd),
            Op::Get => self.get(cmd),
            Op::Del => self.del(cmd),
            Op::Flush => self.flush(cmd),
        }
    }

    fn set(&self, cmd: &Command) -> Result<String, String> {
        let key = if let Some(key) = &cmd.key {
            key
        } else {
            return Err(String::from("key not provided!"));
        };

        let value = if let Some(value) = &cmd.value {
            value
        } else {
            return Err(String::from("value not provided!"));
        };

        self.get_store()?.set(key.clone(), value.clone())
    }

    fn get(&self, cmd: &Command) -> Result<String, String> {
        let key = if let Some(key) = &cmd.key {
            key
        } else {
            return Err(String::from("key not specified"));
        };

        match self.get_store()?.get(key) {
            Ok(value) => {
                if value.is_empty() {
                    Ok(String::from("(nil)"))
                } else {
                    Ok(value.clone())
                }
            }
            Err(error) => Err(error),
        }
    }

    fn del(&self, cmd: &Command) -> Result<String, String> {
        let key = if let Some(key) = &cmd.key {
            key
        } else {
            return Err(String::from("key not specified"));
        };

        self.get_store()?.del(key)
    }

    fn flush(&self, _cmd: &Command) -> Result<String, String> {
        self.get_store()?.flush();
        Ok(String::from("OK"))
    }
}
