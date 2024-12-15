use const_decoder2::decode_base64 as b64;

pub mod der;
pub use der::*;

pub mod pem;
pub use pem::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
