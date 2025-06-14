//! Procedural macro for [`newtype-uuid`](https://docs.rs/newtype-uuid).
//!
//! This crate provides a procedural macro to help with creating
//! [`newtype-uuid`](https://docs.rs/newtype-uuid) instances.
//!
//! For more information, see the documentation for [`impl_typed_uuid_kinds!`].
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use newtype_uuid::TypedUuidKind;
//! use newtype_uuid_macros::impl_typed_uuid_kinds;
//!
//! impl_typed_uuid_kinds! {
//!     kinds = {
//!         User = {},
//!         Organization = {},
//!     },
//! }
//!
//! // This generates empty UserKind and OrganizationKind enums implementing
//! // TypedUuidKind, with the tags "user" and "organization" respectively.
//! // Tags are snake_case versions of type names.
//! assert_eq!(UserKind::tag().as_str(), "user");
//! assert_eq!(OrganizationKind::tag().as_str(), "organization");
//!
//! // The macro also generates UserUuid and OrganizationUuid type aliases.
//! let user_uuid = UserUuid::new_v4();
//! let organization_uuid = OrganizationUuid::new_v4();
//! ```
//!
//! For more details and examples, see the documentation for
//! [`impl_typed_uuid_kinds!`].

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod internals;

use proc_macro::TokenStream;
use quote::ToTokens;

