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
}
#[allow(unused)]
pub type UserUuid = ::newtype_uuid::TypedUuid<UserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustomKind {}
impl ::newtype_uuid::TypedUuidKind for CustomKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "custom",
        );
        TAG
    }
}
#[allow(unused)]
pub type CustomUuid = ::newtype_uuid::TypedUuid<CustomKind>;
