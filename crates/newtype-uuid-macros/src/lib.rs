//! Procedural macros for newtype-uuid
//!
//! This crate provides procedural macros to help with creating typed UUID kinds
//! with compile-time type safety.

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::collections::HashMap;

/// Input structure for the `impl_typed_uuid_kinds` macro.
#[derive(Deserialize)]
struct ImplTypedUuidKindsInput {
    #[serde(default)]
    settings: Option<GlobalSettings>,
    kinds: HashMap<String, KindConfig>,
}

/// Global settings for the macro.
#[derive(Deserialize)]
struct GlobalSettings {
    #[serde(default)]
    schemars08: Option<SchemarsSettings>,
}

/// Settings for schemars08 integration.
#[derive(Deserialize)]
struct SchemarsSettings {
    feature: String,
    rust_type: RustTypeSettings,
}

/// Settings for the x-rust-type extension.
#[derive(Deserialize)]
struct RustTypeSettings {
    #[serde(rename = "crate")]
    crate_name: String,
    version: String,
    path: String,
}

/// Configuration for each kind.
#[derive(Deserialize)]
struct KindConfig {
    // Currently empty, but can be extended in the future for per-kind configuration
}

/// A function-like procedural macro for implementing typed UUID kinds.
///
/// This macro generates enum types that implement `TypedUuidKind` and corresponding
/// type aliases for `TypedUuid<T>`.
///
/// # Example
///
/// ```ignore
/// use newtype_uuid_macros::impl_typed_uuid_kinds;
///
/// impl_typed_uuid_kinds! {
///     settings = {
///         schemars08 = {
///             feature = "schemars08",
///             rust_type = {
///                 crate = "my-crate",
///                 version = "*",
///                 path = "my_crate::mod",
///             },
///         },
///     },
///     kinds = {
///         User = {},
///         Organization = {},
///         Project = {},
///     }
/// }
/// ```
///
/// This generates:
/// - `UserKind` enum implementing `TypedUuidKind`
/// - `UserUuid` type alias for `TypedUuid<UserKind>`
/// - `OrganizationKind` enum implementing `TypedUuidKind`
/// - `OrganizationUuid` type alias for `TypedUuid<OrganizationKind>`
/// - `ProjectKind` enum implementing `TypedUuidKind`
/// - `ProjectUuid` type alias for `TypedUuid<ProjectKind>`
#[proc_macro]
pub fn impl_typed_uuid_kinds(input: TokenStream) -> TokenStream {
    let input: ImplTypedUuidKindsInput = match serde_tokenstream::from_tokenstream(&input.into()) {
        Ok(input) => input,
        Err(err) => {
            return syn::Error::new(proc_macro2::Span::call_site(), err.to_string())
                .to_compile_error()
                .into();
        }
    };

    let mut generated = quote! {};

    for (kind_name, _config) in input.kinds {
        let kind_ident = match syn::parse_str::<syn::Ident>(&kind_name) {
            Ok(ident) => ident,
            Err(err) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Invalid kind name '{}': {}", kind_name, err),
                )
                .to_compile_error()
                .into();
            }
        };

        // Generate tag by converting the kind name to snake_case
        let tag = kind_name.to_snake_case();

        // Generate {Kind}Kind and {Kind}Uuid identifiers
        let kind_name_ident = format_ident!("{}Kind", kind_ident);
        let uuid_name_ident = format_ident!("{}Uuid", kind_ident);

        // Generate JsonSchema implementation if schemars08 settings are provided
        let schemars_impl = if let Some(ref settings) = input.settings {
            if let Some(ref schemars_settings) = settings.schemars08 {
                generate_schemars_impl(&kind_name_ident, &kind_name, schemars_settings)
            } else {
                quote! {}
            }
        } else {
            quote! {}
        };

        let expanded = quote! {
            #[derive(Debug, PartialEq, Eq)]
            pub enum #kind_name_ident {}

            impl newtype_uuid::TypedUuidKind for #kind_name_ident {
                #[inline]
                fn tag() -> newtype_uuid::TypedUuidTag {
                    // `const` ensures that tags are validated at compile-time.
                    const TAG: newtype_uuid::TypedUuidTag = newtype_uuid::TypedUuidTag::new(#tag);
                    TAG
                }
            }

            #schemars_impl

            pub type #uuid_name_ident = newtype_uuid::TypedUuid<#kind_name_ident>;
        };

        generated.extend(expanded);
    }

    TokenStream::from(generated)
}

/// Generate a hand-written JsonSchema implementation for a kind.
fn generate_schemars_impl(
    kind_name_ident: &syn::Ident,
    kind_name: &str,
    schemars_settings: &SchemarsSettings,
) -> proc_macro2::TokenStream {
    let feature = &schemars_settings.feature;
    let crate_name = &schemars_settings.rust_type.crate_name;
    let version = &schemars_settings.rust_type.version;
    let path_prefix = &schemars_settings.rust_type.path;

    // Construct the full path for this specific kind
    let full_path = format!("{}::{}", path_prefix, kind_name_ident);

    quote! {
        #[cfg(feature = #feature)]
        impl schemars::JsonSchema for #kind_name_ident {
            fn schema_name() -> String {
                #kind_name.to_string()
            }

            fn schema_id() -> std::borrow::Cow<'static, str> {
                format!("{}::{}", #full_path, #kind_name).into()
            }

            fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                use schemars::schema::*;

                let mut schema = SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    format: Some("uuid".to_string()),
                    ..Default::default()
                };

                // Add x-rust-type extension
                let mut extensions = std::collections::BTreeMap::new();
                let rust_type = serde_json::json!({
                    "crate": #crate_name,
                    "version": #version,
                    "path": #full_path
                });
                extensions.insert("x-rust-type".to_string(), rust_type);
                schema.extensions = extensions;

                Schema::Object(schema)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_snake_case() {
        assert_eq!("User".to_snake_case(), "user");
        assert_eq!("UserAccount".to_snake_case(), "user_account");
        assert_eq!("Organization".to_snake_case(), "organization");
        assert_eq!("ProjectTask".to_snake_case(), "project_task");
        assert_eq!("HTTPClient".to_snake_case(), "http_client");
        assert_eq!("XMLParser".to_snake_case(), "xml_parser");
    }
}
