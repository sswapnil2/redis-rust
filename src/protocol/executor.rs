use std::process::Command;
use crate::protocol::resp::Resp;

pub struct Executor;

impl Executor {

    pub fn execute(value: &[u8]) -> Option<String> {

        let resp = Resp::parse(value)?;

        Self::execute_command(&resp)
    }

    // find command
    fn execute_command(resp: &Resp) -> Option<String> {
        if resp.data.is_empty() {
            if resp.values.is_empty() {
                None
            } else {
                let values: &Vec<Resp> = &resp.values;

                if values.len() < 1 {
                    return None
                }

                let command = super::commands::Command::parse(values[0].data.as_str());
                if command == super::commands::Command::None {
                    return None;
                }
                command.execute(&values[1..])
            }
        } else {
            let command = super::commands::Command::parse(resp.data.as_str());
            if command == super::commands::Command::None {
                return None;
            }
            command.execute(&resp.values)
        }

    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_echo() {
        let out = Executor::execute(b"*2\r\n$4\r\nECHO\r\n$6\r\nbanana\r\n");
        assert!(out.is_some());
        assert_eq!("$6\r\nbanana\r\n", out.unwrap().as_str())

    }

    #[test]
    pub fn test_ping() {
        let out = Executor::execute(b"*1\r\n$4\r\nPING\r\n");
        assert!(out.is_some());
        assert_eq!("+PONG\r\n", out.unwrap().as_str())

    }
}
