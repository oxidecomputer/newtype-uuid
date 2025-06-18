use super::error_store::{ErrorSink, ErrorStore};
use heck::ToSnakeCase;
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use serde_tokenstream::{
    from_tokenstream, from_tokenstream_spanned, OrderedMap, ParseWrapper, TokenStreamWrapper,
};
use syn::spanned::Spanned;

pub struct ImplKindsOutput {
    pub out: Option<TokenStream>,
    pub errors: Vec<syn::Error>,
}

impl ToTokens for ImplKindsOutput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.out.clone());
        tokens.extend(self.errors.iter().map(|error| error.to_compile_error()));
    }
}

pub fn impl_typed_uuid_kinds(input: TokenStream) -> ImplKindsOutput {
    let params: ImplKindsParams = match from_tokenstream(&input) {
        Ok(input) => input,
        Err(error) => {
            let errors = vec![error];
            return ImplKindsOutput { out: None, errors };
        }
    };

    let newtype_uuid_ident = syn::Ident::new("newtype_uuid", input.span());
    let newtype_uuid_crate = params
        .settings
        .newtype_uuid_crate
        .as_ref()
        .map_or_else(|| &newtype_uuid_ident, |crate_name| crate_name);

    let mut out = quote! {};

    let mut error_store = ErrorStore::new();
    let errors = error_store.sink();

    for (kind_tokens, config_tokens) in params.kinds {
        let errors = errors.new_child();

        let Some((root_ident, config)) = parse_kind(
            kind_tokens.into_inner(),
            config_tokens.into_inner(),
            errors.new_child(),
        ) else {
            // A critical error occurred for this kind -- can't proceed any
            // further.
            continue;
        };
        let Some(config) = config.validate(errors.new_child()) else {
            // The config couldn't be parsed -- can't proceed any further.
            continue;
        };

        let name = if let Some(tag) = &config.tag {
            KindOrExplicitTag::Tag(tag)
        } else {
            KindOrExplicitTag::Kind(&root_ident)
        };

        // Validate the tag name here using the same logic as in newtype-uuid.
        // Doing so in the proc macro results in better error messages.
        validate_tag_name(&name, errors.new_child());
        if errors.has_critical_errors() {
            // Don't generate output since it'll panic and lead to worse errors.
            continue;
        }

        let tag_name = name.tag_name();
        let kind_name_ident = config
            .type_name
            .unwrap_or_else(|| format_ident!("{}Kind", root_ident));
        let alias_ident = config
            .alias
            .unwrap_or_else(|| format_ident!("{}Uuid", root_ident));
        let attrs = config.attrs.as_ref().unwrap_or(&params.settings.attrs);
        let attrs = attrs.iter().map(|attr| &**attr);

        // Generate JsonSchema implementation if schemars08 settings are provided
        let schemars_impl = if let Some(schemars_settings) = &params.settings.schemars08 {
            generate_schemars_impl(
                &kind_name_ident,
                &kind_name_ident.to_string(),
                schemars_settings,
                newtype_uuid_crate,
            )
        } else {
            quote! {}
        };

        let expanded = quote! {
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            #(#attrs)*
            pub enum #kind_name_ident {}

            impl ::#newtype_uuid_crate::TypedUuidKind for #kind_name_ident {

                #[inline]
                fn tag() -> ::#newtype_uuid_crate::TypedUuidTag {
                    // `const` ensures that tags are validated at compile-time.
                    const TAG: ::#newtype_uuid_crate::TypedUuidTag = ::#newtype_uuid_crate::TypedUuidTag::new(#tag_name);
                    TAG
                }
            }

            #schemars_impl

            pub type #alias_ident = ::#newtype_uuid_crate::TypedUuid<#kind_name_ident>;
        };

        out.extend(expanded);
    }

    let errors = error_store.into_inner();
    ImplKindsOutput {
        out: Some(out),
        errors,
    }
}

