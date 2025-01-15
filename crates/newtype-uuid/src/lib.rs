//! A newtype wrapper around [`Uuid`].
//!
//! # Motivation
//!
//! Many large systems use UUIDs as unique identifiers for various entities. However, the [`Uuid`]
//! type does not carry information about the kind of entity it identifies, which can lead to mixing
//! up different types of UUIDs at runtime.
//!
//! This crate provides a wrapper type around [`Uuid`] that allows you to specify the kind of entity
//! the UUID identifies.
//!
//! # Example
//!
//! ```rust
//! use newtype_uuid::{GenericUuid, TypedUuid, TypedUuidKind, TypedUuidTag};
//!
//! // First, define a type that represents the kind of UUID this is.
//! enum MyKind {}
//!
//! impl TypedUuidKind for MyKind {
//!     fn tag() -> TypedUuidTag {
//!         // Tags are required to be ASCII identifiers, with underscores
//!         // and dashes also supported. The validity of a tag can be checked
//!         // at compile time by assigning it to a const, like so:
//!         const TAG: TypedUuidTag = TypedUuidTag::new("my_kind");
//!         TAG
//!     }
//! }
//!
//! // Now, a UUID can be created with this kind.
//! let uuid: TypedUuid<MyKind> = "dffc3068-1cd6-47d5-b2f3-636b41b07084".parse().unwrap();
//!
//! // The Display (and therefore ToString) impls still show the same value.
//! assert_eq!(uuid.to_string(), "dffc3068-1cd6-47d5-b2f3-636b41b07084");
//!
//! // The Debug impl will show the tag as well.
//! assert_eq!(format!("{:?}", uuid), "dffc3068-1cd6-47d5-b2f3-636b41b07084 (my_kind)");
//! ```
//!
//! If you have a large number of UUID kinds, consider defining a macro for your purposes. An
//! example macro:
//!
//! ```rust
//! # use newtype_uuid::{TypedUuidKind, TypedUuidTag};
//! macro_rules! impl_typed_uuid_kind {
//!     ($($kind:ident => $tag:literal),* $(,)?) => {
//!         $(
//!             pub enum $kind {}
//!
//!             impl TypedUuidKind for $kind {
//!                 #[inline]
//!                 fn tag() -> TypedUuidTag {
//!                     const TAG: TypedUuidTag = TypedUuidTag::new($tag);
//!                     TAG
//!                 }
//!             }
//!         )*
//!     };
//! }
//!
//! // Invoke this macro with:
//! impl_typed_uuid_kind! {
//!     Kind1 => "kind1",
//!     Kind2 => "kind2",
//! }
//! ```
//!
//! # Implementations
//!
//! In general, [`TypedUuid`] uses the same wire and serialization formats as [`Uuid`]. This means
//! that persistent representations of [`TypedUuid`] are the same as [`Uuid`]; [`TypedUuid`] is
//! intended to be helpful within Rust code, not across serialization boundaries.
//!
//! - The `Display` and `FromStr` impls are forwarded to the underlying [`Uuid`].
//! - If the `serde` feature is enabled, `TypedUuid` will serialize and deserialize using the same
//!   format as [`Uuid`].
//! - If the `schemars08` feature is enabled, [`TypedUuid`] will implement `JsonSchema` if the
//!   corresponding [`TypedUuidKind`] implements `JsonSchema`.
//!
//! To abstract over typed and untyped UUIDs, the [`GenericUuid`] trait is provided. This trait also
//! permits conversions between typed and untyped UUIDs.
//!
//! # Dependencies
//!
//! - The only required dependency is the [`uuid`] crate. Optional features may add further
//!   dependencies.
//!
//! # Features
//!
//! - `default`: Enables default features in the newtype-uuid crate.
//! - `std`: Enables the use of the standard library. *Enabled by default.*
//! - `serde`: Enables serialization and deserialization support via Serde. *Not enabled by
//!   default.*
//! - `v4`: Enables the `new_v4` method for generating UUIDs. *Not enabled by default.*
//! - `schemars08`: Enables support for generating JSON schemas via schemars 0.8. *Not enabled by
//!   default.* Note that the format of the generated schema is **not currently part** of the stable
//!   API, though we hope to stabilize it in the future.
//! - `proptest1`: Enables support for generating `proptest::Arbitrary` instances of UUIDs. *Not enabled by default.*
//!
//! # Minimum supported Rust version (MSRV)
//!
//! The MSRV of this crate is **Rust 1.67.** In general, this crate will follow the MSRV of the
//! underlying `uuid` crate or of dependencies, with an aim to be conservative.
//!
//! Within the 1.x series, MSRV updates will be accompanied by a minor version bump. The MSRVs for
//! each minor version are:
//!
//! * Version **1.0.x**: Rust 1.60.
//! * Version **1.1.x**: Rust 1.61. This permits `TypedUuid<T>` to have `const fn` methods.
//! * Version **1.2.x**: Rust 1.67, required by some dependency updates.
//!
//! # Alternatives
//!
//! - [`typed-uuid`](https://crates.io/crates/typed-uuid): generally similar, but with a few design
//!   decisions that are different.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg, doc_auto_cfg))]

