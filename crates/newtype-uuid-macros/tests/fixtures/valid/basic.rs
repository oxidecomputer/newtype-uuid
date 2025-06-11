use newtype_uuid::TypedUuidKind;
use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    kinds = {
        User = {},
        Organization = {},
    }
}

fn main() {
    // Test that the generated types exist and work
    let _user_kind_tag = UserKind::tag();
    let _org_kind_tag = OrganizationKind::tag();

    // Test type aliases exist
    let _user_uuid: UserUuid;
    let _org_uuid: OrganizationUuid;
}
