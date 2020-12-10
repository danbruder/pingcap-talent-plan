#[derive(Debug)]
pub struct KvStore;

impl KvStore {
    pub fn new() -> Self {
        panic!("atd");
    }
    pub fn get(&self, key: String) -> Option<String> {
        panic!("atd");
    }
    pub fn set(&self, key: String, value: String) {
        panic!("atd");
    }
    pub fn remove(&self, key: String) {
        panic!("atd");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
