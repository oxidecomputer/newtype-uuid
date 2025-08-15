use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    settings = {
        schemars08 = {
            attrs = [#[cfg(feature = "schemars08")]],
            rust_type = {
                crate = "my-service",
                version = "1.0.0",
                // Missing required 'path' field
            },
        },
    },
    kinds = {
        User = {},
    }
}

fn main() {}