fn parse_kind(
    kind_tokens: TokenStream,
    kind_config_tokens: TokenTree,
    errors: ErrorSink<'_, syn::Error>,
) -> Option<(syn::Ident, KindConfig)> {
    let kind_ident = match syn::parse2::<syn::Ident>(kind_tokens) {
        Ok(ident) => Some(ident),
        Err(err) => {
            // Collect the error.
            errors.push_critical(err);
            None
        }
    };
    // tokens is surrounded by {}
    let kind_config = match kind_config_tokens {
        TokenTree::Group(group) => {
            // group.delimiter() must be {}
            if group.delimiter() == Delimiter::Brace {
                match from_tokenstream_spanned::<KindConfig>(&group.delim_span(), &group.stream()) {
                    Ok(config) => Some(config),
                    Err(err) => {
                        // Collect the error.
                        errors.push_critical(err);
                        None
                    }
                }
            } else {
                // Collect the error.
                errors.push_critical(syn::Error::new(group.span(), "expected `{`"));
                None
            }
        }
        _ => {
            // Collect the error.
            errors.push_critical(syn::Error::new(kind_config_tokens.span(), "expected `{`"));
            None
        }
    };

    if errors.has_critical_errors() {
        None
    } else {
        Some((
            kind_ident.expect("no critical errors => kind is guaranteed to be Some"),
            kind_config.expect("no critical errors => kind config is guaranteed to be Some"),
        ))
    }
}

fn validate_tag_name(name: &KindOrExplicitTag<'_>, errors: ErrorSink<'_, syn::Error>) {
    let tag_name = name.tag_name();
    let span = name.span();

    let mut chars = tag_name.chars();
    let Some(first) = chars.next() else {
        errors.push_critical(syn::Error::new(
            span,
            format!("tag name must not be empty{}", name.hint()),
        ));
        return;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        errors.push_critical(syn::Error::new(
            span,
            format!(
                "tag name `{tag_name}` must start with an ASCII letter or underscore{}",
                name.hint(),
            ),
        ));
    }

    for c in chars {
        // Tag names can contain hyphens, but Rust identifiers cannot -- so we
        // don't check for that here. (Once we allow setting custom tag names,
        // we can check for that functionality.)
        if !(c.is_ascii_alphanumeric() || c == '_') {
            errors.push_critical(syn::Error::new(
                span,
                format!(
                    "tag name `{tag_name}` must consist of ASCII \
                     alphanumeric characters or underscores{}",
                    name.hint()
                ),
            ));
        }
    }
}

enum KindOrExplicitTag<'a> {
    /// A kind name was specified and will be converted into the corresponding
    /// tag name.
    Kind(&'a syn::Ident),

    /// A tag name was specified and will be used as-is.
    Tag(&'a syn::LitStr),
}

impl<'a> KindOrExplicitTag<'a> {
    fn tag_name(&self) -> String {
        match self {
            KindOrExplicitTag::Kind(kind_name) => kind_name.to_string().to_snake_case(),
            KindOrExplicitTag::Tag(tag_name) => tag_name.value(),
        }
    }

    fn span(&self) -> Span {
        match self {
            KindOrExplicitTag::Kind(kind_name) => kind_name.span(),
            KindOrExplicitTag::Tag(tag_name) => tag_name.span(),
        }
    }

    fn hint(&self) -> String {
        match self {
            KindOrExplicitTag::Kind(kind_name) => {
                format!(
                    "\n(hint: tag name `{}` derived from kind name -- \
                     specify `tag = \"...\" for a custom tag name`)",
                    kind_name.to_string().to_snake_case(),
                )
            }
            KindOrExplicitTag::Tag(_) => String::new(),
        }
    }
}

/// Input structure for the `impl_typed_uuid_kinds` macro.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ImplKindsParams {
    #[serde(default)]
    settings: GlobalSettings,
    kinds: OrderedMap<TokenStreamWrapper, ParseWrapper<TokenTree>>,
}

/// Global settings for the macro.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct GlobalSettings {
    /// The name of the newtype-uuid crate.
    #[serde(default)]
    newtype_uuid_crate: Option<ParseWrapper<syn::Ident>>,

    /// Attributes to apply to generated types.
    #[serde(default)]
    attrs: Vec<TokenStreamWrapper>,

    /// Schemars configuration.
    #[serde(default)]
    schemars08: Option<SchemarsSettings>,
}

/// Settings for schemars08 integration.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SchemarsSettings {
    #[serde(default)]
    attrs: Vec<TokenStreamWrapper>,
    rust_type: RustTypeSettings,
}

/// Settings for the x-rust-type extension.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RustTypeSettings {
    #[serde(rename = "crate")]
    crate_name: String,
    version: String,
    path: String,
}