use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    str::FromStr,
};
use uuid::{Uuid, Version};

/// A UUID with type-level information about what it's used for.
///
/// For more, see [the library documentation](crate).
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent, bound = ""))]
pub struct TypedUuid<T: TypedUuidKind> {
    uuid: Uuid,
    _phantom: PhantomData<T>,
}

impl<T: TypedUuidKind> TypedUuid<T> {
    /// The 'nil UUID' (all zeros).
    ///
    /// The nil UUID is a special form of UUID that is specified to have all
    /// 128 bits set to zero.
    ///
    /// # References
    ///
    /// * [Nil UUID in RFC4122](https://tools.ietf.org/html/rfc4122.html#section-4.1.7)
    #[inline]
    #[must_use]
    pub const fn nil() -> Self {
        Self {
            uuid: Uuid::nil(),
            _phantom: PhantomData,
        }
    }

    /// The 'max UUID' (all ones).
    ///
    /// The max UUID is a special form of UUID that is specified to have all
    /// 128 bits set to one.
    ///
    /// # References
    ///
    /// * [Max UUID in Draft RFC: New UUID Formats, Version 4](https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04#section-5.4)
    #[inline]
    #[must_use]
    pub const fn max() -> Self {
        Self {
            uuid: Uuid::max(),
            _phantom: PhantomData,
        }
    }

