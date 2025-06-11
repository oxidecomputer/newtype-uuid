//! Procedural macros for newtype-uuid
//!
//! This crate provides procedural macros to help with creating typed UUID kinds
//! with compile-time type safety.

mod internals;

use proc_macro::TokenStream;
use quote::ToTokens;

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
    internals::impl_typed_uuid_kinds(input.into())
        .into_token_stream()
        .into()
}
