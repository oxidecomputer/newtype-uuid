use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    kinds = {}
}

fn main() {
    // Test that the macro compiles successfully with no kinds defined
    // This should generate no code but not error
}
