pub mod versioned_bincode_serializer;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::result::Result;
pub trait BinarySerializer<const VERSION: u32> {
    type Error: Error;

    // TODO: `Versioned` trait

    fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: Serialize; // + Versioned

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, Self::Error>
    where
        T: Deserialize<'a>; // + Versioned

    fn serialize_oneoff<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: Serialize; // + Versioned;

    fn deserialize_oneoff<'a, T>(bytes: &'a [u8]) -> Result<T, Self::Error>
    where
        T: Deserialize<'a>; // + Versioned;
}

#[cfg(test)]
mod tests {
    #[test]
    fn versioned_call() {}
}
