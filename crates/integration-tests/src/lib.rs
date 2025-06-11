//! Integration tests.

#[cfg(all(test, feature = "internal-schemars08-tests"))]
mod json_schema;
#[cfg(all(test, feature = "internal-proptest1-tests"))]
mod proptests;
#[cfg(test)]
mod ui;
