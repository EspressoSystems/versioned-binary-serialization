pub mod binary_serializer;
pub mod bincode_serializer;
pub mod error;
pub mod version;

pub type BinarySerializer<const MAJOR: u16, const MINOR: u16> =
    bincode_serializer::BincodeSerializer<MAJOR, MINOR>;
