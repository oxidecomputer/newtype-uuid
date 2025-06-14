use my_custom_uuid::TypedUuidKind;
use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    settings = {
        newtype_uuid_crate = my_custom_uuid,
        schemars08 = {
            schemars_crate = custom_schemars,
            attrs = [#[cfg(feature = "internal-schemars08-tests")]],
            rust_type = {
                crate = "my-api-service",
                version = "2.1.0",
                path = "my_api_service::models",
            },
        },
    },
    kinds = {
        Account = {},
        Transaction = {},
        ApiKey = {},
    }
}

fn main() {
    // Test that the generated types exist and work with custom crate alias and schemars
    let _account_kind_tag = AccountKind::tag();
    let _transaction_kind_tag = TransactionKind::tag();
    let _api_key_kind_tag = ApiKeyKind::tag();

    // Test type aliases exist
    let _account_uuid: AccountUuid;
    let _transaction_uuid: TransactionUuid;
    let _api_key_uuid: ApiKeyUuid;

    // Test that tags are properly snake_cased
    assert_eq!(AccountKind::tag().as_str(), "account");
    assert_eq!(TransactionKind::tag().as_str(), "transaction");
    assert_eq!(ApiKeyKind::tag().as_str(), "api_key");
}
