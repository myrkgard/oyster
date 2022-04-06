//! Base-256 positional numeral system.
//!
//! Converts an unsigned integer to a byte vector. Each byte
//! in the vector represents a base-256 digit of the integer.
//! The byte vector is big endian, thus the most significant
//! byte is in position 0. The output is not given in a human
//! readable alphabet but the digits in the byte vector are
//! just plain u8 in the inclusive range of 0 to 255.

/// Trait for number to radix256 bytes.
pub trait ToRadix256Bytes {
    /// Number to radix256 bytes.
    fn to_radix256_bytes(&self) -> Vec<u8>;
}

/// Trait for number from radix256 bytes.
pub trait FromRadix256Bytes<T> {
    /// Number from radix256 bytes. Leading zero bytes are allowed.
    fn from_radix256_bytes(bytes: &[u8]) -> Result<T, String>;
}

use num_bigint::BigUint;

impl ToRadix256Bytes for BigUint {
    fn to_radix256_bytes(&self) -> Vec<u8> {
        self.to_radix_be(256)
    }
}

impl FromRadix256Bytes<BigUint> for BigUint {
    fn from_radix256_bytes(bytes: &[u8]) -> Result<BigUint, String> {
        match BigUint::from_radix_be(bytes, 256) {
            Some(res) => Ok(res),
            None => Err("Invalid input".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_radix256() {
        let n = BigUint::from(2342666u32);
        let bytes = n.to_radix256_bytes();
        println!("{:?}", bytes);
        let n = BigUint::from_radix256_bytes(&bytes).unwrap();
        println!("{}", n.to_string());
    }
}
