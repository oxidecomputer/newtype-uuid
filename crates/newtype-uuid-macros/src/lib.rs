//! Procedural macro for [`newtype-uuid`](https://docs.rs/newtype-uuid).
//!
//! This crate provides a procedural macro to help with creating
//! [`newtype-uuid`](https://docs.rs/newtype-uuid) instances.
//!
//! For more information, see the documentation for [`impl_typed_uuid_kinds!`].

mod internals;

use proc_macro::TokenStream;
use quote::ToTokens;

/// A function-like procedural macro for implementing typed UUID kinds.
///
/// This macro generates enum types that implement `TypedUuidKind` and corresponding
/// type aliases for `TypedUuid<T>`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use newtype_uuid::TypedUuidKind;
/// use newtype_uuid_macros::impl_typed_uuid_kinds;
///
/// impl_typed_uuid_kinds! {
///     kinds = {
///         User = {},
///         Organization = {},
///     },
/// }
///
/// // This generates empty UserKind and OrganizationKind enums implementing
/// // TypedUuidKind, with the tags "user" and "organization" respectively.
/// // Tags are snake_case versions of type names.
/// assert_eq!(UserKind::tag(), "user");
/// assert_eq!(OrganizationKind::tag(), "organization");
///
/// // The macro also generates UserUuid and OrganizationUuid type aliases.
/// let user_uuid = UserUuid::new_v4();
/// let organization_uuid = OrganizationUuid::new_v4();
/// ```
///
/// # Per-kind settings
///
/// Kinds can be customized with:
///
/// - `tag`: The tag to use for the kind (a string literal).
/// - `type_name`: The name of the type to use for the kind (an identifier).
/// - `alias`: The name of the type alias to use for the kind (an identifier).
///
/// Per-kind customizations should generally be unnecessary; the conventionally
/// generated type names should be sufficient for most use cases.
///
/// ## Example
///
/// ```
/// use newtype_uuid_macros::impl_typed_uuid_kinds;
///
/// impl_typed_uuid_kinds! {
///     kinds = {
///         User = { tag = "user", type_name = MyUserKind },
///         Organization = { tag = "org", alias = OrgUuid },
///     },
/// }
///
/// // This generates types with the specified names:
/// assert_eq!(MyUserKind::tag(), "user");
/// assert_eq!(OrganizationKind::tag(), "org");
///
/// let user_uuid = UserUuid::new_v4();
/// let org_uuid = OrgUuid::new_v4();
/// ```
///
/// # Global settings
///
/// This macro accepts a list of global settings under `settings`:
///
/// - `newtype_uuid_crate`: The name of the `newtype-uuid` crate (an identifier). Defaults to `newtype_uuid`.
/// - `schemars08`: If defined, generates schemars support for the given types.
#[proc_macro]
pub fn impl_typed_uuid_kinds(input: TokenStream) -> TokenStream {
    internals::impl_typed_uuid_kinds(input.into())
        .into_token_stream()
        .into()
}
