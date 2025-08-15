use my_custom_uuid::TypedUuidKind;
use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    settings = {
        newtype_uuid_crate = my_custom_uuid,
    },
    kinds = {
        User = {},
        Organization = {},
        Product = {},
    }
}

fn main() {
    // Test that the generated types exist and work with custom crate alias
    let _user_kind_tag = UserKind::tag();
    let _org_kind_tag = OrganizationKind::tag();
    let _product_kind_tag = ProductKind::tag();

    // Test type aliases exist
    let _user_uuid: UserUuid;
    let _org_uuid: OrganizationUuid;
    let _product_uuid: ProductUuid;

    // Test that tags are properly snake_cased
    assert_eq!(UserKind::tag().as_str(), "user");
    assert_eq!(OrganizationKind::tag().as_str(), "organization");
    assert_eq!(ProductKind::tag().as_str(), "product");
}
