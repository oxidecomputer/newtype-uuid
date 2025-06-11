use crate::internals::error_store::{ErrorSink, ErrorStore};
use heck::ToSnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use serde_tokenstream::{OrderedMap, TokenStreamWrapper};

pub(crate) struct ImplKindsOutput {
    pub(crate) out: Option<TokenStream>,
    pub(crate) errors: Vec<syn::Error>,
}

impl ToTokens for ImplKindsOutput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.out.clone());
        tokens.extend(self.errors.iter().map(|error| error.to_compile_error()));
    }
}

pub(crate) fn impl_typed_uuid_kinds(input: TokenStream) -> ImplKindsOutput {
    let input: ImplKindsInput = match serde_tokenstream::from_tokenstream(&input) {
        Ok(input) => input,
        Err(error) => {
            let errors = vec![error];
            return ImplKindsOutput { out: None, errors };
        }
    };

    let mut out = quote! {};

    let mut error_store = ErrorStore::new();
    let errors = error_store.sink();

    for (kind_tokens, _config) in input.kinds {
        // Parse the kind tokens into a syn::Ident one at a time. This allows
        // some generated output to exist even if other kinds fail to parse.
        let kind_tokens = kind_tokens.into_inner();
        let kind_ident = match syn::parse2::<syn::Ident>(kind_tokens) {
            Ok(ident) => ident,
            Err(err) => {
                // Collect the error.
                errors.push_critical(err);
                continue;
            }
        };

        let kind_name = kind_ident.to_string();
        validate_kind_name(kind_ident.span(), &kind_name, errors.new_child());
        if errors.has_critical_errors() {
            // Don't generate output since it'll panic and lead to worse errors.
            continue;
        }

        // Generate the tag by converting the kind name to snake_case.
        let tag = kind_name.to_snake_case();

        // Generate the {Kind}Kind and {Kind}Uuid identifiers
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

        out.extend(expanded);
    }

    let errors = error_store.into_inner();
    ImplKindsOutput {
        out: Some(out),
        errors,
    }
}

fn validate_kind_name(span: Span, kind_name: &str, errors: ErrorSink<'_, syn::Error>) {
    let mut chars = kind_name.chars();
    let Some(first) = chars.next() else {
        errors.push_critical(syn::Error::new(span, "kind name must not be empty"));
        return;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        errors.push_critical(syn::Error::new(
            span,
            "kind name must start with an ASCII letter or underscore",
        ));
    }

    for c in chars {
        // Tag names can contain hyphens, but Rust identifiers cannot -- so we
        // don't check for that here. (Once we allow setting custom tag names,
        // we can check for that functionality.)
        if !(c.is_ascii_alphanumeric() || c == '_') {
            errors.push_critical(syn::Error::new(
                span,
                "kind name must consist of ASCII alphanumeric characters or underscores",
            ));
        }
    }
}

/// Input structure for the `impl_typed_uuid_kinds` macro.
#[derive(Deserialize)]
struct ImplKindsInput {
    #[serde(default)]
    settings: Option<GlobalSettings>,
    kinds: OrderedMap<TokenStreamWrapper, KindConfig>,
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
    #[serde(default)]
    feature: Option<String>,
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

/// Generate a hand-written JsonSchema implementation for a kind.
fn generate_schemars_impl(
    kind_name_ident: &syn::Ident,
    kind_name: &str,
    schemars_settings: &SchemarsSettings,
) -> proc_macro2::TokenStream {
    let feature = schemars_settings.feature.as_ref().map(|feature| {
        quote! { #[cfg(feature = #feature)] }
    });
    let crate_name = &schemars_settings.rust_type.crate_name;
    let version = &schemars_settings.rust_type.version;
    let path_prefix = &schemars_settings.rust_type.path;

    // Construct the full path for this specific kind
    let full_path = format!("{}::{}", path_prefix, kind_name_ident);

    quote! {
        #feature
        impl schemars::JsonSchema for #kind_name_ident {
            fn schema_name() -> String {
                #kind_name.to_string()
            }

            fn schema_id() -> std::borrow::Cow<'static, str> {
                #full_path
            }

            fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                use schemars::schema::*;

                let mut schema = SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    format: Some("uuid".to_string()),
                    ..Default::default()
                };

                // Add x-rust-type extension
                let mut extensions = schemars::Map::new();
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
