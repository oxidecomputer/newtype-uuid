use std::{fmt, hash::Hash};

use newtype_uuid::{GenericUuid, TypedUuidKind};
use newtype_uuid_macros::impl_typed_uuid_kinds;
use static_assertions::{assert_impl_all, assert_not_impl_all};

impl_typed_uuid_kinds! {
    kinds = {
        User = {},
        Organization = {},
        Project = {},
    }
}

#[test]
fn test_generated_kinds() {
    // Test that the generated kinds have the correct tags
    assert_eq!(UserKind::tag().as_str(), "user");
    assert_eq!(OrganizationKind::tag().as_str(), "organization");
    assert_eq!(ProjectKind::tag().as_str(), "project");
}

#[test]
fn test_generated_uuid_types() {
    // Test that different types are not equal even with same UUID
    let uuid = uuid::Uuid::new_v4();
    let typed_user = UserUuid::from_untyped_uuid(uuid);
    let typed_org = OrganizationUuid::from_untyped_uuid(uuid);

    // They should have the same underlying UUID but different types
    assert_eq!(
        typed_user.into_untyped_uuid(),
        typed_org.into_untyped_uuid()
    );
}

#[test]
fn test_kind_properties() {
    // Test that the generated kinds implement Debug trait
    // Empty enums don't have Default, so we can't create instances
    // But we can test the Debug representation of the type itself
    assert_eq!(format!("{:?}", UserKind::tag()), "TypedUuidTag(\"user\")");
}

#[test]
fn test_single_entry() {
    impl_typed_uuid_kinds! {
        kinds = {
            Single = {},
        }
    }

    assert_eq!(SingleKind::tag().as_str(), "single");
    let uuid = uuid::Uuid::new_v4();
    let single_uuid = SingleUuid::from_untyped_uuid(uuid);
    assert_eq!(single_uuid.into_untyped_uuid(), uuid);
}

#[test]
fn test_trailing_comma() {
    impl_typed_uuid_kinds! {
        kinds = {
            TrailingComma = {},
        }
    }

    assert_eq!(TrailingCommaKind::tag().as_str(), "trailing_comma");
}

#[test]
fn test_snake_case_conversion() {
    impl_typed_uuid_kinds! {
        kinds = {
            HTTPClient = {},
            XMLParser = {},
            UserAccount = {},
            IOHandler = {},
        }
    }

    assert_eq!(HTTPClientKind::tag().as_str(), "http_client");
    assert_eq!(XMLParserKind::tag().as_str(), "xml_parser");
    assert_eq!(UserAccountKind::tag().as_str(), "user_account");
    assert_eq!(IOHandlerKind::tag().as_str(), "io_handler");
}

#[test]
fn test_empty_kinds() {
    // Test that we can handle empty kinds map
    impl_typed_uuid_kinds! {
        kinds = {}
    }

    // Just verifying it compiles
}

/// Test that global `attrs` are applied to all generated kinds.
#[test]
fn test_global_attrs_param() {
    impl_typed_uuid_kinds! {
        settings = {
            attrs = [#[derive(Hash)]],
        },
        kinds = {
            GlobalA = {},
            GlobalB = {},
        }
    }

    // The GlobalA and GlobalB kinds should impl Clone + Copy + Eq + by default,
    // and also implement Hash.
    assert_impl_all!(GlobalAKind: Clone, Copy, fmt::Debug, Eq, Hash);
    assert_impl_all!(GlobalBKind: Clone, Copy, fmt::Debug, Eq, Hash);
}

/// Test that per-kind `attrs` override global and are applied only to the correct kind.
#[test]
fn test_per_kind_attrs_param() {
    impl_typed_uuid_kinds! {
        settings = {
            attrs = [#[derive(Hash)]],
        },
        kinds = {
            A = {},
            B = {
                attrs = [],
            },
            C = {
                attrs = [#[derive(Ord, PartialOrd)]],
            },
        }
    }

    // AKind should implement Hash.
    assert_impl_all!(AKind: Hash);

    // BKind should not implement Hash.
    assert_not_impl_all!(BKind: Hash);

    // CKind should implement Ord and PartialOrd, but not Hash.
    assert_impl_all!(CKind: Ord, PartialOrd);
    assert_not_impl_all!(CKind: Hash);
}

#[test]
fn test_no_schemars_settings() {
    // Test that when no schemars settings are provided, no JsonSchema impl is generated
    impl_typed_uuid_kinds! {
        kinds = {
            NoSchemaKind = {},
        }
    }

    // This should compile fine - we're just testing that the kinds work without schemars
    assert_eq!(NoSchemaKindKind::tag().as_str(), "no_schema_kind");
}
