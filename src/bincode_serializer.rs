use std::{io::Write, marker::PhantomData};

use crate::{
    binary_serializer::BinarySerializer,
    version::{StaticVersionType, Version},
    versioned::Versioned,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub struct BincodeSerializer<VER: StaticVersionType>(PhantomData<VER>);
impl<VER: StaticVersionType> BinarySerializer for BincodeSerializer<VER> {
    const MAJOR: u16 = VER::MAJOR;
    const MINOR: u16 = VER::MINOR;
    fn serialize_no_version<T: ?Sized + Serialize>(value: &T) -> Result<Vec<u8>> {
        Ok(bincode::serialize(value)?)
    }

    fn deserialize_no_version<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        Ok(bincode::deserialize(bytes)?)
    }

    fn serialize<T: ?Sized + Serialize>(value: &T) -> Result<Vec<u8>> {
        let mut vec = Self::version().serialize();
        bincode::serialize_into(vec.by_ref(), value)?;
        Ok(vec)
    }

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        let (ver, rest) = Version::deserialize(bytes)?;
        if ver.major != VER::MAJOR || ver.minor != VER::MINOR {
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
pub struct VersionChecker<VER: StaticVersionType, TYPE: ?Sized + Versioned> {
    _phantom_ver: std::marker::PhantomData<VER>,
    _phantom_type: std::marker::PhantomData<TYPE>,
}
impl<VER: StaticVersionType, TYPE: ?Sized + Versioned> VersionChecker<VER, TYPE> {
    pub const VERSION_MISMATCH: () = if VER::MAJOR < TYPE::MIN_MAJOR
        || VER::MAJOR > TYPE::MAX_MAJOR
        || (VER::MAJOR == TYPE::MIN_MAJOR && VER::MINOR < TYPE::MIN_MINOR)
        || (VER::MAJOR == TYPE::MAX_MAJOR && VER::MINOR > TYPE::MAX_MINOR)
    {
        panic!("unsupported type for version")
    };
}

pub trait VersionedBinarySerializer {
    const MAJOR: u16;
    const MINOR: u16;
    type StaticVersion: StaticVersionType;

    fn version() -> Version {
        Version {
            major: Self::MAJOR,
            minor: Self::MINOR,
        }
    }

    // TODO: `Versioned` trait

    fn serialize_no_version<T: ?Sized + Serialize + Versioned>(value: &T) -> Result<Vec<u8>>;

    fn deserialize_no_version<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a> + Versioned;

    fn serialize<T: ?Sized + Serialize + Versioned>(value: &T) -> Result<Vec<u8>>;

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a> + Versioned;
}

pub struct VersionedBincodeSerializer<VER: StaticVersionType>(PhantomData<VER>);
impl<VER: StaticVersionType> VersionedBinarySerializer for VersionedBincodeSerializer<VER> {
    const MAJOR: u16 = VER::MAJOR;
    const MINOR: u16 = VER::MINOR;
    type StaticVersion = VER;

    fn serialize_no_version<T: ?Sized + Serialize + Versioned>(value: &T) -> Result<Vec<u8>> {
        #[allow(clippy::let_unit_value)]
        let _ = VersionChecker::<VER, T>::VERSION_MISMATCH;

        Ok(bincode::serialize(value)?)
    }

    fn deserialize_no_version<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a> + Versioned,
    {
        #[allow(clippy::let_unit_value)]
        let _ = VersionChecker::<VER, T>::VERSION_MISMATCH;

        Ok(bincode::deserialize(bytes)?)
    }

    fn serialize<T: ?Sized + Serialize + Versioned>(value: &T) -> Result<Vec<u8>> {
        #[allow(clippy::let_unit_value)]
        let _ = VersionChecker::<VER, T>::VERSION_MISMATCH;

        let mut vec = Self::version().serialize();
        bincode::serialize_into(vec.by_ref(), value)?;
        Ok(vec)
    }

    fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a> + Versioned,
    {
        #[allow(clippy::let_unit_value)]
        let _ = VersionChecker::<VER, T>::VERSION_MISMATCH;

        let (ver, rest) = Version::deserialize(bytes)?;
        if ver.major != VER::MAJOR || ver.minor != VER::MINOR {
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

    use crate::{
        binary_serializer::BinarySerializer,
        version::{StaticVersion, Version},
        versioned::Versioned,
    };

    use super::{BincodeSerializer, VersionedBinarySerializer, VersionedBincodeSerializer};

    mod version_0_1 {
        use super::*;

        pub type Serializer = BincodeSerializer<StaticVersion<0u16, 1u16>>;
        pub type VSerializer = VersionedBincodeSerializer<StaticVersion<0u16, 1u16>>;

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

        pub type Serializer = BincodeSerializer<StaticVersion<0u16, 2u16>>;
        pub type VSerializer = VersionedBincodeSerializer<StaticVersion<0u16, 2u16>>;

        pub type Thing = version_0_1::Thing;
    }

    mod version_0_3 {
        use super::*;

        pub type Serializer = BincodeSerializer<StaticVersion<0u16, 3u16>>;
        pub type VSerializer = VersionedBincodeSerializer<StaticVersion<0u16, 3u16>>;

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