    /// Creates a UUID from four field values.
    #[inline]
    #[must_use]
    pub const fn from_fields(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> Self {
        Self {
            uuid: Uuid::from_fields(d1, d2, d3, &d4),
            _phantom: PhantomData,
        }
    }

    /// Creates a UUID from four field values in little-endian order.
    ///
    /// The bytes in the `d1`, `d2` and `d3` fields will be flipped to convert into big-endian
    /// order. This is based on the endianness of the UUID, rather than the target environment so
    /// bytes will be flipped on both big and little endian machines.
    #[inline]
    #[must_use]
    pub const fn from_fields_le(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> Self {
        Self {
            uuid: Uuid::from_fields_le(d1, d2, d3, &d4),
            _phantom: PhantomData,
        }
    }

    /// Creates a UUID from a 128bit value.
    #[inline]
    #[must_use]
    pub const fn from_u128(value: u128) -> Self {
        Self {
            uuid: Uuid::from_u128(value),
            _phantom: PhantomData,
        }
    }

    /// Creates a UUID from a 128bit value in little-endian order.
    ///
    /// The entire value will be flipped to convert into big-endian order. This is based on the
    /// endianness of the UUID, rather than the target environment so bytes will be flipped on both
    /// big and little endian machines.
    #[inline]
    #[must_use]
    pub const fn from_u128_le(value: u128) -> Self {
        Self {
            uuid: Uuid::from_u128_le(value),
            _phantom: PhantomData,
        }
    }

    /// Creates a UUID from two 64bit values.
    #[inline]
    #[must_use]
    pub const fn from_u64_pair(d1: u64, d2: u64) -> Self {
        Self {
            uuid: Uuid::from_u64_pair(d1, d2),
            _phantom: PhantomData,
        }
    }

    /// Creates a UUID using the supplied bytes.
    #[inline]
    #[must_use]
    pub const fn from_bytes(bytes: uuid::Bytes) -> Self {
        Self {
            uuid: Uuid::from_bytes(bytes),
            _phantom: PhantomData,
        }
    }

    /// Creates a UUID using the supplied bytes in little-endian order.
    ///
    /// The individual fields encoded in the buffer will be flipped.
    #[inline]
    #[must_use]
    pub const fn from_bytes_le(bytes: uuid::Bytes) -> Self {
        Self {
            uuid: Uuid::from_bytes_le(bytes),
            _phantom: PhantomData,
        }
    }

    /// Creates a new, random UUID v4 of this type.
    #[inline]
    #[cfg(feature = "v4")]
    #[must_use]
    pub fn new_v4() -> Self {
        Self::from_untyped_uuid(Uuid::new_v4())
    }

    /// Returns the version number of the UUID.
    ///
    /// This represents the algorithm used to generate the value.
    /// This method is the future-proof alternative to [`Self::get_version`].
    ///
    /// # References
    ///
    /// * [Version Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.2)
    #[inline]
    pub const fn get_version_num(&self) -> usize {
        self.uuid.get_version_num()
    }

    /// Returns the version of the UUID.
    ///
    /// This represents the algorithm used to generate the value.
    /// If the version field doesn't contain a recognized version then `None`
    /// is returned. If you're trying to read the version for a future extension
    /// you can also use [`Uuid::get_version_num`] to unconditionally return a
    /// number. Future extensions may start to return `Some` once they're
    /// standardized and supported.
    ///
    /// # References
    ///
    /// * [Version Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.2)
    #[inline]
    pub fn get_version(&self) -> Option<Version> {
        self.uuid.get_version()
    }
}

// ---
// Trait impls
// ---

impl<T: TypedUuidKind> PartialEq for TypedUuid<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.uuid.eq(&other.uuid)
    }
}

impl<T: TypedUuidKind> Eq for TypedUuid<T> {}

impl<T: TypedUuidKind> PartialOrd for TypedUuid<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.uuid.cmp(&other.uuid))
    }
}

impl<T: TypedUuidKind> Ord for TypedUuid<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl<T: TypedUuidKind> Hash for TypedUuid<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<T: TypedUuidKind> fmt::Debug for TypedUuid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.uuid.fmt(f)?;
        write!(f, " ({})", T::tag())
    }
}

impl<T: TypedUuidKind> fmt::Display for TypedUuid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.uuid.fmt(f)
    }
}

impl<T: TypedUuidKind> Clone for TypedUuid<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: TypedUuidKind> Copy for TypedUuid<T> {}

impl<T: TypedUuidKind> FromStr for TypedUuid<T> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s).map_err(|error| ParseError {
            error,
            tag: T::tag(),
        })?;
        Ok(Self::from_untyped_uuid(uuid))
    }
}

impl<T: TypedUuidKind> Default for TypedUuid<T> {
    #[inline]
    fn default() -> Self {
        Self::from_untyped_uuid(Uuid::default())
    }
}

#[cfg(feature = "schemars08")]
mod schemars08_imp {
    use super::*;
    use schemars::JsonSchema;

    /// Implements `JsonSchema` for `TypedUuid<T>`, if `T` implements `JsonSchema`.
    ///
    /// * `schema_name` is set to `"TypedUuidFor"`, concatenated by the schema name of `T`.
    /// * `schema_id` is set to `format!("newtype_uuid::TypedUuid<{}>", T::schema_id())`.
    /// * `json_schema` is the same as the one for `Uuid`.
    impl<T> JsonSchema for TypedUuid<T>
    where
        T: TypedUuidKind + JsonSchema,
    {
        #[inline]
        fn schema_name() -> String {
            format!("TypedUuidFor{}", T::schema_name())
        }

        #[inline]
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Owned(format!("newtype_uuid::TypedUuid<{}>", T::schema_id()))
        }

        #[inline]
        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            Uuid::json_schema(gen)
        }
    }
}

