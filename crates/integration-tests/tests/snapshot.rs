use datatest_stable::Utf8Path;
use quote::ToTokens;
use syn::parse_quote;

// We need access to the proc-macro's internals for this test. An alternative
// would be to make this a unit test, but the integration test harness gives us
// automatic discovery of tests in the `fixtures/` directory, along with
// separate reporting for each test. Those are nice benefits.
#[path = "../../newtype-uuid-macros/src/internals/mod.rs"]
mod internals;

datatest_stable::harness! {
    // The pattern matches all .rs files that aren't .output.rs files.
    { test = valid_snapshot, root = "tests/fixtures/valid", pattern = r"^.*(?<!\.output)\.rs$" },
    { test = invalid_snapshot, root = "tests/fixtures/invalid", pattern = r"^.*(?<!\.output)\.rs$" },
}

/// Snapshot tests for valid inputs.
fn valid_snapshot(path: &Utf8Path, input: String) -> datatest_stable::Result<()> {
    let data = syn::parse_str::<syn::File>(&input)?;

    let output = run_macro(&data);
    assert_macro_output(path, output);

    Ok(())
}

/// Snapshot tests for invalid inputs.
fn invalid_snapshot(path: &Utf8Path, input: String) -> datatest_stable::Result<()> {
    let data = syn::parse_str::<syn::File>(&input)?;

    let output = run_macro(&data).map(|output| {
        // Drop the errors for snapshot tests -- only use the output.
        output.out
    });
    assert_macro_output(path, output);

    Ok(())
}

fn run_macro(data: &syn::File) -> impl Iterator<Item = internals::ImplKindsOutput> + '_ {
    // Look for invocations of impl_typed_uuid_kinds in the input.
    let items = data.items.iter().filter_map(|item| match item {
        syn::Item::Macro(item) => {
            let is_invocation = item
                .mac
                .path
                .segments
                .last()
                .map(|s| s.ident == "impl_typed_uuid_kinds")
                .unwrap_or(false);
            is_invocation.then_some(item)
        }
        _ => None,
    });

    // Run the macro on each item.
    items.map(|item| internals::impl_typed_uuid_kinds(item.mac.tokens.clone()))
}

fn assert_macro_output<T: ToTokens>(path: &Utf8Path, output: impl IntoIterator<Item = T>) {
    // Read the output as a `syn::File`.
    let output = output.into_iter();
    let file = parse_quote! {
        #(#output)*
    };

    // Format the output.
    let output = prettyplease::unparse(&file);

    // Compare the output with the snapshot. The new filename is the same as the
    // input, but with ".output.rs" at the end.
    let mut output_path = path.parent().unwrap().to_owned();
    output_path.push("output");
    output_path.push(path.file_name().unwrap());
    output_path.set_extension("output.rs");

    expectorate::assert_contents(&output_path, &output);
}
