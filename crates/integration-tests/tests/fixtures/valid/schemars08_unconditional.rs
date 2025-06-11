//! Test a situation in which settings.schemars08.feature isn't specified. In
//! this case, we expect the macro itself to be unconditional.

#[cfg(feature = "internal-schemars08-tests")]
newtype_uuid_macros::impl_typed_uuid_kinds! {
    settings = {
        schemars08 = {
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
    #[cfg(feature = "internal-schemars08-tests")]
    {
        use newtype_uuid::TypedUuidKind;

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
}
