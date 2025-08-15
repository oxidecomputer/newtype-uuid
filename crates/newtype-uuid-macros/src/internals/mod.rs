//! Internal implementation for newtype-uuid-macros.
//!
//! This module is imported by both the proc macro and by snapshot tests.

mod error_store;
mod imp;

pub use imp::*;
