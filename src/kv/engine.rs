use super::store::Store;

pub struct Engine {
    store: Store,
}

impl Engine {
    pub fn new(store: Store) -> Engine {
        Engine { store }
    }
}
