//! Tests for the property-based infrastructure.

use newtype_uuid::{GenericUuid, TypedUuid, TypedUuidKind, TypedUuidTag};
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

/// Ensure that AsRef and conversions to Vec<u8> match Uuid.
#[proptest]
fn prop_as_bytes(typed_uuid: TypedUuid<MyKind>) {
    let typed_uuid_ref: &[u8] = typed_uuid.as_ref();
    let untyped_uuid_ref: &[u8] = typed_uuid.as_untyped_uuid().as_ref();
    assert_eq!(typed_uuid_ref, untyped_uuid_ref);

    let typed_uuid_vec: Vec<u8> = typed_uuid.into();
    let untyped_uuid_vec: Vec<u8> = typed_uuid.into_untyped_uuid().into();
    assert_eq!(typed_uuid_vec, untyped_uuid_vec);
}
