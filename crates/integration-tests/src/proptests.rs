//! Tests for the property-based infrastructure.

use newtype_uuid::{TypedUuid, TypedUuidKind, TypedUuidTag};
use test_strategy::proptest;
use uuid::Version;

#[derive(Debug)]
enum MyKind {}

impl TypedUuidKind for MyKind {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("my_kind");
        TAG
    }
}

/// Ensure that generated UUIDs are always v4.
#[proptest]
fn prop_is_valid_v4(uuid: TypedUuid<MyKind>) {
    assert_eq!(uuid.get_version(), Some(Version::Random));
}
