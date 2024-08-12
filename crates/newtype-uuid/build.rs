fn main() {
    // Ideally this would be in the [lints] table in Cargo.toml to avoid a build script, but sadly
    // the MSRV for that is Rust 1.75.
    //
    // TODO: switch to [lints] configuration once the MSRV moves beyond that`.
    println!("cargo:rustc-check-cfg=cfg(doc_cfg)");
}
