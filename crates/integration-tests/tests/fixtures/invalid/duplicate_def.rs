use newtype_uuid_macros::impl_typed_uuid_kinds;

enum UserKind {}
struct UserUuid;

impl_typed_uuid_kinds! {
    kinds = {
        User = {},
    }
}

fn main() {}
