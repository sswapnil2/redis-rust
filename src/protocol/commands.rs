use std::error::Error;
use crate::protocol::commands::Command::{Echo, PING};
use crate::protocol::resp::Resp;

#[derive(Debug, PartialEq)]
pub enum Command {
    Echo(EchoCommandExecutor),
    PING(PingCommandExecutor),
    None
}

impl Command {
    pub fn parse(value: &str) -> Self {
        let binding = value.to_lowercase();
        let x = binding.as_str();
        match x {
            "echo" => Echo(EchoCommandExecutor),
            "ping" => PING(PingCommandExecutor),
            _ => Command::None
        }
    }

    pub fn execute(&self, values: &[Resp]) -> Option<String> {

        match self {
            Echo(st) => st.execute(values),
            PING(st) => st.execute(values),
            Command::None => None
        }


    }
}


trait CommandExecutor {
    fn execute(&self, resp: &[Resp]) -> Option<String>;
}


#[derive(Debug, PartialEq)]
pub struct EchoCommandExecutor;

#[derive(Debug, PartialEq)]
pub struct PingCommandExecutor;


impl CommandExecutor for EchoCommandExecutor {
    fn execute(&self, resp: &[Resp]) -> Option<String> {

        if resp.is_empty() || resp.len() > 1 {
            return None;
        }
        let resp = resp.get(0)?;
        String::from_utf8(resp.raw.clone()).ok()
    }
}

impl CommandExecutor for PingCommandExecutor {
    fn execute(&self, resp: &[Resp]) -> Option<String> {
        Some(String::from("+PONG\r\n"))
    }
}
