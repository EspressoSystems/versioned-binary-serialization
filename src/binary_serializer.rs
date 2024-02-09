use crate::version::Version;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::result::Result;

pub trait BinarySerializer<const VER_MAJOR: u16, const VER_MINOR: u16> {
    type Error: Error;
    type NativeError: Error;

    fn version() -> Version {
        Version {
            major: VER_MAJOR,
            minor: VER_MINOR,
        }
    }

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
