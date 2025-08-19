//! JSON schema tests for newtype-uuid.

use newtype_uuid::TypedUuidKind;
use newtype_uuid_macros::impl_typed_uuid_kinds;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use typify::{CrateVers, TypeSpaceSettings};

impl_typed_uuid_kinds! {
    settings = {
        schemars08 = {
            attrs = [
                #[cfg(feature = "internal-schemars08-tests")],
            ],
            rust_type = {
                crate = "my-crate",
                version = "1.0.0",
                path = "my_crate::types",
            },
        },
    },
    kinds = {
        My = {},
        Test = {},
        Another = {},
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct MyPathStruct {
    id: MyUuid,
}

#[test]
fn test_json_schema_snapshot() {
    let schema = schemars::schema_for!(MyPathStruct);
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", std::env::current_dir().unwrap().display());
    expectorate::assert_contents("outputs/typed-uuid-schema.json", &schema_json);

    // Use typify with crate directives -- this is the intended usage with
    // x-rust-type. The x-rust-type extension enables automatic replacement, so
    // that we don't try to generate Rust code for either newtype-uuid or
    // my-crate.
    let mut settings = TypeSpaceSettings::default();
    settings
        .with_crate("newtype-uuid", CrateVers::Any, None)
        .with_crate("my-crate", CrateVers::Any, None);
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

#[test]
fn test_schemars_macro_integration() {
    // Test that JsonSchema is implemented
    use schemars::JsonSchema;

    // Test schema_name
    assert_eq!(TestKind::schema_name(), "TestKind");
    assert_eq!(AnotherKind::schema_name(), "AnotherKind");

    // Test that we can generate schemas.
    let mut generator = schemars::r#gen::SchemaGenerator::default();
    let schema = TestKind::json_schema(&mut generator);

    // Verify it's set to "not: true".
    let expected_schema = schemars::schema::Schema::Object(schemars::schema::SchemaObject {
        instance_type: None,
        subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
            not: Some(Box::new(schemars::schema::Schema::Bool(true))),
            ..Default::default()
        })),
        extensions: {
            let mut extensions = std::collections::BTreeMap::new();
            extensions.insert(
                "x-rust-type".to_string(),
                serde_json::json!({
                    "crate": "my-crate",
                    "version": "1.0.0",
                    "path": "my_crate::types::TestKind"
                }),
            );
            extensions
        },
        ..Default::default()
    });
    assert_eq!(schema, expected_schema);
}

#[test]
fn test_macro_generated_tags() {
    // Test that the generated kinds have the correct tags
    assert_eq!(TestKind::tag().as_str(), "test");
    assert_eq!(AnotherKind::tag().as_str(), "another");
}
