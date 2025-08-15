#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::newtype_uuid::TypedUuidKind for UserKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "user",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(UserUuid))
    }
}
#[allow(unused)]
pub type UserUuid = ::newtype_uuid::TypedUuid<UserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganizationKind {}
impl ::newtype_uuid::TypedUuidKind for OrganizationKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "organization",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(OrganizationUuid))
    }
}
#[allow(unused)]
pub type OrganizationUuid = ::newtype_uuid::TypedUuid<OrganizationKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectKind {}
impl ::newtype_uuid::TypedUuidKind for ProjectKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "project",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(ProjectUuid))
    }
}
#[allow(unused)]
pub type ProjectUuid = ::newtype_uuid::TypedUuid<ProjectKind>;
