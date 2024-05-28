use std::collections::HashSet;

pub struct HTTPCodeSet {
    seen: HashSet<u16>,
}

impl HTTPCodeSet {
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    pub fn ensure_distinct(&mut self, context: String, code: u16) {
        if self.seen.insert(code) == false {
            panic!("{context}: HTTP response code \"{code}\" is also taken by another response variant of this handler.")
        }
    }
}
