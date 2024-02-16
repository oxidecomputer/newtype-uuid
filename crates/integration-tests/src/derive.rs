use newtype_uuid::{TypedUuid, TypedUuidKind};

#[test]
fn derive() {
    #[derive(TypedUuidKind)]
    #[typed_uuid_tag("my_kind")]
    struct MyKind;

    let uuid: TypedUuid<MyKind> = "dffc3068-1cd6-47d5-b2f3-636b41b07084".parse().unwrap();
    assert_eq!(uuid.to_string(), "dffc3068-1cd6-47d5-b2f3-636b41b07084");
    assert_eq!(
        format!("{:?}", uuid),
        "dffc3068-1cd6-47d5-b2f3-636b41b07084 (my_kind)"
    );
}

#[test]
fn default_name() {
    #[derive(TypedUuidKind)]
    struct DefaultNameKind;

    let uuid: TypedUuid<DefaultNameKind> = "dffc3068-1cd6-47d5-b2f3-636b41b07084".parse().unwrap();
    assert_eq!(uuid.to_string(), "dffc3068-1cd6-47d5-b2f3-636b41b07084");
    assert_eq!(
        format!("{:?}", uuid),
        "dffc3068-1cd6-47d5-b2f3-636b41b07084 (DefaultNameKind)"
    );
}
