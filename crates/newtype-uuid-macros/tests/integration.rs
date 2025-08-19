use newtype_uuid::TypedUuidKind;
use newtype_uuid_macros::impl_typed_uuid_kinds;
use static_assertions::{assert_impl_all, assert_not_impl_all};
use std::{fmt, hash::Hash};

impl_typed_uuid_kinds! {
    kinds = {
        User = {},
        Organization = {},
        Project = {},
    }
}

#[test]
fn test_generated_kinds() {
    // Test that the generated kinds have the correct tags.
    assert_eq!(UserKind::tag().as_str(), "user");
    assert_eq!(OrganizationKind::tag().as_str(), "organization");
    assert_eq!(ProjectKind::tag().as_str(), "project");
}

#[test]
fn test_single_entry() {
    impl_typed_uuid_kinds! {
        kinds = {
            Single = {},
        }
    }

    assert_eq!(SingleKind::tag().as_str(), "single");
    let single_uuid = SingleUuid::new_v4();
    assert_eq!(single_uuid.get_version_num(), 4);
}

#[test]
fn test_snake_case_conversion() {
    impl_typed_uuid_kinds! {
        kinds = {
            // The Rust convention is to use `HttpClient`, `XmlParser`, etc, but
            // ensure that these all-caps variants also have the correct tags.
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
    // Test that we can handle an empty kinds map.
    impl_typed_uuid_kinds! {
        kinds = {}
    }
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

    // The GlobalA and GlobalB kinds should impl Clone + Copy + fmt::Debug + Eq
    // by default, and also implement Hash.
    assert_impl_all!(GlobalAKind: Clone, Copy, fmt::Debug, Eq, Hash);
    assert_impl_all!(GlobalBKind: Clone, Copy, fmt::Debug, Eq, Hash);
}

/// Test that per-kind `attrs` override global ones and are applied only to the
/// correct kind.
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
