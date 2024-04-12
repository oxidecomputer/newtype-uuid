# Changelog

## [1.1.0] - 2024-04-12

### Added

- `TypedUuid::nil()` and `max()` constructors.
- `TypedUuid` is now `#[repr(transparent)]`.

### Changed

- MSRV updated to Rust 1.61.

## [1.0.1] - 2024-02-15

### Breaking changes

- `GenericUuid::to_generic_uuid` has been renamed to `GenericUuid::into_generic_uuid`.

### Changed

- Added `#[must_use]` annotations to constructors.

## [1.0.0] - 2024-02-15

(This version was not published due to a CI issue.)

## [0.3.0] - 2024-02-02

### Breaking changes

- `TypedUuidTag::try_new` returns a new `TagError` type rather than just a raw `&'static str`.

### Changed

- `TypedUuidTag::as_str` is now a `const fn`.

## [0.2.1] - 2024-02-02

Documentation improvements.

## [0.2.0] - 2024-02-01

### Breaking changes

- `TypedUuidTag`s are now required to be valid ASCII identifiers, with hyphens also supported.

### Misc

- Added `#[forbid(unsafe_code)]`. Thanks [Robert Lynch](https://github.com/rob0rt) for the contribution!

## [0.1.0] - 2024-01-30

Initial release.

[1.1.0]: https://github.com/oxidecomputer/newtype-uuid/releases/newtype-uuid-1.1.0
[1.0.1]: https://github.com/oxidecomputer/newtype-uuid/releases/newtype-uuid-1.0.1
[1.0.0]: https://github.com/oxidecomputer/newtype-uuid/releases/newtype-uuid-1.0.0
[0.3.0]: https://github.com/oxidecomputer/newtype-uuid/releases/newtype-uuid-0.3.0
[0.2.1]: https://github.com/oxidecomputer/newtype-uuid/releases/newtype-uuid-0.2.1
[0.2.0]: https://github.com/oxidecomputer/newtype-uuid/releases/newtype-uuid-0.2.0
[0.1.0]: https://github.com/oxidecomputer/newtype-uuid/releases/newtype-uuid-0.1.0
