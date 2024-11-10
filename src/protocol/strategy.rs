use std::ops::Index;
use std::str::from_utf8;
use crate::protocol::identifier::Identifier::{Arrays, BulkString, SimpleString};
use crate::protocol::resp::Resp;
use super::identifier::{Identifier, IdentifierStrategy};
use super::constants::{CRLFValidator, CR, CRLF, LF};

#[derive(Debug, PartialEq)]
pub struct SimpleStringStrategy;

#[derive(Debug, PartialEq)]
pub struct BulkStringStrategy;

#[derive(Debug, PartialEq)]
pub struct ArrayStrategy;


impl IdentifierStrategy for SimpleStringStrategy {
    fn apply(&self, value: &[u8]) -> Option<Resp> {

        if value.len() == 0 {
            return None;
        }

        let some_index = CRLFValidator::find(&value)?;

        let start_position = some_index + CRLF.len();

        let next_position = CRLFValidator::find(&value[start_position..])?;

        Some(Resp {
            identifier: SimpleString(SimpleStringStrategy),
            raw: value.to_vec(),
            data: String::from_utf8(value[start_position..start_position+next_position].to_vec()).unwrap(),
            values: vec![],
            end_at: (next_position + CRLF.len()) as i32
        })

    }
}

impl IdentifierStrategy for BulkStringStrategy {


    fn apply(&self, value: &[u8]) -> Option<Resp> {

        if value.len() == 0 {
            return None;
        }


        let length_end_index = CRLFValidator::find(&value)?;

        let length_str = from_utf8(&value[1..length_end_index]).ok()?;

        let length: usize = length_str.parse().ok()?;

        let first_crlf_index_end = length_end_index + CRLF.len();

        let end_position = CRLFValidator::find(&value[first_crlf_index_end..])?;

        let string = String::from_utf8(value[first_crlf_index_end..first_crlf_index_end + length].to_vec()).ok()?;

        // println!("Bulk String: {}: End Position: {}",string, end_position + CRLF.len());

        let end_index = first_crlf_index_end + length + CRLF.len();

        Some (Resp {
            identifier: BulkString(BulkStringStrategy),
            raw: value[..end_index].to_vec(),
            data: string,
            values: vec![],
            end_at: end_index as i32
        })
    }
}

impl IdentifierStrategy for ArrayStrategy {


    fn apply(&self, value: &[u8]) -> Option<Resp> {

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
        Some (Resp {
            identifier: Arrays(ArrayStrategy),
            raw: value.to_vec(),
            data: String::new(),
            values,
            end_at: value_index as i32
        })
    }
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    pub fn simple_strategy() {
        let s = SimpleStringStrategy {};
        assert_eq!(None, s.apply(b"+\r\nhello\r"));
        assert_eq!(None, s.apply(b"+\nhello\r"));
        assert_eq!(None, s.apply(b"+\nhello\r\n"));

        let some_resp = s.apply(b"+\r\nhello\r\n");
        assert!(some_resp.is_some());
        let resp = some_resp.unwrap();
        assert_eq!(resp.data, "hello");

    }


    #[test]
    pub fn bulk_strategy() {
        let s = BulkStringStrategy {};
        let some_resp = s.apply(b"$5\r\nhello\r\n");
        assert!(some_resp.is_some());
        let resp = some_resp.unwrap();
        assert_eq!(resp.data, "hello");
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