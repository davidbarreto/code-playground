pub trait KVStore {
    fn set(&mut self, key: &String, value: &String);
    fn get(&self, key: &String) -> Option<&String>;
    fn delete(&mut self, key: &String) -> Option<String>;
    fn list(&self) -> Vec<(&String, &String)>;
}