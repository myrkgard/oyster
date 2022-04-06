//! Provides common stuff for btt64 and radix64.

pub static IDX_TABLE: &'static [u8; 64] = &[
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L',
    b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'-', b'_',
];

pub fn char_to_idx(d: u8) -> Result<u8, String> {
    for i in 0..IDX_TABLE.len() {
        if IDX_TABLE[i] == d {
            return Ok(i as u8);
        }
    }
    Err("Invalid input".to_string())
}
