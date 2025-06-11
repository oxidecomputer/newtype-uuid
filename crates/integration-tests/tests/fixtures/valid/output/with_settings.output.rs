#[derive(Debug, PartialEq, Eq)]
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
#[cfg(feature = "internal-schemars08-tests")]
impl ::schemars::JsonSchema for UserKind {
    fn schema_name() -> ::std::string::String {
        "User".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::UserKind")
    }
    fn json_schema(
        _gen: &mut ::schemars::gen::SchemaGenerator,
    ) -> ::schemars::schema::Schema {
        use ::schemars::schema::*;
        let mut schema = SchemaObject {
            instance_type: ::std::option::Option::Some(InstanceType::String.into()),
            format: ::std::option::Option::Some("uuid".to_string()),
            ..::std::default::Default::default()
        };
        let mut extensions = ::schemars::Map::new();
        let rust_type = ::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::UserKind" }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
pub type UserUuid = ::newtype_uuid::TypedUuid<UserKind>;
#[derive(Debug, PartialEq, Eq)]
pub enum OrganizationKind {}
impl ::newtype_uuid::TypedUuidKind for OrganizationKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "organization",
        );
        TAG
    }
}
#[cfg(feature = "internal-schemars08-tests")]
impl ::schemars::JsonSchema for OrganizationKind {
    fn schema_name() -> ::std::string::String {
        "Organization".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::OrganizationKind")
    }
    fn json_schema(
        _gen: &mut ::schemars::gen::SchemaGenerator,
    ) -> ::schemars::schema::Schema {
        use ::schemars::schema::*;
        let mut schema = SchemaObject {
            instance_type: ::std::option::Option::Some(InstanceType::String.into()),
            format: ::std::option::Option::Some("uuid".to_string()),
            ..::std::default::Default::default()
        };
        let mut extensions = ::schemars::Map::new();
        let rust_type = ::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::OrganizationKind" }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
pub type OrganizationUuid = ::newtype_uuid::TypedUuid<OrganizationKind>;
#[derive(Debug, PartialEq, Eq)]
pub enum ProjectKind {}
impl ::newtype_uuid::TypedUuidKind for ProjectKind {
    #[inline]
    fn tag() -> ::newtype_uuid::TypedUuidTag {
        const TAG: ::newtype_uuid::TypedUuidTag = ::newtype_uuid::TypedUuidTag::new(
            "project",
        );
        TAG
    }
}
#[cfg(feature = "internal-schemars08-tests")]
impl ::schemars::JsonSchema for ProjectKind {
    fn schema_name() -> ::std::string::String {
        "Project".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::ProjectKind")
    }
    fn json_schema(
        _gen: &mut ::schemars::gen::SchemaGenerator,
    ) -> ::schemars::schema::Schema {
        use ::schemars::schema::*;
        let mut schema = SchemaObject {
            instance_type: ::std::option::Option::Some(InstanceType::String.into()),
            format: ::std::option::Option::Some("uuid".to_string()),
            ..::std::default::Default::default()
        };
        let mut extensions = ::schemars::Map::new();
        let rust_type = ::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::ProjectKind" }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
pub type ProjectUuid = ::newtype_uuid::TypedUuid<ProjectKind>;
