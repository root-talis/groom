use darling::{self, FromMeta};
use strum_macros::Display;
use crate::http::ResponseContentType::{Html, PlainText};

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

    pub(crate) fn count(&self) -> usize {
        let mut result = 0;

        if self.plain_text {
            result += 1;
        }

        if self.html {
            result += 1;
        }

        if self.json {
            result += 1;
        }

        result
    }

    pub(crate) fn get_the_only_value(&self) -> Option<ResponseContentType> {
        if self.count() != 1 {
            None
        } else if self.plain_text {
            Some(ResponseContentType::PlainText)
        } else if self.html {
            Some(ResponseContentType::Html)
        } else if self.json {
            Some(ResponseContentType::Json)
        } else {
            panic!("bug in ResponseContentTypesList::count() or ResponseContentTypesList::get_the_only_value()")
        }
    }

    pub(crate) fn has(&self, t: ResponseContentType) -> bool {
        match t {
            PlainText => self.plain_text,
            Html => self.html,
            ResponseContentType::Json => self.json,
        }
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
