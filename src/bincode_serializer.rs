use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::error::{ExternSerializerSnafu, VersionHeaderSnafu};
use crate::{binary_serializer::BinarySerializer, version::Version};

pub struct BincodeSerializer<const VER_MAJOR: u16, const VER_MINOR: u16>;

impl<const VER_MAJOR: u16, const VER_MINOR: u16> BinarySerializer<VER_MAJOR, VER_MINOR>
    for BincodeSerializer<VER_MAJOR, VER_MINOR>
{
    type Error = crate::error::Error<bincode::Error>;
    type NativeError = bincode::Error;

    fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: Serialize,
    {
        bincode::serialize(value).context(ExternSerializerSnafu)
    }

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, Self::Error>
    where
        T: Deserialize<'a>,
    {
        bincode::deserialize(bytes).context(ExternSerializerSnafu)
    }

    fn serialize_oneoff<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: Serialize,
    {
        bincode::serialize(value)
            .map(|mut vec| {
                let mut ver = Self::version().serialize();
                ver.append(&mut vec);
                ver
            })
            .context(ExternSerializerSnafu)
    }

    fn deserialize_oneoff<'a, T>(bytes: &'a [u8]) -> Result<T, Self::Error>
    where
        T: Deserialize<'a>,
    {
        let (ver, rest) = Version::deserialize(bytes).context(VersionHeaderSnafu)?;
        if ver.major != VER_MAJOR || ver.minor != VER_MINOR {
            return Err(Self::Error::VersionMismatch {
                version: ver,
                wanted: Self::version(),
            });
        }
        bincode::deserialize(rest).context(ExternSerializerSnafu)
    }
}