#[cfg(feature = "proptest1")]
mod proptest1_imp {
    use super::*;
    use proptest::{
        arbitrary::{any, Arbitrary},
        strategy::{BoxedStrategy, Strategy},
    };

    /// Parameters for use with `proptest` instances.
    ///
    /// This is currently not exported as a type because it has no options. But
    /// it's left in as an extension point for the future.
    #[derive(Clone, Debug, Default)]
    pub struct TypedUuidParams(());

    /// Generates random `TypedUuid<T>` instances.
    ///
    /// Currently, this always returns a version 4 UUID. Support for other kinds
    /// of UUIDs might be added via [`Self::Parameters`] in the future.
    impl<T> Arbitrary for TypedUuid<T>
    where
        T: TypedUuidKind,
    {
        type Parameters = TypedUuidParams;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            let bytes = any::<[u8; 16]>();
            bytes
                .prop_map(|b| {
                    let uuid = uuid::Builder::from_random_bytes(b).into_uuid();
                    TypedUuid::<T>::from_untyped_uuid(uuid)
                })
                .boxed()
        }
    }
}

/// Represents marker types that can be used as a type parameter for [`TypedUuid`].
///
/// Generally, an implementation of this will be a zero-sized type that can never be constructed. An
/// empty struct or enum works well for this.
///
/// # Implementations
///
/// If the `schemars08` feature is enabled, and [`JsonSchema`] is implemented for a kind `T`, then
/// [`TypedUuid`]`<T>` will also implement [`JsonSchema`].
///
/// # Notes
///
/// If you have a large number of UUID kinds, it can be repetitive to implement this trait for each
/// kind. Here's a template for a macro that can help:
///
/// ```
/// use newtype_uuid::{TypedUuidKind, TypedUuidTag};
///
/// macro_rules! impl_typed_uuid_kind {
///     ($($kind:ident => $tag:literal),* $(,)?) => {
///         $(
///             pub enum $kind {}
///
///             impl TypedUuidKind for $kind {
///                 #[inline]
///                 fn tag() -> TypedUuidTag {
///                     const TAG: TypedUuidTag = TypedUuidTag::new($tag);
///                     TAG
///                 }
///             }
///         )*
///     };
/// }
///
/// // Invoke this macro with:
/// impl_typed_uuid_kind! {
///     Kind1 => "kind1",
///     Kind2 => "kind2",
/// }
/// ```
///
/// [`JsonSchema`]: schemars::JsonSchema
pub trait TypedUuidKind: Send + Sync + 'static {
    /// Returns the corresponding tag for this kind.
    ///
    /// The tag forms a runtime representation of this type.
    ///
    /// The tag is required to be a static string.
    fn tag() -> TypedUuidTag;
}

/// Describes what kind of [`TypedUuid`] something is.
///
/// This is the runtime equivalent of [`TypedUuidKind`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedUuidTag(&'static str);

impl TypedUuidTag {
    /// Creates a new `TypedUuidTag` from a static string.
    ///
    /// The string must be non-empty, and consist of:
    /// - ASCII letters
    /// - digits (only after the first character)
    /// - underscores
    /// - hyphens (only after the first character)
    ///
    /// # Panics
    ///
    /// Panics if the above conditions aren't met. Use [`Self::try_new`] to handle errors instead.
    #[must_use]
    pub const fn new(tag: &'static str) -> Self {
        match Self::try_new_impl(tag) {
            Ok(tag) => tag,
            Err(message) => panic!("{}", message),
        }
    }

    /// Attempts to create a new `TypedUuidTag` from a static string.
    ///
    /// The string must be non-empty, and consist of:
    /// - ASCII letters
    /// - digits (only after the first character)
    /// - underscores
    /// - hyphens (only after the first character)
    ///
    /// # Errors
    ///
    /// Returns a [`TagError`] if the above conditions aren't met.
    pub const fn try_new(tag: &'static str) -> Result<Self, TagError> {
        match Self::try_new_impl(tag) {
            Ok(tag) => Ok(tag),
            Err(message) => Err(TagError {
                input: tag,
                message,
            }),
        }
    }

