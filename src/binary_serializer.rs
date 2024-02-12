use crate::version::Version;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub trait BinarySerializer<const VER_MAJOR: u16, const VER_MINOR: u16> {
    fn version() -> Version {
        Version {
            major: VER_MAJOR,
            minor: VER_MINOR,
        }
    }

    // TODO: `Versioned` trait

    fn serialize_no_version<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize; // + Versioned

    fn deserialize_no_version<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>; // + Versioned

    fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize; // + Versioned;

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>; // + Versioned;
}
