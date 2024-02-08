//! JSON schema tests for newtype-uuid.

use dropshot::{
    endpoint, ApiDescription, HttpError, HttpResponseOk, Path, Query, RequestContext, TypedBody,
};
use newtype_uuid::{TypedUuid, TypedUuidKind, TypedUuidTag};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, JsonSchema)]
enum MyKind {}

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
    expectorate::assert_contents(
        "tests/integration-tests/outputs/typed-uuid-schema.json",
        &schema_json,
    );

    // Now attempt to use typify to convert the JSON schema into Rust code.
    let mut type_space = typify::TypeSpace::default();
    type_space
        .add_root_schema(schema)
        .expect("adding root schema succeeded");
    let tokens = type_space.to_stream();
    let file: syn::File = syn::parse2(tokens).expect("parsing tokens succeeded");
    let output = prettyplease::unparse(&file);
    expectorate::assert_contents("tests/integration-tests/outputs/schema-rust.rs", &output);
}

#[test]
fn test_openapi_snapshot() {
    let mut api = ApiDescription::new();
    api.register(my_endpoint).unwrap();
    let openapi = api.openapi("my-api", "1.0.0");
    let json_value = openapi.json().expect("serialization to json worked");
    let api_json = serde_json::to_string_pretty(&json_value).unwrap();
    expectorate::assert_contents(
        "tests/integration-tests/outputs/typed-uuid-openapi.json",
        &api_json,
    );
}
