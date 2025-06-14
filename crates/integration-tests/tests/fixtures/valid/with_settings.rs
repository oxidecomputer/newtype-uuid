use newtype_uuid::TypedUuidKind;
use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    settings = {
        schemars08 = {
            attrs = [#[cfg(feature = "internal-schemars08-tests")]],
            rust_type = {
                crate = "my-service",
                version = "1.0.0",
                path = "my_service::types",
            },
        },
    },
    kinds = {
        User = {},
        Organization = {},
        Project = {},
    }
}

fn main() {
    // Test that the generated types exist and work
    let _user_kind_tag = UserKind::tag();
    let _org_kind_tag = OrganizationKind::tag();
    let _project_kind_tag = ProjectKind::tag();

    // Test type aliases exist
    let _user_uuid: UserUuid;
    let _org_uuid: OrganizationUuid;
    let _project_uuid: ProjectUuid;

    // Test that tags are properly snake_cased
    assert_eq!(UserKind::tag().as_str(), "user");
    assert_eq!(OrganizationKind::tag().as_str(), "organization");
    assert_eq!(ProjectKind::tag().as_str(), "project");
}
