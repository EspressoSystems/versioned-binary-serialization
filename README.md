# vbs

Provides the following:
- a trait `StaticVersionType` for constraining to a version (major.minor) at compile-time.
  - sealed to a struct `StaticVersion<const MAJOR: u16, const MINOR: u16>`.
  - for the purposes of version enforcement, patch versions are not treated as a type change.
- a struct `Version` for runtime operations against a version, without requiring dyn
  - used to \[de\]serialize a version in a strictly defined and immutable form, so that updates to the serialization format iteself can be a version controlled property.
  - does not include patch level, prerelease identifiers, or build metadata; this is not intended to be a general purpose `semver` crate.
- a trait `BinarySerializer`, an adaptor that can be implemented around any data format that adapts the `serde` data model.
  - by default, serializes a version prefix before each top-level serialization, and verifies version compatibility when deserializing the serialized message.
  - supports unversioned `[de]serialize_no_version` operations, which, by default, should simply be the same as calling `type.serialize(serializer)` for the embedded serializer.
- implementations against existing data formats
  - currently, only [bincode](https://crates.io/crates/bincode)