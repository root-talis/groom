use std::collections::HashSet;
use std::fmt::Display;

pub struct HTTPCodeSet {
    seen: HashSet<u16>,
}

impl Default for HTTPCodeSet {
    fn default() -> Self {
        Self::new()
    }
}

impl HTTPCodeSet {
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    /// Records `code` or panics if it was already seen.
    /// `context` is formatted only on the panic path — no allocation when codes are distinct.
    pub fn ensure_distinct(&mut self, context: impl Display, code: u16) {
        if !self.seen.insert(code) {
            panic!("{context}: HTTP response code \"{code}\" is also taken by another response variant of this handler.")
        }
    }
}

/// Tracks response formats seen while walking a handler's return type.
/// `record` collects a type's supported mimes; `assert_same_as` panics when two
/// arms of a composite type (e.g. `Result<T, E>`) declare different lists.
///
/// Equality is order-insensitive and param-insensitive: only `mime.essence_str()`
/// values are stored (a `HashSet`). Negotiation declaration order is unchanged —
/// this type is only for Result-arm equality checks.
pub struct HTTPFormatsSet {
    formats: HashSet<String>,
}

impl Default for HTTPFormatsSet {
    fn default() -> Self {
        Self::new()
    }
}

impl HTTPFormatsSet {
    pub fn new() -> Self {
        Self {
            formats: HashSet::new(),
        }
    }

    pub fn record(&mut self, _context: impl Display, formats: &[::mime::Mime]) {
        for mime in formats {
            self.formats.insert(mime.essence_str().to_owned());
        }
    }

    pub fn assert_same_as(&self, context: impl Display, other: &HTTPFormatsSet) {
        if self.formats != other.formats {
            let this = sorted_join(&self.formats);
            let that = sorted_join(&other.formats);
            panic!("{context}: Ok variant supports formats [{this}] but Err variant supports formats [{that}]. Both variants of a Result response must support the same list of formats.")
        }
    }

    pub fn merge(&mut self, other: &HTTPFormatsSet) {
        self.formats.extend(other.formats.iter().cloned());
    }
}

fn sorted_join(formats: &HashSet<String>) -> String {
    let mut essences: Vec<&str> = formats.iter().map(String::as_str).collect();
    essences.sort_unstable();
    essences.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_set_equality_is_order_insensitive() {
        let mut a = HTTPFormatsSet::new();
        a.record("ok", &[mime::APPLICATION_JSON, mime::TEXT_HTML]);

        let mut b = HTTPFormatsSet::new();
        b.record("err", &[mime::TEXT_HTML, mime::APPLICATION_JSON]);

        a.assert_same_as("Result formats", &b);
    }

    #[test]
    #[should_panic(
        expected = "Ok variant supports formats [application/json] but Err variant supports formats [text/html]"
    )]
    fn formats_set_mismatch_still_panics() {
        let mut a = HTTPFormatsSet::new();
        a.record("ok", &[mime::APPLICATION_JSON]);

        let mut b = HTTPFormatsSet::new();
        b.record("err", &[mime::TEXT_HTML]);

        a.assert_same_as("Result formats", &b);
    }

    #[test]
    fn formats_set_equality_ignores_mime_params() {
        let mut a = HTTPFormatsSet::new();
        a.record("ok", &[mime::TEXT_HTML_UTF_8]);

        let mut b = HTTPFormatsSet::new();
        b.record("err", &[mime::TEXT_HTML]);

        a.assert_same_as("Result formats", &b);
    }

    #[test]
    fn formats_set_panic_message_joins_sorted_essences() {
        let mut a = HTTPFormatsSet::new();
        a.record("ok", &[mime::APPLICATION_JSON, mime::TEXT_HTML]);

        let mut b = HTTPFormatsSet::new();
        b.record("err", &[mime::TEXT_PLAIN]);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            a.assert_same_as("Result formats", &b);
        }));
        let payload = result.expect_err("expected formats mismatch panic");
        let message = payload
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .expect("panic payload should be a string");
        assert!(
            message.contains("Ok variant supports formats [application/json, text/html]"),
            "panic text should join sorted Ok essences, got: {message}"
        );
        assert!(
            message.contains("Err variant supports formats [text/plain]"),
            "panic text should join sorted Err essences, got: {message}"
        );
    }
}
