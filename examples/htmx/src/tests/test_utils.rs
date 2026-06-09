use std::collections::HashMap;

use axum::{
    Router,
    body::Body,
    http::{self, HeaderMap, Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[derive(Debug, PartialEq)]
pub struct RequestResult {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

impl RequestResult {
    pub fn assert_body_contains(&self, expected: &str) -> &Self {
        assert!(
            self.body.contains(expected),
            "expected body to contain {expected:?}, got:\n{}",
            self.body
        );
        self
    }

    pub fn assert_status(&self, expected: u16) -> &Self {
        assert_eq!(self.status, expected);
        self
    }

    pub fn assert_content_type(&self, expected: &str) -> &Self {
        assert_eq!(
            self.headers
                .get("content-type")
                .expect("should respond with content-type header"),
            expected
        );

        self
    }
}

pub struct Req {
    pub method: http::Method,
    pub url: String,
    pub headers: Option<HashMap<&'static str, &'static str>>,
    pub body: Option<ReqBody>,
}

impl Req {
    pub fn get<T: Into<String>>(url: T) -> Self {
        Self::new(http::Method::GET, url)
    }

    pub fn put<T: Into<String>>(url: T) -> Self {
        Self::new(http::Method::PUT, url)
    }

    fn new<T: Into<String>>(method: http::Method, url: T) -> Self {
        Self {
            method,
            url: url.into(),
            headers: None,
            body: None,
        }
    }

    pub fn with_body(mut self, body: ReqBody) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn call(self, router: &Router) -> RequestResult {
        let mut request = Request::builder().uri(self.url).method(self.method);

        if let Some(headers) = self.headers {
            for (name, value) in headers {
                request = request.header(name, value);
            }
        }

        let request = if let Some(body) = self.body {
            if let Some(content_type) = body.content_type {
                request = request.header("content-type", content_type);
            }

            request.body(body.body).unwrap()
        } else {
            request.body(Body::empty()).unwrap()
        };

        let response = router.clone().oneshot(request).await.unwrap();

        let status = response.status();
        let headers = response.headers().to_owned();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap().to_owned();

        RequestResult {
            status,
            headers,
            body,
        }
    }
}

pub struct ReqBody {
    pub content_type: Option<&'static str>,
    pub body: Body,
}

impl ReqBody {
    pub fn url_encoded(body: impl Into<Body>) -> Self {
        Self {
            content_type: Some("application/x-www-form-urlencoded"),
            body: body.into(),
        }
    }
}

/// Builds a URL-encoded request body from field/value pairs.
///
/// ```ignore
/// url_encoded_body! { message => "hello" }
/// url_encoded_body! { name => "Mark", age => "20" }
/// ```
macro_rules! url_encoded_body {
    ($($field:ident => $value:expr),+ $(,)?) => {{
        $crate::tests::test_utils::ReqBody::url_encoded({
            let mut body = String::new();
            $(
                if !body.is_empty() {
                    body.push('&');
                }
                body.push_str(stringify!($field));
                body.push('=');
                body.push_str(&$crate::tests::test_utils::form_encode($value));
            )+
            body
        })
    }};
}

pub(crate) use url_encoded_body;

pub(crate) fn form_encode(value: impl AsRef<str>) -> String {
    let mut out = String::new();

    for byte in value.as_ref().bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }

    out
}
