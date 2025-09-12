
use super::traits::KVStore;
use std::collections::HashMap;

pub struct InMemoryKVStore {
    store: HashMap<String, String>,
}

impl InMemoryKVStore {
    pub fn new() -> Self {
        InMemoryKVStore {
            store: HashMap::new(),
        }
    }
}

impl KVStore for InMemoryKVStore {
    
    fn set(&mut self, key: &String, value: &String) {
        self.store.insert(key.clone(), value.clone());
    }

    fn get(&self, key: &String) -> Option<&String> {
        self.store.get(key)
    }

    fn delete(&mut self, key: &String) -> Option<String> {
        self.store.remove(key)
    }

    fn list(&self) -> Vec<(&String, &String)> {
        self.store.iter().collect()
    }
}