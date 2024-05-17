/// These aren't really tests. Expansion is used here to automate `cargo expand` and validate it
/// against a previously expanded version to better understand generated code and ensure
/// nothing's missing out.
///
/// Real tests are all under `/groom_tests`.

#[test]
pub fn expand_00_simple_rest() {
    macrotest::expand("tests/expand/00-simple-rest.rs");
}

#[test]
pub fn expand_01_response() {
    macrotest::expand("tests/expand/01-response.rs");
}
