use std::io::Write;

use crate::{binary_serializer::BinarySerializer, version::Version};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use crate::{binary_serializer::BinarySerializer, version::Version};

    use super::BincodeSerializer;

    mod version_0_1 {
        use super::*;

        pub type Serializer = BincodeSerializer<0u16, 1u16>;

        #[derive(Serialize, Deserialize)]
        pub struct Thing {
            pub field1: u32,
            pub field2: String,
            pub field3: u64,
        }
    }

    mod version_0_2 {
        use super::*;

        pub type Serializer = BincodeSerializer<0u16, 2u16>;

        #[derive(Serialize, Deserialize)]
        pub struct Thing {
            pub field1: u64,
            pub field2: String,
            pub field3: u64,
        }
    }

    #[test]
    fn version_in_version_out() {
        let thing_in = version_0_1::Thing {
            field1: 0,
            field2: "0.1".to_string(),
            field3: 1,
        };

        let buf = version_0_1::Serializer::serialize(&thing_in).unwrap();

        let version_in = version_0_1::Serializer::version();
        let (version_out, _) = Version::deserialize(&buf).unwrap();
        assert_eq!(version_in, version_out);

        let thing_out = version_0_1::Serializer::deserialize::<version_0_1::Thing>(&buf);
        assert!(thing_out.is_ok());
        // With Versioned, we will want these to fail at compile time if Thing is changes between v 0.1 and v 0.2, and pass otherwise.
        let ver_err_out = version_0_2::Serializer::deserialize::<version_0_1::Thing>(&buf);
        assert!(ver_err_out.is_err());
        let type_err_out = version_0_2::Serializer::deserialize::<version_0_2::Thing>(&buf);
        assert!(type_err_out.is_err());
    }
}
