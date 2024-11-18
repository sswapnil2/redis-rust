use crate::protocol::commands::{CommandExecutor, EchoCommand, GetCommand, PingCommand, SetCommand};
use crate::protocol::store::Store;

pub(super) enum CommandType {
    Ping(PingCommand),
    Echo(EchoCommand),
    Set(SetCommand),
    Get(GetCommand)
}

impl CommandExecutor for CommandType {
    fn execute(&self, store: &mut Store) -> Option<String> {
        match self {
            CommandType::Ping(c) => c.execute(store),
            CommandType::Echo(c) => c.execute(store),
            CommandType::Set(c) => c.execute(store),
            CommandType::Get(c) => c.execute(store)
        }

    }
}


