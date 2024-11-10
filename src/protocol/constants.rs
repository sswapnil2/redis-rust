
pub const CR: u8 = 13;
pub const LF: u8 = 10;

pub const CRLF: [u8;2] = [CR, LF];
pub struct CRLFValidator;

impl CRLFValidator {
    pub fn find(arr: &[u8]) -> Option<usize> {
        if arr.len() < 2 {
            return None;
        }
        arr.windows(2).position(|a| a == CRLF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_1() {
        assert!(CRLFValidator::find(&[CR, LF]).is_some());
    }

}