use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    // this is not valid syntax at all
    User => "user",
    Organization => "org",
}

fn main() {}
