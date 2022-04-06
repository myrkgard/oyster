//! Serde support for BigUint.
//!
//! We can't use the serde support that is built into the BigUint
//! library because it uses an incompatible base-32 format.
//! We also can't directly implement Serialize and Deserialize on
//! the BigUint type as one can not implement foreign traits on foreign
//! types.
//!
//! Example:
//!
//! ```
//! use num_bigint::BigUint;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     name: String,
//!     #[serde(with = "oyster::biguint_format")]
//!     atoms_in_the_universe: BigUint,
//!     age: u8,
//! }
//! ```

use crate::radix64::{FromRadix64String, ToRadix64String};
use num_bigint::BigUint;
use serde::{Deserialize, Deserializer, Serializer};

/// Serde radix64-serialize support for BigUint.
pub fn serialize<S>(n: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = n.to_radix64_string();
    serializer.serialize_str(&s)
}

/// Serde radix64-deserialize support for BigUint.
pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    <BigUint>::from_radix64_string(&s).map_err(serde::de::Error::custom)
}