/// A function-like procedural macro for implementing typed UUID kinds.
///
/// This macro generates types that implement `TypedUuidKind` and corresponding
/// type aliases for `TypedUuid<T>`. The macro provides an easy way to generate
/// typed UUID kinds in bulk, and also to implement `JsonSchema` support with
/// schemars 0.8.
///
/// # Basic usage
///
/// Invoke the `impl_typed_uuid_kinds!` macro within a path that's publicly
/// visible. A dedicated crate for UUID kinds is recommended.
///
/// By default, for a kind `Foo`, this macro generates:
///
/// * A `FooKind` type that implements `TypedUuidKind`: `pub type FooKind {}`.
/// * A `FooUuid` type alias: `pub type FooUuid = TypedUuid<FooKind>;`.
///
/// ## Examples
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
/// assert_eq!(UserKind::tag().as_str(), "user");
/// assert_eq!(OrganizationKind::tag().as_str(), "organization");
///
/// // The macro also generates UserUuid and OrganizationUuid type aliases.
/// let user_uuid = UserUuid::new_v4();
/// let organization_uuid = OrganizationUuid::new_v4();
/// ```
///
/// * The generated `Kind` types always implement `Clone`, `Copy`, `Debug`,
///   `Eq`, and `PartialEq`.
/// * The `Kind` types are all empty (uninhabited) enums,
///   which means that values for these types cannot be created. (Using empty
///   enums is the recommended approach for `newtype-uuid`).
///
/// # Per-kind settings
///
/// Kinds can be customized with the following settings:
///
/// - `attrs`: Attributes to apply to the kind enum, such as `#[derive(SomeTrait)]`
///   or `#[cfg(feature = "some-feature")]`. *Optional, defaults to the global
///   `attrs`.*
/// - `tag`: The tag to use for the kind (a string literal). *Optional, defaults to
///   the snake_case version of the type name.*
/// - `type_name`: The name of the type to use for the kind (a Rust identifier).
///   *Optional, defaults to `{Name}Kind`*.
/// - `alias`: The name of the type alias to use for the kind (a Rust identifier).
///   *Optional, defaults to `{Name}Uuid`*.
///
/// Per-kind customizations should generally be unnecessary; the conventionally
/// generated type names should be sufficient for most use cases.
///
/// ## Examples
///
/// In this example, we derive [`daft::Diffable`] for `MyUserKind`.
///
/// ```
/// use daft::Diffable;
/// use newtype_uuid::TypedUuidKind;
/// use newtype_uuid_macros::impl_typed_uuid_kinds;
///
/// impl_typed_uuid_kinds! {
///     kinds = {
///         User = {
///             // This is a list of attributes surrounded by square brackets.
///             attrs = [#[derive(Diffable)]],
///             tag = "user",
///             type_name = MyUserKind,
///         },
///         Organization = { tag = "org", alias = OrgUuid },
///     },
/// }
///
/// // This generates types with the specified names:
/// assert_eq!(MyUserKind::tag().as_str(), "user");
/// assert_eq!(OrganizationKind::tag().as_str(), "org");
///
/// let user_uuid = UserUuid::new_v4();
/// let org_uuid = OrgUuid::new_v4();
///
/// // MyUserKind also implements `Diffable`.
/// static_assertions::assert_impl_all!(MyUserKind: Diffable);
/// ```
///
/// [`daft::Diffable`]: https://docs.rs/daft/latest/daft/trait.Diffable.html
///
/// # Global settings
///
/// This macro accepts global settings under a top-level `settings` map:
///
/// - `attrs`: A list of attributes to apply to all generated `Kind` types. Per-kind
///   attributes, if provided, will override these. *Optional, defaults to the empty list.*
/// - `newtype_uuid_crate`: The name of the `newtype-uuid` crate (a Rust identifier).
///   *Optional, defaults to `newtype_uuid`.*
/// - `schemars08`: If defined, generates JSON Schema support for the given types
///   using [`schemars` 0.8]. *Optional.*
///
/// ## JSON Schema support
///
/// If the `schemars08` global setting is defined, the macro generates JSON
/// Schema support for the `Kind` instances using [schemars 0.8]. Within
/// `settings.schemars08`, the options are:
///
/// - `attrs`: A list of attributes to apply to all generated `JsonSchema`
///   implementations. This will often be something like
///   `[#cfg(feature = "schemars-feature-name")]]`.
/// - `schemars_crate`: The name of the `schemars` crate (a Rust identifier).
///   Defaults to `schemars`.
/// - `rust_type`: If defined, adds the `x-rust-type` extension to the schema,
///   enabling automatic replacement with [`typify`] and other tools that
///   support it. *Optional, defaults to not adding the extension.*
///
///   Automatic replacement enables an end-to-end workflow where the same UUID
///   kinds can be shared between servers and clients.
///
///   `rust_type` is a map of the following options:
///
///   - `crate`: The crate name consumers will use to access these types. *Required.*
///   - `version`: The versions of the crate that automatic replacement is supported for. *Required.*
///   - `path`: The path to the module these types can be accessed from, including the crate name. *Required.*
///
///   For more about `x-rust-type`, see the [`typify` documentation].
///
/// [`schemars` 0.8]: https://docs.rs/schemars/0.8/schemars/
/// [`typify`]: https://docs.rs/typify
/// [`typify` documentation]: https://github.com/oxidecomputer/typify#rust---schema---rust
///
/// ## Examples
///
/// An example with all global settings defined:
///
/// ```
/// use daft::Diffable;
/// use newtype_uuid::TypedUuidKind;
/// use newtype_uuid_macros::impl_typed_uuid_kinds;
///
/// impl_typed_uuid_kinds! {
///     settings = {
///         attrs = [#[derive(Diffable)]],
///         newtype_uuid_crate = newtype_uuid,
///         schemars08 = {
///             attrs = [#[cfg(feature = "schemars")]],
///             schemars_crate = schemars,
///             rust_type = {
///                 crate = "my-crate",
///                 version = "*",
///                 path = "my_crate::types",
///             },
///         },
///     },
///     kinds = {
///         User = {},
///         Organization = {},
///         Project = {},
///     },
/// }
///
/// let user_uuid = UserUuid::new_v4();
/// let org_uuid = OrganizationUuid::new_v4();
/// let project_uuid = ProjectUuid::new_v4();
/// ```
#[proc_macro]
pub fn impl_typed_uuid_kinds(input: TokenStream) -> TokenStream {
    internals::impl_typed_uuid_kinds(input.into())
        .into_token_stream()
        .into()
}
