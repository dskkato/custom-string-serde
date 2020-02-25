#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/serde_test.rs");
    t.pass("tests/size_test.rs");
}
