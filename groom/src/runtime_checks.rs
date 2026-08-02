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
pub struct HTTPFormatsSet {
    formats: Vec<::mime::Mime>,
}

impl Default for HTTPFormatsSet {
    fn default() -> Self {
        Self::new()
    }
}

impl HTTPFormatsSet {
    pub fn new() -> Self {
        Self {
            formats: Vec::new(),
        }
    }

    pub fn record(&mut self, _context: impl Display, formats: &[::mime::Mime]) {
        self.formats.extend(formats.iter().cloned());
    }

    pub fn assert_same_as(&self, context: impl Display, other: &HTTPFormatsSet) {
        if self.formats != other.formats {
            let this = self.formats.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ");
            let that = other.formats.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ");
            panic!("{context}: Ok variant supports formats [{this}] but Err variant supports formats [{that}]. Both variants of a Result response must support the same list of formats.")
        }
    }

    pub fn merge(&mut self, other: &HTTPFormatsSet) {
        self.formats.extend(other.formats.iter().cloned());
    }
}
