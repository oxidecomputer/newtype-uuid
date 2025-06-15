//! End-to-end UUID kinds definitions.
//!
//! This crate defines the UUID kinds used by both the producer and consumer
//! in the end-to-end JSON schema example.

use newtype_uuid_macros::impl_typed_uuid_kinds;

// Use the newtype-uuid-macros crate to define typed UUID kinds.
impl_typed_uuid_kinds! {
    settings = {
        // Defining the `schemars08` entry causes the kinds to implement
        // `JsonSchema`.
        schemars08 = {
            // settings.schemars08 accepts an `attrs` field which can be used to
            // define attributes like `#[cfg(feature)]` flags.
            attrs = [#[cfg(feature = "schemars08")]],
            // The `rust_type` setting is optional. Defining it makes the kinds
            // suitable for automatic replacement with typify and other tools.
            rust_type = {
                crate = "e2e-kinds",
                // This version should typically match the version of the crate,
                // but it can also be "*" to match any version. "*" is
                // recommended when the list of kinds is append-only.
                version = "*", // or version = "0.1.0"
                // This is the path to the crate and module where the kinds are
                // defined. Here, the kinds are accessible from the crate root.
                path = "e2e_kinds",
            },
        },
    },
    // A couple of example UUID kinds.
    kinds = {
        User = {},
        Organization = {},
    }
}
