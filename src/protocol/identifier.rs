use crate::protocol::identifier::Identifier::{Arrays, BulkString, SimpleString};
use crate::protocol::resp::Resp;
use super::strategy::{ArrayStrategy, BulkStringStrategy, SimpleStringStrategy};

#[derive(Debug, PartialEq)]
pub enum Identifier {
    SimpleString(SimpleStringStrategy),
    SimpleError,
    BulkString(BulkStringStrategy),
    Arrays(ArrayStrategy),
    Integer,
    None
}

impl Identifier {
    pub fn from(c: char) -> Self {

        match c {
            '+' => Identifier::SimpleString(SimpleStringStrategy),
            '-' => Identifier::SimpleError,
            ':' => Identifier::Integer,
            '$' => Identifier::BulkString(BulkStringStrategy),
            '*' => Identifier::Arrays(ArrayStrategy),
            _ => Identifier::None
        }
    }
}

impl IdentifierStrategy for Identifier {
    fn apply(&self, value: &[u8]) -> Option<Resp> {
        match self {
            SimpleString(strategy) => strategy.apply(value),
            BulkString(st) => st.apply(value),
            Arrays(st) => st.apply(value),
            _ => None
        }

    }
}


pub trait IdentifierStrategy {
    fn apply(&self, value: &[u8]) -> Option<Resp>;

}