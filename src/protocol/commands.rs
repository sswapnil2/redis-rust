use crate::protocol::store::Store;
use super::value::{Value};

#[derive(Debug)]
pub struct PingCommand;

#[derive(Debug)]
pub struct EchoCommand {
    pub(crate) value: Value
}

#[derive(Debug)]
pub struct SetCommand {
    key: String,
    value: Value,
    args: Option<Vec<String>>
}



impl SetCommand {
    pub fn from(values: Vec<Value>) -> Option<Self> {

        if values.is_empty() {
            return None;
        }

        if values.len() < 2 {
            return None
        }

        if let Value::String(key) = values.first()? {
            let value = values.get(1).cloned()?;

            return Some(SetCommand {
                key: key.clone(),
                value,
                args: None
            })
        }
        None
    }
}

impl CommandExecutor for SetCommand {
    fn execute(&self, store: &mut Store) -> Option<String> {
        store.put(self.key.as_str(), self.value.clone());
        Some(String::from("+OK\r\n"))
    }
}


#[derive(Debug)]
pub struct GetCommand {
    key: String
}

impl GetCommand {

    pub fn from(values: Vec<Value>) -> Option<Self> {
        if let Value::String(key) = values.first()? {
            return Some(GetCommand {
                key: key.to_owned()
            })
        }
        None
    }
}

impl CommandExecutor for GetCommand {
    fn execute(&self, store: &mut Store) -> Option<String> {

        if let Some(value) = store.get(self.key.as_str()) {
            Some(value.to_response_string())
        } else {
            Some(String::from("$-1\r\n"))
        }
    }
}


pub(super) trait CommandExecutor {
    fn execute(&self, store: &mut Store) -> Option<String>;
}

impl CommandExecutor for EchoCommand {
    fn execute(&self, _: &mut Store) -> Option<String> {

        // if let Value::List(vec) = &self.value {
        //     let out: Vec<String> = vec.iter().skip(1).map(|s| { s.to_response_string().to_owned() }).collect();
        //     println!("{:?}", &out);
        //     return Some(out.first().cloned()?)
        // }
        if let Value::String(_) = &self.value {
            return Some(self.value.to_response_string());
        }
        None
    }
}

impl CommandExecutor for PingCommand {
    fn execute(&self, _: &mut Store) -> Option<String> {
        Some(String::from("+PONG\r\n"))
    }
}


#[cfg(test)]
mod tests {
    use crate::protocol::commands::SetCommand;
    use crate::protocol::value::Value;

    #[test]
    pub fn test_set_command() {

        let values = vec![Value::String("foo".to_string()), Value::String("bar".to_string())];

        let command = SetCommand::from(values);
        assert!(command.is_some());
        let c = command.unwrap();
        assert_eq!("foo".to_string(), c.key);
        assert_eq!(Value::String("bar".to_string()), c.value);
    }
}