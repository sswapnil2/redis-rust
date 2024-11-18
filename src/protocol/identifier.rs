use std::str::from_utf8;
use crate::protocol::constants::{CRLFValidator, CRLF};
use crate::protocol::identifier::Identifier::{Arrays, BulkString, SimpleString};
use crate::protocol::value::Value;

#[derive(Debug, PartialEq)]
pub enum Identifier {
    SimpleString,
    SimpleError,
    BulkString,
    Arrays,
    Integer,
    None
}

impl Identifier {
    pub fn from(c: char) -> Self {

        match c {
            '+' => Identifier::SimpleString,
            '-' => Identifier::SimpleError,
            ':' => Identifier::Integer,
            '$' => Identifier::BulkString,
            '*' => Identifier::Arrays,
            _ => Identifier::None
        }
    }

    pub fn parse(&self, value: &[u8]) -> Option<Value> {
        let wrapper = self.apply(value)?;
        Some(wrapper.value)
    }
}

impl IdentifierStrategy for Identifier {

    fn apply(&self, value: &[u8]) -> Option<ValueWrapper> {
        match self {
            SimpleString => SimpleStringStrategy.apply(value),
            BulkString => BulkStringStrategy.apply(value),
            Arrays => ArrayStrategy.apply(value),
            _ => None
        }

    }
}

pub(super) struct ValueWrapper {
    value: Value,
    end_at: i32
}


pub trait IdentifierStrategy {
    fn apply(&self, value: &[u8]) -> Option<ValueWrapper>;

}


#[derive(Debug, PartialEq)]
pub struct SimpleStringStrategy;

#[derive(Debug, PartialEq)]
pub struct BulkStringStrategy;

#[derive(Debug, PartialEq)]
pub struct ArrayStrategy;


impl IdentifierStrategy for SimpleStringStrategy {
    fn apply(&self, value: &[u8]) -> Option<ValueWrapper> {

        if value.len() == 0 {
            return None;
        }

        let some_index = CRLFValidator::find(&value)?;

        let start_position = some_index + CRLF.len();

        let next_position = CRLFValidator::find(&value[start_position..])?;

        Some (
            ValueWrapper {
                value: Value::String(String::from_utf8(value[start_position..start_position+next_position].to_vec()).unwrap()),
                end_at: (next_position + CRLF.len()) as i32
            }
        )
    }
}

impl IdentifierStrategy for BulkStringStrategy {


    fn apply(&self, value: &[u8]) -> Option<ValueWrapper> {

        if value.len() == 0 {
            return None;
        }

        let length_end_index = CRLFValidator::find(&value)?;

        let length_str = from_utf8(&value[1..length_end_index]).ok()?;

        let length: usize = length_str.parse().ok()?;

        let first_crlf_index_end = length_end_index + CRLF.len();


        let string = String::from_utf8(value[first_crlf_index_end..first_crlf_index_end + length].to_vec()).ok()?;


        let end_index = first_crlf_index_end + length + CRLF.len();


        Some(
            ValueWrapper {
                value: Value::String(string),
                end_at: end_index as i32
            }
        )
    }
}

impl IdentifierStrategy for ArrayStrategy {


    fn apply(&self, value: &[u8]) -> Option<ValueWrapper> {

        if value.len() == 0 {
            return None;
        }


        let length_end_index = CRLFValidator::find(&value)?;

        let length_str = from_utf8(&value[1..length_end_index]).ok()?;

        let num_of_items: usize = length_str.parse().ok()?;

        let first_crlf_index_end = length_end_index + CRLF.len();


        let mut value_index= first_crlf_index_end;

        let mut values = vec![];

        for _  in 0..num_of_items {
            if let Some(resp) = Identifier::from(char::from(value[value_index]))
                .apply(&value[value_index..]) {
                value_index = (resp.end_at + value_index as i32 ) as usize;
                values.push(resp);
            } else {
                return None
            }
        }

        Some(
            ValueWrapper {
                value: Value::List(values.iter().map(|v| v.value.clone()).collect()),
                end_at: value_index as i32
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn simple_strategy() {
        let s = SimpleStringStrategy {};
        assert!(s.apply(b"+\r\nhello\r").is_none());
        assert!(s.apply(b"+\nhello\r").is_none());
        assert!(s.apply(b"+\nhello\r\n").is_none());

        let some_resp = s.apply(b"+\r\nhello\r\n");
        assert!(some_resp.is_some());
        let resp = some_resp.unwrap();
        assert_eq!(resp.value, Value::String(String::from("hello")));
    }


    #[test]
    pub fn bulk_strategy() {
        let s = BulkStringStrategy {};
        let some_resp = s.apply(b"$5\r\nhello\r\n");
        assert!(some_resp.is_some());
        let resp = some_resp.unwrap();
        assert_eq!(resp.value, Value::String(String::from("hello")));
    }

    #[test]
    pub fn array_command_strategy() {
        let s = ArrayStrategy {};
        let some_resp = s.apply(b"*2\r\n$4\r\nECHO\r\n$6\r\nbanana\r\n");
        assert!(some_resp.is_some());
        // let resp = some_resp.unwrap();
        // assert_eq!(resp.data, "hello");

    }
}