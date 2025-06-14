use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    settings = {
        newtype_uuid_crate = "my_custom_uuid",
    },
    kinds = {
        User = {},
    }
}

fn main() {}
