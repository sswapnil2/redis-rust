use crate::protocol::store::Store;
use super::value::{Value};
use std::time::{SystemTime, UNIX_EPOCH};

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
    px: Option<i64>
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

            let mut command = SetCommand {
                key: key.clone(),
                value,
                px: None
            };

            for value in values.chunks(2) {
                if let [Value::String(key), Value::String(v)] = value {
                    if key.to_lowercase() == "px" {
                        if let Ok(k) = v.parse::<i64>() {
                            command.px = Some(k)
                        }
                    }
                }
            }

            return Some(command)
        }
        None
    }
}

impl CommandExecutor for SetCommand {
    fn execute(&self, store: &mut Store) -> Option<String> {
        store.put(self.key.as_str(), self.value.clone());


        if let Some(v) = self.px {
            let time = SystemTime::now().duration_since(UNIX_EPOCH)
                .unwrap().as_millis() + v as u128;
            store.set_expiry(self.key.as_str(), time);
        }
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
            let time = SystemTime::now().duration_since(UNIX_EPOCH)
                .unwrap().as_millis();
            if let Some(expiry_ts) = store.get_expiry(self.key.as_str()) {
                return if expiry_ts > time {
                    Some(value.to_response_string())
                } else {
                    Some(String::from("$-1\r\n"))
                }
            }

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