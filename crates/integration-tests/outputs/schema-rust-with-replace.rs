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
    pub id: ::newtype_uuid::TypedUuid<::my_crate::MyKind>,
}
impl From<&MyPathStruct> for MyPathStruct {
    fn from(value: &MyPathStruct) -> Self {
        value.clone()
    }
}
