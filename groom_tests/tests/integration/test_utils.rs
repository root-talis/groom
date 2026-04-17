use axum::{
    Router, body::Body, http::{self, HeaderMap, Request, StatusCode}
};

use serde_json::Value;
use tower::ServiceExt; // for `call`, `oneshot` and `ready`
use http_body_util::BodyExt;
use utoipa::{OpenApi, openapi::OpenApiBuilder};

#[cfg(test)]
use pretty_assertions::assert_eq;

#[derive(Debug, PartialEq)]
pub struct RequestResult {
    pub status:  StatusCode,
    pub headers: HeaderMap,
    pub body:    String,
}

impl RequestResult {
    /// Assert Body value in response
    #[allow(dead_code)]
    pub fn assert_body<T: Into<String>>(&self, expected: T) -> &Self {
        assert_eq!(self.body, expected.into());
        self
    }
    /// Assert Body value in response
    #[allow(dead_code)]
    pub fn assert_no_body(&self) -> &Self {
        assert_eq!(self.body, "");
        self
    }

    /// Assert Status value in response
    #[allow(dead_code)]
    pub fn assert_status(&self, expected: u16) -> &Self {
        assert_eq!(self.status, expected);
        self
    }

    /// Assert Content-Type value in response
    #[allow(dead_code)]
    pub fn assert_content_type(&self, expected: &str) -> &Self {
        assert_eq!(
            self.headers.get("content-type").expect("should respond with content-type header"),
            expected
        );

        self
    }

    /// Assert that Content-Type is missing from response
    #[allow(dead_code)]
    pub fn assert_no_content_type(&self) -> &Self {
        assert_eq!(
            self.headers.get("content-type"),
            None,
            "should respond without content-type header"
        );

        self
    }
}

pub struct Req {
    pub method: http::Method,
    pub url:    String,
    pub accept: Option<&'static str>,
    pub body:   Option<ReqBody>,
}

#[allow(dead_code)]
impl Req {
    pub fn delete<T: Into<String>>(url: T) -> Self {
        Self::new(http::Method::DELETE, url)
    }

    pub fn get<T: Into<String>>(url: T) -> Self {
        Self::new(http::Method::GET, url)
    }

    pub fn post<T: Into<String>>(url: T) -> Self {
        Self::new(http::Method::POST, url)
    }

    pub fn put<T: Into<String>>(url: T) -> Self {
        Self::new(http::Method::PUT, url)
    }

    pub fn patch<T: Into<String>>(url: T) -> Self {
        Self::new(http::Method::PATCH, url)
    }

    fn new<T: Into<String>>(method: http::Method, url: T) -> Self {
        Self { method, url: url.into(), accept: None, body: None }
    }

    pub fn accept(mut self, accept: &'static str) -> Self {
        self.accept = Some(accept);
        self
    }

    pub fn with_body(mut self, body: ReqBody) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn call(self, r: &Router) -> RequestResult {
        let mut request = Request::builder().uri(self.url).method(self.method);

        if self.accept.is_some() {
            request = request.header("accept", self.accept.unwrap());
        }

        let request = if let Some(b) = self.body {
            if b.content_type.is_some() {
                request = request.header("content-type", b.content_type.unwrap());
            }

            request.body(b.body).unwrap()
        } else {
            request.body(Body::empty()).unwrap()
        };

        let response = r.clone()
            .oneshot(request)
            .await
            .unwrap()
        ;

        let status = response.status();
        let headers = response.headers().to_owned();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap().to_owned();

        RequestResult{ status, headers, body }
    }
}

pub struct ReqBody {
    pub content_type: Option<&'static str>,
    pub body: Body
}

#[allow(dead_code)]
impl ReqBody {
    pub fn new(body: Body) -> Self {
        Self { body, content_type: None }
    }

    pub fn with_content_type(mut self, ct: &'static str) -> Self {
        self.content_type = Some(ct);
        self
    }
}

pub fn assert_openapi_doc(
    merge: fn (b: OpenApiBuilder) -> OpenApiBuilder,
    expected_json: Value
) {
    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "t",
            description = "d",
            license(name = "n"),
            version = "0.0.0",
            contact(name = "name", email = "mail@example.com")
        )
    )]
    struct ApiDoc;

    let api = OpenApiBuilder::from(ApiDoc::openapi());
    let api = merge(api);

    let json = api.build().to_json().expect("expected a valid json string");

    // eprintln!("generated openapi definition as json:\n---\n{json}\n---");

    assert_eq!(
        json.parse::<serde_json::Value>().expect("expected a parsed json"),
        expected_json
    );
}
