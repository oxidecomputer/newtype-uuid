use datatest_stable::Utf8Path;
use integration_tests::snapshot_utils;

datatest_stable::harness! {
    // The pattern matches all .rs files that aren't .output.rs files.
    { test = valid_snapshot, root = "tests/fixtures/valid", pattern = r"^.*(?<!\.output)\.rs$" },
    { test = invalid_snapshot, root = "tests/fixtures/invalid", pattern = r"^.*(?<!\.output)\.rs$" },
}

/// Snapshot tests for valid inputs.
fn valid_snapshot(path: &Utf8Path, input: String) -> datatest_stable::Result<()> {
    snapshot_utils::valid_snapshot(path, input)
}

/// Snapshot tests for invalid inputs.
fn invalid_snapshot(path: &Utf8Path, input: String) -> datatest_stable::Result<()> {
    snapshot_utils::invalid_snapshot(path, input)
}
