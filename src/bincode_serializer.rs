use std::io::Write;

use crate::{binary_serializer::BinarySerializer, version::Version, versioned::Versioned};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub struct BincodeSerializer<const VER_MAJOR: u16, const VER_MINOR: u16>;
impl<const VER_MAJOR: u16, const VER_MINOR: u16> BinarySerializer
    for BincodeSerializer<VER_MAJOR, VER_MINOR>
{
    const MAJOR: u16 = VER_MAJOR;
    const MINOR: u16 = VER_MINOR;
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

// Testing; will use to replace BincodeSerializer after applying `Versioned` to existing serialized types
pub struct VersionChecker<const VER_MAJOR: u16, const VER_MINOR: u16, TYPE: ?Sized + Versioned> {
    _phantom: std::marker::PhantomData<TYPE>,
}
impl<const VER_MAJOR: u16, const VER_MINOR: u16, TYPE: ?Sized + Versioned>
    VersionChecker<VER_MAJOR, VER_MINOR, TYPE>
{
    pub const VERSION_MISMATCH: () = if VER_MAJOR < TYPE::MIN_MAJOR
        || VER_MAJOR > TYPE::MAX_MAJOR
        || (VER_MAJOR == TYPE::MIN_MAJOR && VER_MINOR < TYPE::MIN_MINOR)
        || (VER_MAJOR == TYPE::MAX_MAJOR && VER_MINOR > TYPE::MAX_MINOR)
    {
        panic!("unsupported type for version")
    };
}

pub trait VersionedBinarySerializer {
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
        T: Serialize + Versioned;

    fn deserialize_no_version<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a> + Versioned;

    fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize + Versioned;

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a> + Versioned;
}

pub struct VersionedBincodeSerializer<const VER_MAJOR: u16, const VER_MINOR: u16>;
impl<const VER_MAJOR: u16, const VER_MINOR: u16> VersionedBinarySerializer
    for VersionedBincodeSerializer<VER_MAJOR, VER_MINOR>
{
    const MAJOR: u16 = VER_MAJOR;
    const MINOR: u16 = VER_MINOR;

    fn serialize_no_version<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize + Versioned,
    {
        #[allow(clippy::let_unit_value)]
        let _ = VersionChecker::<VER_MAJOR, VER_MINOR, T>::VERSION_MISMATCH;

        Ok(bincode::serialize(value)?)
    }

    fn deserialize_no_version<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a> + Versioned,
    {
        #[allow(clippy::let_unit_value)]
        let _ = VersionChecker::<VER_MAJOR, VER_MINOR, T>::VERSION_MISMATCH;

        Ok(bincode::deserialize(bytes)?)
    }

    fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize + Versioned,
    {
        #[allow(clippy::let_unit_value)]
        let _ = VersionChecker::<VER_MAJOR, VER_MINOR, T>::VERSION_MISMATCH;

        let mut vec = Self::version().serialize();
        bincode::serialize_into(vec.by_ref(), value)?;
        Ok(vec)
    }

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a> + Versioned,
    {
        #[allow(clippy::let_unit_value)]
        let _ = VersionChecker::<VER_MAJOR, VER_MINOR, T>::VERSION_MISMATCH;

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

    use super::{BincodeSerializer, VersionedBinarySerializer, VersionedBincodeSerializer};

    mod version_0_1 {
        use crate::versioned::Versioned;

        use super::*;

        pub type Serializer = BincodeSerializer<0u16, 1u16>;
        pub type VSerializer = VersionedBincodeSerializer<0u16, 1u16>;

        #[derive(Serialize, Deserialize)]
        pub struct Thing {
            pub field1: u32,
            pub field2: String,
            pub field3: u64,
        }

        impl Versioned for Thing {
            const MIN_MAJOR: u16 = 0;
            const MIN_MINOR: u16 = 1;
            const MAX_MAJOR: u16 = 0;
            const MAX_MINOR: u16 = 2;
        }
    }

    mod version_0_2 {
        use super::*;

        pub type Serializer = BincodeSerializer<0u16, 2u16>;
        pub type VSerializer = VersionedBincodeSerializer<0u16, 2u16>;

        pub type Thing = version_0_1::Thing;
    }

    mod version_0_3 {
        use crate::versioned::Versioned;

        use super::*;

        pub type Serializer = BincodeSerializer<0u16, 3u16>;
        pub type VSerializer = VersionedBincodeSerializer<0u16, 3u16>;

        #[derive(Serialize, Deserialize)]
        pub struct Thing {
            pub field1: u64,
            pub field2: String,
            pub field3: u64,
        }
        impl Versioned for Thing {
            const MIN_MAJOR: u16 = 0;
            const MIN_MINOR: u16 = 3;
            const MAX_MAJOR: u16 = 0;
            const MAX_MINOR: u16 = 3;
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
        // this behavior is not what we want...
        let ver_err_out = version_0_2::Serializer::deserialize::<version_0_1::Thing>(&buf);
        assert!(ver_err_out.is_err());
        let ver_err_out = version_0_3::Serializer::deserialize::<version_0_1::Thing>(&buf);
        assert!(ver_err_out.is_err());
        let type_err_out = version_0_3::Serializer::deserialize::<version_0_3::Thing>(&buf);
        assert!(type_err_out.is_err());
    }

    #[test]
    fn version_in_version_out_versioned() {
        let thing_in = version_0_1::Thing {
            field1: 0,
            field2: "0.1".to_string(),
            field3: 1,
        };

        let buf = version_0_1::VSerializer::serialize(&thing_in).unwrap();

        let version_in = version_0_1::VSerializer::version();
        let (version_out, _) = Version::deserialize(&buf).unwrap();
        assert_eq!(version_in, version_out);

        let thing_out = version_0_1::VSerializer::deserialize::<version_0_1::Thing>(&buf);
        assert!(thing_out.is_ok());
        let ver_err_out = version_0_2::VSerializer::deserialize::<version_0_1::Thing>(&buf);
        assert!(ver_err_out.is_err());
        let ver_err_out = version_0_2::VSerializer::deserialize::<version_0_2::Thing>(&buf);
        assert!(ver_err_out.is_err());
        // the following test fails at compile time, as desired.
        // let ver_err_out = version_0_3::VSerializer::deserialize::<version_0_1::Thing>(&buf);
        // assert!(ver_err_out.is_err());
        let type_err_out = version_0_3::VSerializer::deserialize::<version_0_3::Thing>(&buf);
        assert!(type_err_out.is_err());
    }
}
