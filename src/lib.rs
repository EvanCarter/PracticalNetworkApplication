use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, val1: String, val2: String) {
        self.map.insert(val1, val2);
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        self.map.remove(&key)
    }
}
