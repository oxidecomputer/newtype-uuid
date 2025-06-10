//! JSON schema tests for newtype-uuid.

use newtype_uuid::{TypedUuid, TypedUuidKind, TypedUuidTag};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use typify::TypeSpaceSettings;

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
    let output = generate_schema_with(&settings, schema);
    expectorate::assert_contents("outputs/schema-rust-with-replace.rs", &output);
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
