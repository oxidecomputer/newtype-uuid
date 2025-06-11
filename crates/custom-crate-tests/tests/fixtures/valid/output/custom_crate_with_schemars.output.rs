#[derive(Debug, PartialEq, Eq)]
pub enum AccountKind {}
impl ::my_custom_uuid::TypedUuidKind for AccountKind {
    #[inline]
    fn tag() -> ::my_custom_uuid::TypedUuidTag {
        const TAG: ::my_custom_uuid::TypedUuidTag = ::my_custom_uuid::TypedUuidTag::new(
            "account",
        );
        TAG
    }
}
#[cfg(feature = "internal-schemars08-tests")]
impl ::schemars::JsonSchema for AccountKind {
    fn schema_name() -> ::std::string::String {
        "Account".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::AccountKind")
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
            { "crate" : "my-api-service", "version" : "2.1.0", "path" :
            "my_api_service::models::AccountKind" }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
pub type AccountUuid = ::my_custom_uuid::TypedUuid<AccountKind>;
#[derive(Debug, PartialEq, Eq)]
pub enum TransactionKind {}
impl ::my_custom_uuid::TypedUuidKind for TransactionKind {
    #[inline]
    fn tag() -> ::my_custom_uuid::TypedUuidTag {
        const TAG: ::my_custom_uuid::TypedUuidTag = ::my_custom_uuid::TypedUuidTag::new(
            "transaction",
        );
        TAG
    }
}
#[cfg(feature = "internal-schemars08-tests")]
impl ::schemars::JsonSchema for TransactionKind {
    fn schema_name() -> ::std::string::String {
        "Transaction".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::TransactionKind")
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
            { "crate" : "my-api-service", "version" : "2.1.0", "path" :
            "my_api_service::models::TransactionKind" }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
pub type TransactionUuid = ::my_custom_uuid::TypedUuid<TransactionKind>;
#[derive(Debug, PartialEq, Eq)]
pub enum ApiKeyKind {}
impl ::my_custom_uuid::TypedUuidKind for ApiKeyKind {
    #[inline]
    fn tag() -> ::my_custom_uuid::TypedUuidTag {
        const TAG: ::my_custom_uuid::TypedUuidTag = ::my_custom_uuid::TypedUuidTag::new(
            "api_key",
        );
        TAG
    }
}
#[cfg(feature = "internal-schemars08-tests")]
impl ::schemars::JsonSchema for ApiKeyKind {
    fn schema_name() -> ::std::string::String {
        "ApiKey".to_string()
    }
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("my_api_service::models::ApiKeyKind")
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
            { "crate" : "my-api-service", "version" : "2.1.0", "path" :
            "my_api_service::models::ApiKeyKind" }
        );
        extensions.insert("x-rust-type".to_string(), rust_type);
        schema.extensions = extensions;
        Schema::Object(schema)
    }
}
pub type ApiKeyUuid = ::my_custom_uuid::TypedUuid<ApiKeyKind>;
