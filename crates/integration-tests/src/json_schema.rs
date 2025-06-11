//! JSON schema tests for newtype-uuid.

use newtype_uuid::{TypedUuid, TypedUuidKind, TypedUuidTag};
use newtype_uuid_macros::impl_typed_uuid_kinds;
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

    // Use typify with a replace directive - this is the intended usage with x-rust-type.
    // The x-rust-type extension enables automatic replacement, so we don't try to
    // generate Rust code for the empty enum directly.
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

impl_typed_uuid_kinds! {
    settings = {
        schemars08 = {
            feature = "internal-schemars08-tests",
            rust_type = {
                crate = "my-crate",
                version = "1.0.0",
                path = "my_crate::types",
            },
        },
    },
    kinds = {
        Test = {},
        Another = {},
    }
}

#[test]
fn test_schemars_macro_integration() {
    // Test that JsonSchema is implemented
    use schemars::JsonSchema;

    // Test schema_name
    assert_eq!(TestKind::schema_name(), "Test");
    assert_eq!(AnotherKind::schema_name(), "Another");

    // Test that we can generate schemas
    let mut gen = schemars::gen::SchemaGenerator::default();
    let schema = TestKind::json_schema(&mut gen);

    // Verify it's a string with uuid format
    if let schemars::schema::Schema::Object(obj) = schema {
        assert_eq!(
            obj.instance_type,
            Some(schemars::schema::InstanceType::String.into())
        );
        assert_eq!(obj.format, Some("uuid".to_string()));

        // Verify x-rust-type extension is present
        assert!(obj.extensions.contains_key("x-rust-type"));
        if let Some(rust_type) = obj.extensions.get("x-rust-type") {
            let rust_type_obj = rust_type.as_object().unwrap();
            assert_eq!(rust_type_obj["crate"].as_str().unwrap(), "my-crate");
            assert_eq!(rust_type_obj["version"].as_str().unwrap(), "1.0.0");
            assert_eq!(
                rust_type_obj["path"].as_str().unwrap(),
                "my_crate::types::TestKind"
            );
        }
    } else {
        panic!("Expected Schema::Object");
    }
}

#[test]
fn test_macro_generated_tags() {
    // Test that the generated kinds have the correct tags
    assert_eq!(TestKind::tag().as_str(), "test");
    assert_eq!(AnotherKind::tag().as_str(), "another");
}
