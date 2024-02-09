use std::array::TryFromSliceError;

use displaydoc::Display;
use serde::{Deserialize, Serialize};

use crate::BinarySerializer;

#[derive(Debug, Display)]
pub enum BincodeSerializerError {
    /// Error from try_from
    VersionError(TryFromSliceError),
    /// Error from bincode
    BincodeError(bincode::Error),
}

impl std::error::Error for BincodeSerializerError {}

pub struct BincodeSerializer<const VERSION: u32>;

impl<const VERSION: u32> BinarySerializer<VERSION>
    for BincodeSerializer<VERSION>
{
    type Error = BincodeSerializerError;

    fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: Serialize,
    {
        bincode::serialize(value).map_err(|e| BincodeSerializerError::BincodeError(e))
    }

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, Self::Error>
    where
        T: Deserialize<'a>,
    {
        bincode::deserialize(bytes).map_err(|e| BincodeSerializerError::BincodeError(e))
    }

    fn serialize_oneoff<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: Serialize,
    {
        bincode::serialize(value).map(|mut vec| {
            let mut ver = VERSION.to_le_bytes().to_vec();
            ver.append(&mut vec);
            ver
        }).map_err(|e| BincodeSerializerError::BincodeError(e))
    }

    fn deserialize_oneoff<'a, T>(bytes: &'a [u8]) -> Result<T, Self::Error>
    where
        T: Deserialize<'a>,
    {
        let (ver_bytes, rest) = bytes.split_at(std::mem::size_of::<u32>());
        let _ver = u32::from_le_bytes(ver_bytes.try_into().map_err(|e| BincodeSerializerError::VersionError(e))?);
        // TODO: check ver against compatible range for VERSION
        bincode::deserialize(rest).map_err(|e| BincodeSerializerError::BincodeError(e))
    }
}
