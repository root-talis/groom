/// Escapes a string for use in a JSON Pointer fragment according to RFC 6901.
///
/// Replaces:
/// - `~` with `~0`
/// - `/` with `~1`
///
/// # Examples
///
/// ```
/// let escaped = escape_json_pointer("users/~/test");
/// assert_eq!(escaped, "users~1~0~1test");
///
/// let escaped = escape_json_pointer("path//with/slashes");
/// assert_eq!(escaped, "path~1~1with~1slashes");
/// ```
pub fn escape_json_pointer(input: &str) -> String {
    String::from(input)
        .replace('~', "~0")
        .replace('/', "~1")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_escape_basic() {
        assert_eq!(escape_json_pointer("users"), "users");
        assert_eq!(escape_json_pointer(""), "");
    }

    #[test]
    fn test_escape_forward_slash() {
        assert_eq!(escape_json_pointer("users/active"), "users~1active");
        assert_eq!(escape_json_pointer("/root"), "~1root");
        assert_eq!(escape_json_pointer("multiple/slashes/here"), "multiple~1slashes~1here");
        assert_eq!(escape_json_pointer("path//double"), "path~1~1double");
    }

    #[test]
    fn test_escape_tilde() {
        assert_eq!(escape_json_pointer("tilde~test"), "tilde~0test");
        assert_eq!(escape_json_pointer("~~"), "~0~0");
        assert_eq!(escape_json_pointer("~"), "~0");
    }

    #[test]
    fn test_escape_mixed() {
        assert_eq!(escape_json_pointer("users/~/profile"), "users~1~0~1profile");
        assert_eq!(escape_json_pointer("~/test/"), "~0~1test~1");
        assert_eq!(escape_json_pointer("a~b/c~d/e"), "a~0b~1c~0d~1e");
    }

    #[test]
    fn test_escape_with_special_chars_not_escaped() {
        // These characters are NOT escaped in JSON Pointer
        assert_eq!(escape_json_pointer("users/{id}"), "users~1{id}");
        assert_eq!(escape_json_pointer("name with spaces"), "name with spaces");
        assert_eq!(escape_json_pointer("path?query=value"), "path?query=value");
        assert_eq!(escape_json_pointer("file[name].yaml"), "file[name].yaml");
    }

    #[test]
    fn test_escape_unicode() {
        assert_eq!(escape_json_pointer("ユーザー/データ"), "ユーザー~1データ");
        assert_eq!(escape_json_pointer("~/単体テスト"), "~0~1単体テスト");
    }

    #[test]
    fn test_build_openapi_ref() {
        // Realistic OpenAPI $ref examples
        let ref_path = format!("#/paths/~1users~1{{id}}");
        assert_eq!(ref_path, "#/paths/~1users~1{id}");
        
        let escaped = escape_json_pointer("paths/users/{id}");
        assert_eq!(escaped, "paths~1users~1{id}");
        
        let full_ref = format!("#/{}", escaped);
        assert_eq!(full_ref, "#/paths~1users~1{id}");
    }
}
