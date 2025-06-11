# Custom Crate Tests

This crate contains integration tests specifically for the `newtype_uuid_crate` parameter of the `impl_typed_uuid_kinds!` macro.

When `newtype_uuid_crate = my_custom_uuid` is specified, the generated code should use `::my_custom_uuid::TypedUuidKind`, `::my_custom_uuid::TypedUuidTag`, etc. instead of the default `::newtype_uuid::` prefixes.
