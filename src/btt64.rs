//! Binary-to-Text module.

use crate::common::{char_to_idx, IDX_TABLE};
use serde::{Deserialize, Deserializer, Serializer};

/// Conversion to string.
pub trait ToBtt64String {
    /// Conversion to string.
    fn to_btt64_string(&self) -> String;
}

/// Conversion from string.
pub trait FromBtt64String<T> {
    /// Conversion from string. May fail with error.
    fn from_btt64_string(s: &str) -> Result<T, String>;
}

impl ToBtt64String for Vec<u8> {
    fn to_btt64_string(&self) -> String {
        let blk_len: usize = 3;
        let blk_count = self.len() / blk_len;
        let rest_count = self.len() % blk_len;
        let mut msg = String::new();
        for i in 0..blk_count {
            let s = encode_block_vec_u8(self, i * blk_len, blk_len);
            msg.push_str(&s);
        }
        if rest_count > 0 {
            let s = encode_block_vec_u8(self, blk_count * blk_len, rest_count);
            msg.push_str(&s);
        }
        msg
    }
}

fn encode_block_vec_u8(data: &[u8], offset: usize, byte_count: usize) -> String {
    debug_assert!(byte_count > 0 && byte_count <= 3);

    let mut idx = Vec::with_capacity(byte_count + 1);

    idx.push(0);
    for i in 0..byte_count {
        idx[0] = idx[0] | ((data[offset + i] & 0b11000000) >> ((i + 1) * 2));
    }

    for i in 0..byte_count {
        idx.push(data[offset + i] & 0b00111111);
    }

    let mut msg = String::with_capacity(idx.len());
    for i in 0..idx.len() {
        msg.push(IDX_TABLE[idx[i] as usize] as char);
    }
    msg
}

impl FromBtt64String<Vec<u8>> for Vec<u8> {
    fn from_btt64_string(s: &str) -> Result<Vec<u8>, String> {
        let chars = s.as_bytes().to_owned();
        let blk_len: usize = 4;
        let blk_count = chars.len() / blk_len;
        let rest_count = chars.len() % blk_len;
        let mut data: Vec<u8> = Vec::new();
        for i in 0..blk_count {
            let tmp = decode_block_vec_u8(&chars, i * blk_len, blk_len);
            if tmp.is_ok() {
                data.append(&mut tmp.unwrap());
            } else {
                return Err("Invalid input".to_string());
            }
        }
        if rest_count > 0 {
            let tmp = decode_block_vec_u8(&chars, blk_count * blk_len, rest_count);
            if tmp.is_ok() {
                data.append(&mut tmp.unwrap());
            } else {
                return Err("Invalid input".to_string());
            }
        }
        Ok(data)
    }
}

fn decode_block_vec_u8(
    chars: &Vec<u8>,
    offset: usize,
    char_count: usize,
) -> Result<Vec<u8>, String> {
    debug_assert!(char_count > 1 && char_count <= 4);

    let mut idx: Vec<u8> = Vec::with_capacity(char_count);
    for i in 0..char_count {
        let tmp = char_to_idx(chars[offset + i]);
        if tmp.is_ok() {
            idx.push(tmp.unwrap());
        } else {
            return Err("Invalid input".to_string());
        }
    }

    // if it's not a full block, make sure first idx is valid
    {
        let mask: u8 = match char_count {
            2 => 0b00110000,
            3 => 0b00111100,
            4 => 0b00111111,
            _ => panic!("unreachable"),
        };
        if idx[0] & mask != idx[0] {
            return Err("Invalid input".to_string());
        }
    }

    let mut data: Vec<u8> = Vec::with_capacity(char_count - 1);
    for i in 0..char_count - 1 {
        let d = ((idx[0] << ((i + 1) * 2)) & 0b11000000) | idx[i + 1];
        data.push(d);
    }

    Ok(data)
}

impl ToBtt64String for String {
    fn to_btt64_string(&self) -> String {
        self.as_bytes().to_vec().to_btt64_string()
    }
}

impl FromBtt64String<String> for String {
    fn from_btt64_string(s: &str) -> Result<String, String> {
        let decoded_bytes = <Vec<u8>>::from_btt64_string(s)?;
        match String::from_utf8(decoded_bytes) {
            Ok(res) => Ok(res),
            Err(_) => Err("Invalid input".to_string()),
        }
    }
}

impl ToBtt64String for &str {
    fn to_btt64_string(&self) -> String {
        self.as_bytes().to_vec().to_btt64_string()
    }
}

/// Serde serialize support for bytes (Vec<u8>).
pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = bytes.to_btt64_string();
    serializer.serialize_str(&s)
}

/// Serde deserialize support for bytes (Vec<u8>).
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    <Vec<u8>>::from_btt64_string(&s).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {

    fn get_random_usize(max: usize) -> usize {
        use rand::Rng;
        rand::thread_rng().gen_range(0..max)
    }

    fn get_random_data(length: usize) -> Vec<u8> {
        use rand::Rng;
        let mut data: Vec<u8> = Vec::with_capacity(length);
        for _ in 0..length {
            let random_val: u8 = rand::thread_rng().gen();
            data.push(random_val);
        }
        data
    }

    #[test]
    fn t_encode_decode() {
        use super::*;
        for _ in 0..100 {
            let data_len = get_random_usize(1000);
            let data = get_random_data(data_len);
            let msg = data.to_btt64_string();
            let received = Vec::from_btt64_string(&msg).unwrap();
            assert_eq!(data, received);
        }
    }

    #[test]
    fn t_string_encode_decode() {
        use super::*;
        let s = "Hello world!".to_string();
        let s_encoded = s.to_btt64_string();
        let s_decoded = <String>::from_btt64_string(&s_encoded).unwrap();
        assert_eq!(s_decoded, s);
    }

    #[test]
    fn t_ref_str_encode_decode() {
        use super::*;
        let s = "Hello world!";
        let s_encoded = s.to_btt64_string();
        let s_decoded = <String>::from_btt64_string(&s_encoded).unwrap();
        assert_eq!(s_decoded, s);
    }
}
