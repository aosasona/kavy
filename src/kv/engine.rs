use std::sync::Arc;

use crate::query::Command;

use super::Store;

pub struct Engine {
    store: Arc<Store>,
}

impl Engine {
    pub fn new(store: Arc<Store>) -> Engine {
        Engine { store }
    }

    pub fn execute(&self, cmd: &Command) -> Result<String, String> {
        Ok(String::from(""))
    }
}
