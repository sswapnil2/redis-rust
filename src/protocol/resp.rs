use crate::protocol::identifier::{Identifier, IdentifierStrategy};

#[derive(Debug, PartialEq)]
pub struct Resp {
    pub identifier: Identifier,
    pub raw: Vec<u8>,
    pub data: String,
    pub values:  Vec<Resp>,
    pub end_at: i32
}

impl Resp {
    pub fn parse(input: &[u8]) -> Option<Resp> {

        if input.is_empty() {
            return None
        }
        Identifier::from(char::from(input[0])).apply(input)
    }
}




#[cfg(test)]
mod tests {
    use crate::protocol::strategy::BulkStringStrategy;
    use super::*;

    #[test]
    pub fn parse_1() {
         let some_resp = Resp::parse(b"$3\r\nHey\r\n");
        assert!(some_resp.is_some());
        let resp = some_resp.unwrap();
        assert_eq!(resp.identifier, Identifier::BulkString(BulkStringStrategy))

    }


    #[test]
    pub fn test_identifier() {
        assert_eq!(Identifier::BulkString(BulkStringStrategy), Identifier::from(char::from(36u8)));
    }

}