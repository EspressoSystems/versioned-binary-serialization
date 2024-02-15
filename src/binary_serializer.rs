use crate::version::Version;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub trait BinarySerializer {
    const MAJOR: u16;
    const MINOR: u16;

    fn version() -> Version {
        Version {
            major: Self::MAJOR,
            minor: Self::MINOR,
        }
    }

    // TODO: `Versioned` trait

    fn serialize_no_version<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize;

    fn deserialize_no_version<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>;

    fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize;

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>;
}
