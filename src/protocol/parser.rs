use crate::protocol::command_type::CommandType;
use crate::protocol::commands::{EchoCommand, GetCommand, PingCommand, SetCommand};
use crate::protocol::identifier::{Identifier, IdentifierStrategy};
use crate::protocol::value;
use crate::protocol::value::Value;

pub struct Parser;


impl Parser {

    pub fn parse(input: &[u8]) -> Option<CommandType> {

        if input.is_empty() {
            return None
        }

        let ch = char::from(input[0]);

        let value = Identifier::from(ch).parse(input)?;

        match value {
            Value::String(s) => Self::parse_string(s),
            Value::List(vec) => Self::parse_list(vec),
            _ => None,

        }
    }


    fn parse_string(string: String) -> Option<CommandType> {
        match string.to_lowercase().as_str() {
            "ping" => Some(CommandType::Ping(PingCommand)),
            _ => None
        }
    }

    fn parse_list(values: Vec<Value>) -> Option<CommandType> {

        let Value::String(command) = values.first()? else {
            return None
        };

        let all = values.iter().skip(1).map(|a| a.to_owned()).collect();

        match command.to_lowercase().as_str() {
                    "echo" => Some(CommandType::Echo(EchoCommand {
                        value: values.get(1).cloned()?
                    })),
                    "ping" => Some(CommandType::Ping(PingCommand)),
                    "set" => Some(CommandType::Set(SetCommand::from(all)?)),
                    "get" => Some(CommandType::Get(GetCommand::from(all)?)),
                    _ => None
        }
    }



}

