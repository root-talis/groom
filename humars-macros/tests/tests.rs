#[test]
pub fn openapi_gen_test() {
    macrotest::expand("tests/expand/01-simple-rest.rs");
    //macrotest::expand_without_refresh("tests/expand/01-simple-rest.rs");
}
