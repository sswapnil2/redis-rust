use std::process::Command;
use crate::protocol::command_type;
use crate::protocol::command_type::CommandType;
use crate::protocol::commands::CommandExecutor;
use crate::protocol::parser::Parser;
use crate::protocol::store::Store;

pub struct Executor;

impl Executor {

    pub fn execute(store: &mut Store, value: &[u8]) -> Option<String> {

        let command_type = Parser::parse(value)?;
        command_type.execute(store)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_echo() {
        let mut s = Store::new();
        let out = Executor::execute( &mut s, b"*2\r\n$4\r\nECHO\r\n$6\r\nbanana\r\n");
        assert!(out.is_some());
        assert_eq!("$6\r\nbanana\r\n", out.unwrap().as_str())

    }

    #[test]
    pub fn test_ping() {
        let mut s = Store::new();
        let out = Executor::execute(&mut s, b"*1\r\n$4\r\nPING\r\n");
        assert!(out.is_some());
        assert_eq!("+PONG\r\n", out.unwrap().as_str())

    }

    #[test]
    pub fn test_set_and_get() {
        let mut s = Store::new();
        let out = Executor::execute(&mut s, b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");
        assert!(out.is_some());
        assert_eq!("+OK\r\n", out.unwrap().as_str());

        let get_output = Executor::execute(&mut s, b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n");
        assert!(get_output.is_some());
        assert_eq!("$3\r\nbar\r\n", get_output.unwrap().as_str());
    }
}
