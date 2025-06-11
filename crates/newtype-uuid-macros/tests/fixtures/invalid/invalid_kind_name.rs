use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    kinds = {
        123InvalidName = {},
        NonÅscii = {},
        User = {},
        "" = {},
    }
}

fn main() {}
