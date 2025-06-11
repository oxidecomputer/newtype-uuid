set positional-arguments

# Note: help messages should be 1 line long as required by just.

# Print a help message.
help:
    just --list

excluded_features_default := "internal-schemars08-tests internal-proptest1-tests"
excluded_features_no_std := "schemars08 serde default std alloc v4 proptest1"

# Run `cargo hack --feature-powerset` on crates
powerset *args:
    #!/usr/bin/env bash
    excluded_features="{{excluded_features_default}}"
    NEXTEST_NO_TESTS=pass cargo hack --feature-powerset --workspace --exclude-features "${excluded_features// /,}" "$@"

# Run `cargo hack` with no-std-compatible features.
powerset-no-std *args:
    #!/usr/bin/env bash
    excluded_features="{{excluded_features_default}} {{excluded_features_no_std}}"
    NEXTEST_NO_TESTS=pass cargo hack --feature-powerset --workspace --exclude-features "${excluded_features// /,}" "$@"

# Build docs for crates and direct dependencies
rustdoc *args:
    @cargo tree --depth 1 -e normal --prefix none --workspace --all-features \
        | gawk '{ gsub(" v", "@", $0); printf("%s\n", $1); }' \
        | xargs printf -- '-p %s\n' \
        | RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS='--cfg=doc_cfg' xargs cargo doc --no-deps --all-features {{args}}

# Generate README.md files using `cargo-sync-rdme`.
generate-readmes:
    cargo sync-rdme --toolchain nightly --workspace --all-features

# Run cargo release in CI.
ci-cargo-release:
    # cargo-release requires a release off a branch.
    git checkout -B to-release
    cargo release publish --publish --execute --no-confirm --workspace
    git checkout -
    git branch -D to-release
