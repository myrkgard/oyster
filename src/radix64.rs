//! Base-64 positional numeral system.

use crate::common::{char_to_idx, IDX_TABLE};

/// Trait for number to radix64.
pub trait ToRadix64String {
    /// Number to radix64 string.
    fn to_radix64_string(&self) -> String;
}

/// Trait for number from radix64 string.
pub trait FromRadix64String<T> {
    /// Number from radix64 string.
    fn from_radix64_string(s: &str) -> Result<T, String>;
}

use num_bigint::BigUint;

impl ToRadix64String for BigUint {
    fn to_radix64_string(&self) -> String {
        use num_integer::Integer;
        use num_traits::cast::ToPrimitive;
        use num_traits::Zero;

        let zero = BigUint::zero();
        let sixty_four = BigUint::from(64u8);

        let mut chars: Vec<u8> = Vec::new();
        let mut n = self.clone();
        loop {
            let (q, r) = n.div_mod_floor(&sixty_four);
            chars.push(IDX_TABLE[r.to_u8().unwrap() as usize]);
            if q == zero {
                break;
            } else {
                n = q;
            }
        }

        chars.reverse();
        String::from_utf8(chars).unwrap()
    }
}

impl FromRadix64String<BigUint> for BigUint {
    /// For implementation reasons, returns error if string length -1 is greater than u32::MAX
    fn from_radix64_string(s: &str) -> Result<BigUint, String> {
        use num_traits::Zero;

        let mut chars = s.as_bytes().to_owned();
        chars.reverse();

        if chars.len() == 0 || chars.len() - 1 > (u32::MAX as usize) {
            return Err("Invalid input".to_string());
        }

        let mut res = BigUint::zero();
        let sixty_four = BigUint::from(64u8);
        for i in 0..chars.len() {
            let tmp = char_to_idx(chars[i]);
            if tmp.is_ok() {
                let idx = tmp.unwrap();
                res = res + BigUint::from(idx) * sixty_four.pow(i as u32);
            } else {
                return Err("Invalid input".to_string());
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_random_biguint(bits: u64) -> BigUint {
        use num_bigint::RandomBits;
        use rand::Rng;
        rand::thread_rng().sample(RandomBits::new(bits))
    }

    #[test]
    fn t_to_radix64_and_back_random() {
        for _ in 0..100 {
            let n = get_random_biguint(256);
            let s = n.to_radix64_string();
            let n2 = BigUint::from_radix64_string(&s).unwrap();
            assert_eq!(n, n2);
        }
    }

    #[test]
    fn t_to_radix64_and_back_systematically() {
        use num_traits::{One, Zero};
        let mut n = BigUint::zero();
        let one = BigUint::one();
        for _ in 0..1000000u64 {
            n = &n + &one;
            let s = n.to_radix64_string();
            let n2 = BigUint::from_radix64_string(&s).unwrap();
            assert_eq!(n, n2);
        }
    }
    #[test]
    fn t_to_radix64_and_back() {
        let n = BigUint::from(2342666u32);
        let s = n.to_radix64_string();
        println!("{}", s);
    }
}
