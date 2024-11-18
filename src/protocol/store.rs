use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::protocol::value::Value;

#[derive(Debug)]
pub struct Store {
    map: Arc<RwLock<HashMap<String, Value>>>,
}

impl Store {

    pub fn new() -> Self {
        Store {
            map: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn put(&mut self, key: &str, value: Value)  {
        let mut map = self.map.write().unwrap();
        map.insert(key.to_string(), value);
    }

    pub fn get(&mut self, key: &str) -> Option<Value> {
        let map = self.map.read().unwrap();
        map.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn read_write() {
        let mut store = Store::new();

        // integer
        store.put("abc", Value::Int(24));
        let val = store.get("abc").unwrap();
        assert_eq!(val, Value::Int(24));

        // string
        store.put("s", Value::String(String::from("abc")));
        let val = store.get("s").unwrap();
        assert_eq!(val, Value::String(String::from("abc")));

        // // array
        // store.put("a", Value::Array(vec![Value::Integer(234)]));
        // let val = store.get("a").unwrap();
        // assert_eq!(Value::Array(vec![Value::Integer(234)]), val);

        // value not present
        let val = store.get("zya");
        assert!(val.is_none())

    }

}