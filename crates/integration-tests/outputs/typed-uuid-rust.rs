///MyPathStruct
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "MyPathStruct",
  "type": "object",
  "required": [
    "id"
  ],
  "properties": {
    "id": {
      "$ref": "#/definitions/TypedUuidForMyKind"
    }
  }
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MyPathStruct {
    pub id: TypedUuidForMyKind,
}
impl From<&MyPathStruct> for MyPathStruct {
    fn from(value: &MyPathStruct) -> Self {
        value.clone()
    }
}
///TypedUuidForMyKind
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "string",
  "format": "uuid"
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypedUuidForMyKind(pub uuid::Uuid);
impl std::ops::Deref for TypedUuidForMyKind {
    type Target = uuid::Uuid;
    fn deref(&self) -> &uuid::Uuid {
        &self.0
    }
}
impl From<TypedUuidForMyKind> for uuid::Uuid {
    fn from(value: TypedUuidForMyKind) -> Self {
        value.0
    }
}
impl From<&TypedUuidForMyKind> for TypedUuidForMyKind {
    fn from(value: &TypedUuidForMyKind) -> Self {
        value.clone()
    }
}
impl From<uuid::Uuid> for TypedUuidForMyKind {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for TypedUuidForMyKind {
    type Err = <uuid::Uuid as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for TypedUuidForMyKind {
    type Error = <uuid::Uuid as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TypedUuidForMyKind {
    type Error = <uuid::Uuid as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TypedUuidForMyKind {
    type Error = <uuid::Uuid as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for TypedUuidForMyKind {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
