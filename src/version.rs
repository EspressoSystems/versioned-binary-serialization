use anyhow::anyhow;
use core::fmt::Debug;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
/// Type for protocol version number
#[display(fmt = "{major}.{minor}")]
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

pub trait StaticVersionType: Sync + Send + Clone + Copy + Debug + private::Sealed {
    const MAJOR: u16;
    const MINOR: u16;
    const VERSION: Version = Version {
        major: Self::MAJOR,
        minor: Self::MINOR,
    };

    fn version() -> Version {
        Self::VERSION
    }

    fn instance() -> Self;
}

#[derive(Clone, Copy, Display)]
#[display(fmt = "{MAJOR}.{MINOR}")]
pub struct StaticVersion<const MAJOR: u16, const MINOR: u16> {}

impl<const MAJOR: u16, const MINOR: u16> StaticVersionType for StaticVersion<MAJOR, MINOR> {
    const MAJOR: u16 = MAJOR;
    const MINOR: u16 = MINOR;

    fn version() -> Version {
        Version {
            major: Self::MAJOR,
            minor: Self::MINOR,
        }
    }

    fn instance() -> Self {
        Self {}
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compare_version() {
        let v1 = Version { major: 1, minor: 3 };
        let v2 = Version { major: 1, minor: 2 };

        assert!(v1 > v2);

        let v3 = Version { major: 2, minor: 3 };
        let v4 = Version { major: 1, minor: 3 };

        assert!(v3 > v4);

        let v5 = Version { major: 2, minor: 2 };
        let v6 = Version { major: 1, minor: 3 };

        assert!(v5 > v6);
    }

    #[test]
    fn test_version_display() {
        assert_eq!("1.2", Version { major: 1, minor: 2 }.to_string());
    }

    #[test]
    fn test_static_version_display() {
        assert_eq!("1.2", StaticVersion::<1, 2> {}.to_string());
    }

    #[test]
    fn test_static_version_const() {
        assert_eq!(
            StaticVersion::<1, 2>::VERSION,
            StaticVersion::<1, 2>::version()
        );
        assert_eq!(
            Version { major: 1, minor: 2 },
            StaticVersion::<1, 2>::version()
        );
    }
}
