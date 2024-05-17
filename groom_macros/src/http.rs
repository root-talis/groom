use darling::{self, FromMeta};
use strum_macros::Display;

#[derive(Debug, Copy, Clone, FromMeta, Eq, PartialEq, Hash, Display)]
#[darling(rename_all = "lowercase")]
#[strum(serialize_all="lowercase")]
pub(crate) enum HTTPMethod {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}

#[derive(Debug, Copy, Clone, FromMeta, Eq, PartialEq, Hash)]
pub(crate) struct HTTPStatusCode(pub(crate) u16); // todo: std::num::NonZeroU16?

impl Default for HTTPStatusCode {
    fn default() -> Self {
        Self(200u16)
    }
}
