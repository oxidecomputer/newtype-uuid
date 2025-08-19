//! End-to-end schema producer crate.
//!
//! This crate is part of the end-to-end JSON Schema generation and replacment
//! example in this repository. See the parent README for more.

pub use e2e_kinds::{OrganizationUuid, UserUuid};
use serde::{Deserialize, Serialize};

/// A simple type which uses the `UserUuid` and `OrganizationUuid` types defined
/// in this crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
pub struct Assignment {
    pub user_id: UserUuid,
    pub organization_id: OrganizationUuid,
}

#[cfg(test)]
mod tests {
    /// This test generates the JSON schema checked in at
    /// ../tests/output/assignment-schema.json.
    #[cfg(feature = "schemars08")]
    #[test]
    fn test_generate_assignment_schema() {
        let schema = schemars::schema_for!(super::Assignment);
        let schema_json = serde_json::to_string_pretty(&schema).unwrap();
        expectorate::assert_contents("tests/output/assignment-schema.json", &schema_json);
    }
}
