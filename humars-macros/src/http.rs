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

#[derive(Debug, Copy, Clone, FromMeta, Eq, PartialEq, Hash, Display)]
#[darling(rename_all = "snake_case")]
#[strum(serialize_all="snake_case")]
pub(crate) enum ResponseContentType {
    PlainText,
    Html,
    Json,
}

#[derive(FromMeta, Default)]
pub(crate) struct ResponseContentTypesList {
    #[darling(default)]
    pub(crate) plain_text: bool,

    #[darling(default)]
    pub(crate) html: bool,

    #[darling(default)]
    pub(crate) json: bool,
}


impl ResponseContentTypesList {
    pub(crate) fn is_any(&self) -> bool {
        return self.plain_text || self.html || self.json;
    }
}


impl Into<Vec<ResponseContentType>> for ResponseContentTypesList {
    fn into(self) -> Vec<ResponseContentType> {
        let mut v = Vec::with_capacity(3);
        if self.plain_text {
            v.push(ResponseContentType::PlainText)
        }
        if self.html {
            v.push(ResponseContentType::Html)
        }
        if self.json {
            v.push(ResponseContentType::Json)
        }
        v
    }
}
