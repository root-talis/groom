use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Opaque storage-layer failure that preserves the underlying error chain
/// without exposing infrastructure types through the service public API.
#[derive(Debug)]
pub struct StorageError(Box<dyn Error + Send + Sync>);

impl StorageError {
    pub(crate) fn new<E>(source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self(Box::new(source))
    }
}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Error for StorageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}
