//! Basic usage example for the `impl_typed_uuid_kinds` macro.
//!
//! This example demonstrates how to use the macro to generate typed UUID kinds
//! and their corresponding type aliases.

use newtype_uuid::TypedUuidKind;
use newtype_uuid_macros::impl_typed_uuid_kinds;

// Generate typed UUID kinds for different entities in our application.
impl_typed_uuid_kinds! {
    kinds = {
        User = {},
        Organization = {},
        Project = {},
    }
}

// The above macro generates:
//
// * pub enum UserKind {}
// * pub type UserUuid = TypedUuid<UserKind>;
// * pub type OrganizationKind {}
// * pub type OrganizationUuid = TypedUuid<OrganizationKind>;
// * pub type ProjectKind {}
// * pub type ProjectUuid = TypedUuid<ProjectKind>;

fn main() {
    // Create some UUIDs of different types.
    let user_uuid = UserUuid::new_v4();
    let org_uuid = OrganizationUuid::new_v4();
    let project_uuid = ProjectUuid::new_v4();

    // Print the UUIDs and their tags.
    println!("User UUID: {} (tag: {})", user_uuid, UserKind::tag());
    println!(
        "Organization UUID: {} (tag: {})",
        org_uuid,
        OrganizationKind::tag()
    );
    println!(
        "Project UUID: {} (tag: {})",
        project_uuid,
        ProjectKind::tag()
    );

    // The compiler ensures type safety -- you can't accidentally mix up types.
    // This would be a compile error:
    // let _error: UserUuid = typed_org;  // Error: mismatched types
}
