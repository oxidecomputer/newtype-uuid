use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    settings = {
        newtype_uuid_crate = nonexistent_crate,
    },
    kinds = {
        User = {},
    }
}

fn main() {}
