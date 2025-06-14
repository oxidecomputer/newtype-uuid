#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/fixtures/valid/*.rs");
    t.compile_fail("tests/fixtures/invalid/*.rs");
}
