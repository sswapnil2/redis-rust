use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    List(Vec<Value>),
}

impl Value {
    pub fn to_response_string(&self) -> String {

        match self {
            Value::String(s) => format!("${}\r\n{}\r\n", s.len(), s),
            _ => String::from("\r\n")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn string_test() {
        let s = "abc";
        let expected = format!("${}\r\n{}\r\n", s.len(), s);

        assert_eq!(expected, Value::String("abc".to_string()).to_response_string())
    }


}