/// These aren't really tests. Expansion is used here to automate `cargo expand` and validate it
/// against a previously expanded version to better understand generated code and ensure
/// nothing's missing out.
///
/// Real tests are all under `/groom_tests`.

#[test]
pub fn expansion_tests() {
    macrotest::expand("tests/expand/01-simple-rest.rs");
    //macrotest::expand_without_refresh("tests/expand/01-simple-rest.rs");
}
