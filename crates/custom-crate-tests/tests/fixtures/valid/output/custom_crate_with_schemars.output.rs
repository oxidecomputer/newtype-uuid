#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccountKind {}
impl ::my_custom_uuid::TypedUuidKind for AccountKind {
    #[inline]
    fn tag() -> ::my_custom_uuid::TypedUuidTag {
        const TAG: ::my_custom_uuid::TypedUuidTag = ::my_custom_uuid::TypedUuidTag::new(
            "account",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(AccountUuid))
    }
}
#[cfg(feature = "internal-schemars08-tests")]
impl ::my_custom_uuid::macro_support::schemars08::JsonSchema for AccountKind {
    fn schema_name() -> ::std::string::String {
        "AccountKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::AccountKind")
    }
    fn json_schema(
        _gen: &mut ::my_custom_uuid::macro_support::schemars08::gen::SchemaGenerator,
    ) -> ::my_custom_uuid::macro_support::schemars08::schema::Schema {
        use ::my_custom_uuid::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            instance_type: ::std::option::Option::None,
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::my_custom_uuid::macro_support::schemars08::Map::new();
        let rust_type = ::my_custom_uuid::macro_support::serde_json::json!(
            { "crate" : "my-api-service", "version" : "2.1.0", "path" :
            "my_api_service::models::AccountKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type AccountUuid = ::my_custom_uuid::TypedUuid<AccountKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionKind {}
impl ::my_custom_uuid::TypedUuidKind for TransactionKind {
    #[inline]
    fn tag() -> ::my_custom_uuid::TypedUuidTag {
        const TAG: ::my_custom_uuid::TypedUuidTag = ::my_custom_uuid::TypedUuidTag::new(
            "transaction",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(TransactionUuid))
    }
}
#[cfg(feature = "internal-schemars08-tests")]
impl ::my_custom_uuid::macro_support::schemars08::JsonSchema for TransactionKind {
    fn schema_name() -> ::std::string::String {
        "TransactionKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::TransactionKind")
    }
    fn json_schema(
        _gen: &mut ::my_custom_uuid::macro_support::schemars08::gen::SchemaGenerator,
    ) -> ::my_custom_uuid::macro_support::schemars08::schema::Schema {
        use ::my_custom_uuid::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            instance_type: ::std::option::Option::None,
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::my_custom_uuid::macro_support::schemars08::Map::new();
        let rust_type = ::my_custom_uuid::macro_support::serde_json::json!(
            { "crate" : "my-api-service", "version" : "2.1.0", "path" :
            "my_api_service::models::TransactionKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type TransactionUuid = ::my_custom_uuid::TypedUuid<TransactionKind>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiKeyKind {}
impl ::my_custom_uuid::TypedUuidKind for ApiKeyKind {
    #[inline]
    fn tag() -> ::my_custom_uuid::TypedUuidTag {
        const TAG: ::my_custom_uuid::TypedUuidTag = ::my_custom_uuid::TypedUuidTag::new(
            "api_key",
        );
        TAG
    }
    fn alias() -> Option<&'static str> {
        Some(stringify!(ApiKeyUuid))
    }
}
#[cfg(feature = "internal-schemars08-tests")]
impl ::my_custom_uuid::macro_support::schemars08::JsonSchema for ApiKeyKind {
    fn schema_name() -> ::std::string::String {
        "ApiKeyKind".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::ApiKeyKind")
    }
    fn json_schema(
        _gen: &mut ::my_custom_uuid::macro_support::schemars08::gen::SchemaGenerator,
    ) -> ::my_custom_uuid::macro_support::schemars08::schema::Schema {
        use ::my_custom_uuid::macro_support::schemars08::schema::*;
        let mut schema = SchemaObject {
            instance_type: ::std::option::Option::None,
            subschemas: ::std::option::Option::Some(
                Box::new(SubschemaValidation {
                    not: ::std::option::Option::Some(Box::new(Schema::Bool(true))),
                    ..::std::default::Default::default()
                }),
            ),
            ..::std::default::Default::default()
        };
        let mut extensions = ::my_custom_uuid::macro_support::schemars08::Map::new();
        let rust_type = ::my_custom_uuid::macro_support::serde_json::json!(
            { "crate" : "my-api-service", "version" : "2.1.0", "path" :
            "my_api_service::models::ApiKeyKind", }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
#[allow(unused)]
pub type ApiKeyUuid = ::my_custom_uuid::TypedUuid<ApiKeyKind>;
