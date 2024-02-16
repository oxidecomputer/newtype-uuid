//! Derive macros for the [`newtype-uuid`](https://docs.rs/newtype-uuid) crate.
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

/// Derive the [`TypedUuidKind`] trait for this type.
///
/// By default, the tag for the derived implementation is the name of the Rust
/// type that derives [`TypedUuidKind`]. For example:
/// ```
/// use newtype_uuid::TypedUuidKind;
///
/// #[derive(TypedUuidKind)]
/// struct MyKind;
///
/// assert_eq!(MyKind::tag().as_str(), "MyKind");
/// ```
///
/// The `#[typed_uuid_tag("tag")]` attribute can be used to specify a string
/// literal to use as the [`TypedUuidTag`] for the derived [`TypedUuidKind`]
/// implementation. For example:
///
/// ```
/// use newtype_uuid::TypedUuidKind;
///
/// #[derive(TypedUuidKind)]
/// #[typed_uuid_tag("my-great-uuid-kind")]
/// struct MyKind;
///
/// assert_eq!(MyKind::tag().as_str(), "my-great-uuid-kind");
/// ```
///
/// The tag is provided as a string literal. It must be non-empty and consist
/// of only the following characters:
///
/// - ASCII letters
/// - digits (only after the first character)
/// - underscores
/// - hyphens (only after the first character)
///
/// For example, the following will not compile:
///
/// ```compile_fail
/// use newtype_uuid::TypedUuidKind;
///
/// #[derive(TypedUuidKind)]
/// #[typed_uuid_tag("1tag")]
/// struct MyKind;
/// ```
///
/// ```compile_fail
/// use newtype_uuid::TypedUuidKind;
///
/// #[derive(TypedUuidKind)]
/// #[typed_uuid_tag("")]
/// struct MyKind;
/// ```
///
/// ```compile_fail
/// use newtype_uuid:: TypedUuidKind;
///
/// #[derive(TypedUuidKind)]
/// #[typed_uuid_tag()]
/// struct MyKind;
/// ```
/// [`TypedUuidKind`]:
///     https://docs.rs/newtype-uuid/latest/newtype_uuid/trait.TypedUuidKind.html
/// [`TypedUuidTag`]: https://docs.rs/newtype-uuid/latest/newtype_uuid/struct.TypedUuidTag.html
#[proc_macro_derive(TypedUuidKind, attributes(typed_uuid_tag))]
pub fn derive_typed_uuid_kind(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match gen_typed_uuid_kind(input) {
        Ok(tokens) => tokens.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn gen_typed_uuid_kind(input: DeriveInput) -> Result<impl ToTokens, syn::Error> {
    let name = input.ident;
    let tag = {
        let tag_attr = input
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("typed_uuid_tag"));
        match tag_attr {
            Some(tag_attr) => {
                let tag = tag_attr.parse_args::<syn::LitStr>()?;
                let tag = validate_tag(tag)?;
                quote! { #tag }
            }
            None => quote! { stringify!(#name) },
        }
    };
    Ok(quote! {
        impl newtype_uuid::TypedUuidKind for #name {
            fn tag() -> newtype_uuid::TypedUuidTag {
                const TAG: newtype_uuid::TypedUuidTag = newtype_uuid::TypedUuidTag::new(#tag);
                TAG
            }
        }
    })
}

fn validate_tag(tag: syn::LitStr) -> Result<syn::LitStr, syn::Error> {
    let tag_str = tag.value();
    let mut chars = tag_str.chars();
    let c = match chars.next() {
        Some(c) => c,
        None => return Err(syn::Error::new_spanned(tag, "tag must not be empty")),
    };

    if !c.is_ascii_alphabetic() && c != '_' {
        return Err(syn::Error::new_spanned(
            tag,
            format!("a tag's first character may only be an ASCII letter or an underscore (found {c:?})"),
        ));
    }

    for c in tag_str.chars() {
        if !c.is_ascii_alphanumeric() && c != '_' && c != '-' {
            return Err(syn::Error::new_spanned(
                tag,
                format!("a tag may only contain ASCII alphanumeric characters, underscores, and hyphens (found {c:?})"),
            ));
        }
    }

    Ok(tag)
}
