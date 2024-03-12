use anyhow::anyhow;
use core::fmt::Debug;
use displaydoc::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
/// Type for protocol version number
pub struct Version {
    /// major version number
    pub major: u16,
    /// minor version number
    pub minor: u16,
}

impl Version {
    pub fn serialize(&self) -> Vec<u8> {
        let mut ver = self.major.to_le_bytes().to_vec();
        ver.append(&mut self.minor.to_le_bytes().to_vec());
        ver
    }

    pub fn deserialize(bytes: &[u8]) -> Result<(Self, &[u8]), anyhow::Error> {
        if bytes.len() < 4 {
            return Err(anyhow!("deserializing from a buffer too small to contain a version. Minimum size is 4 bytes, got {}", bytes.len()));
        }
        let (ver_major, rest) = bytes.split_at(std::mem::size_of::<u16>());
        let (ver_minor, rest) = rest.split_at(std::mem::size_of::<u16>());

        let ver = Version {
            major: u16::from_le_bytes(ver_major.try_into()?),
            minor: u16::from_le_bytes(ver_minor.try_into()?),
        };
        Ok((ver, rest))
    }
}

pub trait StaticVersionType: Clone + Copy + Debug + private::Sealed {
    const MAJOR: u16;
    const MINOR: u16;

    fn version() -> Version {
        Version {
            major: Self::MAJOR,
            minor: Self::MINOR,
        }
    }
}

#[derive(Clone, Copy, Display)]
pub struct StaticVersion<const MAJOR: u16, const MINOR: u16>;

impl<const MAJOR: u16, const MINOR: u16> StaticVersionType for StaticVersion<MAJOR, MINOR> {
    const MAJOR: u16 = MAJOR;
    const MINOR: u16 = MINOR;

    fn version() -> Version {
        Version {
            major: Self::MAJOR,
            minor: Self::MINOR,
        }
    }
}

impl<const MAJOR: u16, const MINOR: u16> Debug for StaticVersion<MAJOR, MINOR> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticVersion<MAJOR, MINOR>")
            .field("MAJOR", &Self::MAJOR)
            .field("MINOR", &Self::MINOR)
            .finish()
    }
}

mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl<const MAJOR: u16, const MINOR: u16> Sealed for super::StaticVersion<MAJOR, MINOR> {}
}
