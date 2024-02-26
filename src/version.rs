use anyhow::anyhow;
use displaydoc::Display;

#[derive(Clone, Copy, Debug, Display, PartialEq, Hash, Eq)]
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

pub struct StaticVersion<const MAJOR: u16, const MINOR: u16>;