    const fn try_new_impl(tag: &'static str) -> Result<Self, &'static str> {
        if tag.is_empty() {
            return Err("tag must not be empty");
        }

        let bytes = tag.as_bytes();
        if !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
            return Err("first character of tag must be an ASCII letter or underscore");
        }

        let mut bytes = match bytes {
            [_, rest @ ..] => rest,
            [] => panic!("already checked that it's non-empty"),
        };
        while let [rest @ .., last] = &bytes {
            if !(last.is_ascii_alphanumeric() || *last == b'_' || *last == b'-') {
                break;
            }
            bytes = rest;
        }

        if !bytes.is_empty() {
            return Err("tag must only contain ASCII letters, digits, underscores, or hyphens");
        }

        Ok(Self(tag))
    }

    /// Returns the tag as a string.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for TypedUuidTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl AsRef<str> for TypedUuidTag {
    fn as_ref(&self) -> &str {
        self.0
    }
}

/// An error that occurred while creating a [`TypedUuidTag`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct TagError {
    /// The input string.
    pub input: &'static str,

    /// The error message.
    pub message: &'static str,
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error creating tag from '{}': {}",
            self.input, self.message
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TagError {}

/// An error that occurred while parsing a [`TypedUuid`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ParseError {
    /// The underlying error.
    pub error: uuid::Error,

    /// The tag of the UUID that failed to parse.
    pub tag: TypedUuidTag,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error parsing UUID ({})", self.tag)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// A trait abstracting over typed and untyped UUIDs.
///
/// This can be used to write code that's generic over [`TypedUuid`], [`Uuid`], and other types that
/// may wrap [`TypedUuid`] (due to e.g. orphan rules).
///
/// This trait is similar to `From`, but a bit harder to get wrong -- in general, the conversion
/// from and to untyped UUIDs should be careful and explicit.
pub trait GenericUuid {
    /// Creates a new instance of `Self` from an untyped [`Uuid`].
    #[must_use]
    fn from_untyped_uuid(uuid: Uuid) -> Self
    where
        Self: Sized;

    /// Converts `self` into an untyped [`Uuid`].
    #[must_use]
    fn into_untyped_uuid(self) -> Uuid
    where
        Self: Sized;

    /// Returns the inner [`Uuid`].
    ///
    /// Generally, [`into_untyped_uuid`](Self::into_untyped_uuid) should be preferred. However,
    /// in some cases it may be necessary to use this method to satisfy lifetime constraints.
    fn as_untyped_uuid(&self) -> &Uuid;
}

impl GenericUuid for Uuid {
    #[inline]
    fn from_untyped_uuid(uuid: Uuid) -> Self {
        uuid
    }

    #[inline]
    fn into_untyped_uuid(self) -> Uuid {
        self
    }

    #[inline]
    fn as_untyped_uuid(&self) -> &Uuid {
        self
    }
}

impl<T: TypedUuidKind> GenericUuid for TypedUuid<T> {
    #[inline]
    fn from_untyped_uuid(uuid: Uuid) -> Self {
        Self {
            uuid,
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn into_untyped_uuid(self) -> Uuid {
        self.uuid
    }

    #[inline]
    fn as_untyped_uuid(&self) -> &Uuid {
        &self.uuid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_tags() {
        for &valid_tag in &[
            "a", "a-", "a_", "a-b", "a_b", "a1", "a1-", "a1_", "a1-b", "a1_b", "_a",
        ] {
            TypedUuidTag::try_new(valid_tag).expect("tag is valid");
            // Should not panic
            _ = TypedUuidTag::new(valid_tag);
        }

        for invalid_tag in &["", "1", "-", "a1b!", "a1-b!", "a1_b:", "\u{1f4a9}"] {
            TypedUuidTag::try_new(invalid_tag).unwrap_err();
        }
    }

    // This test just ensures that `GenericUuid` is object-safe.
    #[test]
    #[cfg(all(feature = "v4", feature = "std"))]
    fn test_generic_uuid_object_safe() {
        let uuid = Uuid::new_v4();
        let box_uuid = Box::new(uuid) as Box<dyn GenericUuid>;
        assert_eq!(box_uuid.as_untyped_uuid(), &uuid);
    }
}
