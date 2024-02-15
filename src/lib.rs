pub mod binary_serializer;
pub mod bincode_serializer;
pub mod version;
pub mod versioned;

pub type BinarySerializer<const MAJOR: u16, const MINOR: u16> =
    bincode_serializer::BincodeSerializer<MAJOR, MINOR>;
