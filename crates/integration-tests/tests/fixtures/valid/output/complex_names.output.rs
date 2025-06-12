#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTTPClientKind {}
impl ::newtype_uuid::TypedUuidKind for HTTPClientKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "http_client",
        );
        TAG
    }
}
pub type HTTPClientUuid = ::newtype_uuid::TypedUuid<HTTPClientKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XMLParserKind {}
impl ::newtype_uuid::TypedUuidKind for XMLParserKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "xml_parser",
        );
        TAG
    }
}
pub type XMLParserUuid = ::newtype_uuid::TypedUuid<XMLParserKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APIKeyKind {}
impl ::newtype_uuid::TypedUuidKind for APIKeyKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "api_key",
        );
        TAG
    }
}
pub type APIKeyUuid = ::newtype_uuid::TypedUuid<APIKeyKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOHandlerKind {}
impl ::newtype_uuid::TypedUuidKind for IOHandlerKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "io_handler",
        );
        TAG
    }
}
pub type IOHandlerUuid = ::newtype_uuid::TypedUuid<IOHandlerKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAccountKind {}
impl ::newtype_uuid::TypedUuidKind for UserAccountKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "user_account",
        );
        TAG
    }
}
pub type UserAccountUuid = ::newtype_uuid::TypedUuid<UserAccountKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectTaskKind {}
impl ::newtype_uuid::TypedUuidKind for ProjectTaskKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "project_task",
        );
        TAG
    }
}
pub type ProjectTaskUuid = ::newtype_uuid::TypedUuid<ProjectTaskKind>;
