#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fixtures/invalid/*.rs");
    t.pass("tests/fixtures/valid/*.rs");
}