/// Configuration for each kind.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct KindConfig {
    /// The type name for this kind.
    #[serde(default)]
    type_name: Option<TokenStreamWrapper>,

    /// The type alias for the kind.
    alias: Option<TokenStreamWrapper>,

    /// The tag for this kind. Defaults to the snake_case name of the kind.
    #[serde(default)]
    tag: Option<TokenStreamWrapper>,

    /// Attributes to apply to generated types (e.g. derives).
    #[serde(default)]
    attrs: Option<Vec<TokenStreamWrapper>>,
}

impl KindConfig {
    fn validate(self, errors: ErrorSink<'_, syn::Error>) -> Option<ParsedKindConfig> {
        // Parse the type name as an Ident.
        let type_name = match self.type_name {
            Some(type_name) => match syn::parse2::<syn::Ident>(type_name.into_inner()) {
                Ok(ident) => Ok(Some(ident)),
                Err(error) => {
                    errors.push_critical(error);
                    Err(())
                }
            },
            None => Ok(None),
        };
        // Parse the alias as an Ident.
        let alias = match self.alias {
            Some(alias) => match syn::parse2::<syn::Ident>(alias.into_inner()) {
                Ok(ident) => Ok(Some(ident)),
                Err(error) => {
                    errors.push_critical(error);
                    Err(())
                }
            },
            None => Ok(None),
        };
        // Parse the tag as a LitStr.
        let tag = match self.tag {
            Some(tag) => match syn::parse2::<syn::LitStr>(tag.into_inner()) {
                Ok(lit_str) => Ok(Some(lit_str)),
                Err(error) => {
                    errors.push_critical(error);
                    Err(())
                }
            },
            None => Ok(None),
        };

        if errors.has_critical_errors() {
            None
        } else {
            // All parsed values are valid if present.
            Some(ParsedKindConfig {
                type_name: type_name.expect("type name is valid"),
                alias: alias.expect("alias is valid"),
                tag: tag.expect("tag is valid"),
                attrs: self.attrs,
            })
        }
    }
}

struct ParsedKindConfig {
    type_name: Option<syn::Ident>,
    alias: Option<syn::Ident>,
    tag: Option<syn::LitStr>,
    attrs: Option<Vec<TokenStreamWrapper>>,
}

/// Generate a hand-written JsonSchema implementation for a kind.
fn generate_schemars_impl(
    kind_name_ident: &syn::Ident,
    kind_name: &str,
    schemars_settings: &SchemarsSettings,
    newtype_uuid_crate: &syn::Ident,
) -> proc_macro2::TokenStream {
    let attrs = schemars_settings.attrs.iter().map(|attrs| &**attrs);
    let crate_name = &schemars_settings.rust_type.crate_name;
    let version = &schemars_settings.rust_type.version;
    let path_prefix = &schemars_settings.rust_type.path;

    // Construct the full path for this specific kind.
    let full_path = format!("{}::{}", path_prefix, kind_name_ident);

    quote! {
        #(#attrs)*
        impl ::#newtype_uuid_crate::macro_support::schemars08::JsonSchema for #kind_name_ident {
            fn schema_name() -> ::std::string::String {
                #kind_name.to_string()
            }

            fn schema_id() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(#full_path)
            }

            fn json_schema(
                _gen: &mut ::#newtype_uuid_crate::macro_support::schemars08::gen::SchemaGenerator,
            ) -> ::#newtype_uuid_crate::macro_support::schemars08::schema::Schema {
                use ::#newtype_uuid_crate::macro_support::schemars08::schema::*;

                let mut schema = SchemaObject {
                    instance_type: ::std::option::Option::None,
                    enum_values: ::std::option::Option::Some(::std::vec::Vec::new()),
                    ..::std::default::Default::default()
                };

                // Add the x-rust-type extension.
                let mut extensions = ::#newtype_uuid_crate::macro_support::schemars08::Map::new();
                let rust_type = ::#newtype_uuid_crate::macro_support::serde_json::json!({
                    "crate": #crate_name,
                    "version": #version,
                    "path": #full_path,
                });
                extensions.insert("x-rust-type".to_string(), rust_type);
                schema.extensions = extensions;

                Schema::Object(schema)
            }
        }
    }
}
