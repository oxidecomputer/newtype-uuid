<!-- cargo-sync-rdme title [[ -->
# newtype-uuid-macros
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/newtype-uuid-macros.svg?)
[![crates.io](https://img.shields.io/crates/v/newtype-uuid-macros.svg?logo=rust)](https://crates.io/crates/newtype-uuid-macros)
[![docs.rs](https://img.shields.io/docsrs/newtype-uuid-macros.svg?logo=docs.rs)](https://docs.rs/newtype-uuid-macros)
[![Rust: ^1.79.0](https://img.shields.io/badge/rust-^1.79.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme rustdoc [[ -->
Procedural macro for [`newtype-uuid`](https://docs.rs/newtype-uuid).

This crate provides a procedural macro to help with creating
[`newtype-uuid`](https://docs.rs/newtype-uuid) instances.

For more information, see the documentation for [`impl_typed_uuid_kinds!`](https://docs.rs/newtype-uuid-macros/0.1.0/newtype_uuid_macros/macro.impl_typed_uuid_kinds.html).

## Examples

Basic usage:

````rust
use newtype_uuid::TypedUuidKind;
use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    kinds = {
        User = {},
        BusinessUnit = {},
    },
}

// This generates empty UserKind and BusinessUnitKind enums implementing
// TypedUuidKind, with the tags "user" and "business_unit" respectively.
// Tags are snake_case versions of type names.
assert_eq!(UserKind::tag().as_str(), "user");
assert_eq!(BusinessUnitKind::tag().as_str(), "business_unit");

// The macro also generates UserUuid and BusinessUnitUuid type aliases.
let user_uuid = UserUuid::new_v4();
let business_unit_uuid = BusinessUnitUuid::new_v4();
````

For more details and examples, see the documentation for
[`impl_typed_uuid_kinds!`](https://docs.rs/newtype-uuid-macros/0.1.0/newtype_uuid_macros/macro.impl_typed_uuid_kinds.html).
<!-- cargo-sync-rdme ]] -->

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).
