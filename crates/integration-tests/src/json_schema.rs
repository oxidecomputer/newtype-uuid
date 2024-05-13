//! JSON schema tests for newtype-uuid.

use dropshot::{
    endpoint, ApiDescription, HttpError, HttpResponseOk, Path, Query, RequestContext, TypedBody,
};
use newtype_uuid::{TypedUuid, TypedUuidKind, TypedUuidTag};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use typify::TypeSpaceSettings;

#[derive(Debug)]
enum MyKind {}

impl JsonSchema for MyKind {
    fn schema_name() -> String {
        "MyKind".to_string()
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::SchemaObject {
            subschemas: Some(
                schemars::schema::SubschemaValidation {
                    not: Some(schemars::schema::Schema::Bool(true).into()),
                    ..Default::default()
                }
                .into(),
            ),
            extensions: [(
                "x-rust-type".to_string(),
                serde_json::json!({
                    "crate": "my-crate",
                    "version": "0.1.0",
                    "path": "my_crate::MyKind",
                }),
            )]
            .into_iter()
            .collect(),
            ..Default::default()
        }
        .into()
    }
}

impl TypedUuidKind for MyKind {
    fn tag() -> TypedUuidTag {
        const TAG: TypedUuidTag = TypedUuidTag::new("my_kind");
        TAG
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct MyPathStruct {
    id: TypedUuid<MyKind>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct MyQueryStruct {
    query_id: TypedUuid<MyKind>,
}

#[endpoint {
    method = POST,
    path = "/my-endpoint/{id}",
}]
async fn my_endpoint(
    _rqctx: RequestContext<()>,
    _path: Path<MyPathStruct>,
    _query: Query<MyQueryStruct>,
    _body: TypedBody<MyPathStruct>,
) -> Result<HttpResponseOk<MyPathStruct>, HttpError> {
    unreachable!("this method is never actually called")
}

#[test]
fn test_json_schema_snapshot() {
    let schema = schemars::schema_for!(MyPathStruct);
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", std::env::current_dir().unwrap().display());
    expectorate::assert_contents("outputs/typed-uuid-schema.json", &schema_json);

    // Now attempt to use typify to convert the JSON schema into Rust code.
    let output = generate_schema_with(&TypeSpaceSettings::default(), schema.clone());
    expectorate::assert_contents("outputs/schema-rust.rs", &output);

    // Do so, with a replace directive.
    let mut settings = TypeSpaceSettings::default();
    settings.with_replacement(
        "TypedUuidForMyKind",
        "::newtype_uuid::TypedUuid<::my_crate::MyKind>",
        std::iter::empty(),
    );
    let output = generate_schema_with(&settings, schema.clone());
    expectorate::assert_contents("outputs/schema-rust-with-replace.rs", &output);

    // And finally, using the x-rust-type extension we include.
    let mut settings = TypeSpaceSettings::default();
    settings
        .with_crate(
            "newtype-uuid",
            typify::CrateVers::Version("1.2.0".parse().unwrap()),
            None,
        )
        .with_crate(
            "my-crate",
            typify::CrateVers::Version("0.1.0".parse().unwrap()),
            None,
        );
    let output = generate_schema_with(&settings, schema);
    expectorate::assert_contents("outputs/schema-rust-with-extension.rs", &output);
}

fn generate_schema_with(
    settings: &TypeSpaceSettings,
    schema: schemars::schema::RootSchema,
) -> String {
    let mut type_space = typify::TypeSpace::new(settings);
    type_space
        .add_root_schema(schema)
        .expect("adding root schema succeeded");
    let tokens = type_space.to_stream();
    let file: syn::File = syn::parse2(tokens).expect("parsing tokens succeeded");
    prettyplease::unparse(&file)
}

#[test]
fn test_openapi_snapshot() {
    let mut api = ApiDescription::new();
    api.register(my_endpoint).unwrap();
    let openapi = api.openapi("my-api", "1.0.0");
    let json_value = openapi.json().expect("serialization to json worked");
    let api_json = serde_json::to_string_pretty(&json_value).unwrap();
    expectorate::assert_contents("outputs/typed-uuid-openapi.json", &api_json);
}
