pub mod binary_serializer;
pub mod bincode_serializer;
pub mod version;

pub use binary_serializer::BinarySerializer;
pub type Serializer<const MAJOR: u16, const MINOR: u16> =
    bincode_serializer::BincodeSerializer<MAJOR, MINOR>;
