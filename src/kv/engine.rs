use std::sync::Arc;

use super::Store;

pub struct Engine {
    store: Arc<Store>,
}

impl Engine {
    pub fn new(store: Arc<Store>) -> Engine {
        Engine { store }
    }
}
