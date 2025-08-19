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
#[cfg(feature = "internal-schemars08-tests")]
impl ::newtype_uuid::macro_support::schemars08::JsonSchema for UserKind {
    fn schema_name() -> ::std::string::String {
        "UserKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::UserKind")
    }
    fn json_schema(
        _gen: &mut ::newtype_uuid::macro_support::schemars08::gen::SchemaGenerator,
    ) -> ::newtype_uuid::macro_support::schemars08::schema::Schema {
        use ::newtype_uuid::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::newtype_uuid::macro_support::schemars08::Map::new();
        let rust_type = ::newtype_uuid::macro_support::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::UserKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
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
#[cfg(feature = "internal-schemars08-tests")]
impl ::newtype_uuid::macro_support::schemars08::JsonSchema for OrganizationKind {
    fn schema_name() -> ::std::string::String {
        "OrganizationKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::OrganizationKind")
    }
    fn json_schema(
        _gen: &mut ::newtype_uuid::macro_support::schemars08::gen::SchemaGenerator,
    ) -> ::newtype_uuid::macro_support::schemars08::schema::Schema {
        use ::newtype_uuid::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::newtype_uuid::macro_support::schemars08::Map::new();
        let rust_type = ::newtype_uuid::macro_support::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::OrganizationKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
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
#[cfg(feature = "internal-schemars08-tests")]
impl ::newtype_uuid::macro_support::schemars08::JsonSchema for ProjectKind {
    fn schema_name() -> ::std::string::String {
        "ProjectKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_service::types::ProjectKind")
    }
    fn json_schema(
        _gen: &mut ::newtype_uuid::macro_support::schemars08::gen::SchemaGenerator,
    ) -> ::newtype_uuid::macro_support::schemars08::schema::Schema {
        use ::newtype_uuid::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::newtype_uuid::macro_support::schemars08::Map::new();
        let rust_type = ::newtype_uuid::macro_support::serde_json::json!(
            { "crate" : "my-service", "version" : "1.0.0", "path" :
            "my_service::types::ProjectKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type ProjectUuid = ::newtype_uuid::TypedUuid<ProjectKind>;
