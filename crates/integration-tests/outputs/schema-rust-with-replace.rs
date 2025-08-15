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
///      "$ref": "#/definitions/MyUuid"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MyPathStruct {
    pub id: ::my_crate::types::MyUuid,
}
impl ::std::convert::From<&MyPathStruct> for MyPathStruct {
    fn from(value: &MyPathStruct) -> Self {
        value.clone()
    }
}
