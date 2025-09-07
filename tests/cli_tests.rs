#[test]
fn cli_suite() {
    trycmd::TestCases::new().case("tests/cmd/*.toml");
}
