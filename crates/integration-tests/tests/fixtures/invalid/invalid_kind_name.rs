use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    kinds = {
        123Invalid = {},
        NonÅscii = {},
        // User and Custom are valid.
        User = {},
        Custom = { tag = "custom" },
        Hello = { tag = "Hellö" },
        Tag = "hi",
        Youre = [ "it" ],
        Bad = {
            tag = "this_is_fine",
            type_name = 42,
            alias = 13,
        },
        "" = {},
    }
}

fn main() {
    // UserUuid and CustomUuid should exist.
    let _user = UserUuid::nil();
    let _custom = CustomUuid::nil();
    let _user_kind: UserKind;
    let _custom_kind: CustomKind;
}
