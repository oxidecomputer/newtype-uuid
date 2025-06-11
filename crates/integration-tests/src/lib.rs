//! Integration tests.

#[cfg(all(test, feature = "internal-schemars08-tests"))]
mod json_schema;
#[cfg(all(test, feature = "internal-proptest1-tests"))]
mod proptests;
pub mod snapshot_utils;
#[cfg(test)]
mod ui;
