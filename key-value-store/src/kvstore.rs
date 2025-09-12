use std::collections::HashMap;

pub struct KVStore {
    store: std::collections::HashMap<String, String>,
}

impl KVStore {

    pub fn new() -> Self {
        KVStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &String, value: &String) {
        self.store.insert(key.clone(), value.clone());
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &String) -> Option<String> {
        self.store.remove(key)
    }

    pub fn list(&self) -> Vec<(&String, &String)> {
        self.store.iter().collect()
    }
}