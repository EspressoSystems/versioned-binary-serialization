use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::{binary_serializer::BinarySerializer, version::Version};
use anyhow::{anyhow, Result};

pub struct BincodeSerializer<const VER_MAJOR: u16, const VER_MINOR: u16>;

impl<const VER_MAJOR: u16, const VER_MINOR: u16> BinarySerializer<VER_MAJOR, VER_MINOR>
    for BincodeSerializer<VER_MAJOR, VER_MINOR>
{
    fn serialize_no_version<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        Ok(bincode::serialize(value)?)
    }

    fn deserialize_no_version<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        Ok(bincode::deserialize(bytes)?)
    }

    fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        let mut vec = Self::version().serialize();
        bincode::serialize_into(vec.by_ref(), value)?;
        Ok(vec)
    }

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        let (ver, rest) = Version::deserialize(bytes)?;
        if ver.major != VER_MAJOR || ver.minor != VER_MINOR {
            return Err(anyhow!(
                "Version Mismatch! Expected {}, got {}",
                ver,
                Self::version()
            ));
        }
        Ok(bincode::deserialize(rest)?)
    }
}
