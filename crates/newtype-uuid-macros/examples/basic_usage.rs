//! Basic usage example for the `impl_typed_uuid_kinds` macro.
//!
//! This example demonstrates how to use the macro to generate typed UUID kinds
//! and their corresponding type aliases.

use newtype_uuid::{GenericUuid, TypedUuidKind};
use newtype_uuid_macros::impl_typed_uuid_kinds;

// Generate typed UUID kinds for different entities in our application
impl_typed_uuid_kinds! {
    kinds = {
        User = {},
        Organization = {},
        Project = {},
        Task = {},
    }
}

fn main() {
    // Create some UUIDs of different types
    let user_uuid = UserUuid::from_untyped_uuid(uuid::Uuid::new_v4());
    let org_uuid = OrganizationUuid::from_untyped_uuid(uuid::Uuid::new_v4());
    let project_uuid = ProjectUuid::from_untyped_uuid(uuid::Uuid::new_v4());
    let task_uuid = TaskUuid::from_untyped_uuid(uuid::Uuid::new_v4());

    // Print the UUIDs and their types
    println!(
        "User UUID: {} (tag: {})",
        user_uuid.into_untyped_uuid(),
        UserKind::tag()
    );
    println!(
        "Organization UUID: {} (tag: {})",
        org_uuid.into_untyped_uuid(),
        OrganizationKind::tag()
    );
    println!(
        "Project UUID: {} (tag: {})",
        project_uuid.into_untyped_uuid(),
        ProjectKind::tag()
    );
    println!(
        "Task UUID: {} (tag: {})",
        task_uuid.into_untyped_uuid(),
        TaskKind::tag()
    );

    // Demonstrate that the types are different even with same underlying UUID
    let base_uuid = uuid::Uuid::new_v4();
    let typed_user = UserUuid::from_untyped_uuid(base_uuid);
    let typed_org = OrganizationUuid::from_untyped_uuid(base_uuid);

    println!("\nSame underlying UUID, different types:");
    println!(
        "User: {} (tag: {})",
        typed_user.into_untyped_uuid(),
        UserKind::tag()
    );
    println!(
        "Org:  {} (tag: {})",
        typed_org.into_untyped_uuid(),
        OrganizationKind::tag()
    );

    // The compiler ensures type safety - you can't accidentally mix them up
    // This would be a compile error:
    // let _error: UserUuid = typed_org;  // Error: mismatched types

    // Demonstrate some common operations
    println!("\nCommon operations:");

    // Tags are validated at compile time
    assert_eq!(UserKind::tag().as_str(), "user");
    assert_eq!(OrganizationKind::tag().as_str(), "organization");

    // Types can be compared for equality
    let user1 = UserUuid::from_untyped_uuid(base_uuid);
    let user2 = UserUuid::from_untyped_uuid(base_uuid);
    assert_eq!(user1.into_untyped_uuid(), user2.into_untyped_uuid());

    println!("✓ All assertions passed!");
}
