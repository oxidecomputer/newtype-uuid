//! Integration tests.

#[cfg(all(test, feature = "internal-schemars08-tests"))]
mod json_schema;
#[cfg(all(test, feature = "internal-proptest1-tests"))]
mod proptests;
pub mod snapshot_utils;
#[cfg(test)]
mod ui;

// These deps are used only by cfg(test) modules or trybuild fixtures, so mark
// them as used to silence the cargo::unused_dependencies lint. newtype-uuid
// can't be a dev-dep, since internal-schemars08-tests must enable
// newtype-uuid/schemars08 in trybuild's generated project. typify and
// test-strategy are optional, and optional dev-deps aren't allowed.
use newtype_uuid as _;
#[cfg(feature = "internal-proptest1-tests")]
use test_strategy as _;
#[cfg(feature = "internal-schemars08-tests")]
use typify as _;
