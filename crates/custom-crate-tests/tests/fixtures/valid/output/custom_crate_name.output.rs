#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserKind {}
impl ::my_custom_uuid::TypedUuidKind for UserKind {
    #[inline]
    fn tag() -> ::my_custom_uuid::TypedUuidTag {
        const TAG: ::my_custom_uuid::TypedUuidTag = ::my_custom_uuid::TypedUuidTag::new(
            "user",
        );
        TAG
    }
}
#[allow(unused)]
pub type UserUuid = ::my_custom_uuid::TypedUuid<UserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganizationKind {}
impl ::my_custom_uuid::TypedUuidKind for OrganizationKind {
    #[inline]
    fn tag() -> ::my_custom_uuid::TypedUuidTag {
        const TAG: ::my_custom_uuid::TypedUuidTag = ::my_custom_uuid::TypedUuidTag::new(
            "organization",
        );
        TAG
    }
}
#[allow(unused)]
pub type OrganizationUuid = ::my_custom_uuid::TypedUuid<OrganizationKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductKind {}
impl ::my_custom_uuid::TypedUuidKind for ProductKind {
    #[inline]
    fn tag() -> ::my_custom_uuid::TypedUuidTag {
        const TAG: ::my_custom_uuid::TypedUuidTag = ::my_custom_uuid::TypedUuidTag::new(
            "product",
        );
        TAG
    }
}
#[allow(unused)]
pub type ProductUuid = ::my_custom_uuid::TypedUuid<ProductKind>;
