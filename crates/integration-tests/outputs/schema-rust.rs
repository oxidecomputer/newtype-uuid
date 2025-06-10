/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`MyPathStruct`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "MyPathStruct",
///  "type": "object",
///  "required": [
///    "id"
///  ],
///  "properties": {
///    "id": {
///      "$ref": "#/definitions/TypedUuidForMyKind"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MyPathStruct {
    pub id: TypedUuidForMyKind,
}
impl ::std::convert::From<&MyPathStruct> for MyPathStruct {
    fn from(value: &MyPathStruct) -> Self {
        value.clone()
    }
}
///`TypedUuidForMyKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "format": "uuid"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TypedUuidForMyKind(pub ::uuid::Uuid);
impl ::std::ops::Deref for TypedUuidForMyKind {
    type Target = ::uuid::Uuid;
    fn deref(&self) -> &::uuid::Uuid {
        &self.0
    }
}
impl ::std::convert::From<TypedUuidForMyKind> for ::uuid::Uuid {
    fn from(value: TypedUuidForMyKind) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypedUuidForMyKind> for TypedUuidForMyKind {
    fn from(value: &TypedUuidForMyKind) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMyKind {
    fn from(value: ::uuid::Uuid) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for TypedUuidForMyKind {
    type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for TypedUuidForMyKind {
    type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for TypedUuidForMyKind {
    type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for TypedUuidForMyKind {
    type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for TypedUuidForMyKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
