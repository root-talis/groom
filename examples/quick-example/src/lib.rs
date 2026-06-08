include!("quickstart_snippet.rs");

pub fn build_router() -> Router {
    make_router()
}

pub fn build_openapi() -> utoipa::openapi::OpenApi {
    make_openapi()
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    async fn response_body(router: axum::Router, url: &str) -> (StatusCode, String) {
        let response = router
            .oneshot(Request::get(url).body(Body::empty()).unwrap())
            .await
            .unwrap();

        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn greet_defaults_to_world() {
        let (status, body) = response_body(build_router(), "/hello").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#"{"message":"Hello, world!"}"#);
    }

    #[tokio::test]
    async fn greet_uses_name_query_param() {
        let (status, body) = response_body(build_router(), "/hello?name=Groom").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#"{"message":"Hello, Groom!"}"#);
    }

    #[tokio::test]
    async fn greet_rejects_empty_name() {
        let (status, body) = response_body(build_router(), "/hello?name=").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body,
            r#"{"error":"`name` must be omitted or non-empty"}"#
        );
    }

    #[test]
    fn openapi_includes_hello_path() {
        let spec = build_openapi();
        assert!(spec.paths.paths.contains_key("/hello"));
    }
}
