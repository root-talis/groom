use std::collections::btree_map::Entry::Vacant;

use color_eyre::eyre::{self, Result, eyre};

use axum::{Extension, response::{IntoResponse, Response}, routing::get};
use axum::middleware::from_fn;
use groom::router::GroomRouterValid;
use groom_macros::Controller;
use http::StatusCode;
use utoipa::{OpenApi, openapi::{RefOr, ResponseBuilder, path::{HttpMethod, Operation}}};

// 
// Example auth middleware: runtime + OpenAPI spec contribution
// 

/// Valid base64-encoded token for authentication.
/// Hard-coded for demonstration purposes only. Don't do this in production!
const VALID_TOKEN: &str = "dXNlcjpwYXNz"; // base64("user:pass")

/// OpenApiSpecLayer that checks for a Base64 token
/// and documents the security scheme in the OpenAPI spec.
#[derive(Clone)]
pub struct Base64AuthLayer;

impl groom::router::OpenApiSpecLayer for Base64AuthLayer {
    fn modify_openapi(&self, api: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

        let scheme = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("Base64")
                .build(),
        );

        api.components
            .get_or_insert_with(utoipa::openapi::Components::new)
            .security_schemes
            .insert("base64Auth".to_string(), scheme);
    }

    fn modify_operation(&self, _path: &str, _method: &HttpMethod, operation: &mut Operation) {
        // Manually add "security" to the operation
        use utoipa::openapi::security::SecurityRequirement;
        operation.security = Some(vec![
            SecurityRequirement::new("base64Auth", [] as [&str; 0])
        ]);

        // Manually add response variant for Unauthorized response returned from this middleware
        let entry = operation.responses.responses.entry("401".to_string());
        if let Vacant(entry) = entry {
            entry.insert(RefOr::T(
                ResponseBuilder::new()
                    .description("Token validation failed")
                    .build()
            ));
        }
    }

    fn mount<S>(&self, r: axum::Router<S>) -> axum::Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        r.layer(from_fn(check_auth))
    }
}

async fn check_auth(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    let is_valid = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| {
            v.strip_prefix("Bearer ")
                .is_some_and(|token| token == VALID_TOKEN)
        });

    if is_valid {
        next.run(req).await
    } else {
        (http::StatusCode::UNAUTHORIZED, "Unauthorized: invalid or missing token").into_response()
    }
}

// 
// Controllers
// 

/// Public controller — no auth required.
#[Controller()]
mod public_controller {
    use axum::response::IntoResponse;
    use groom::response::Response;
    use groom_macros::Response;

    #[Response(format(plain_text))]
    pub enum HealthResponse {
        #[Response(code = 200)]
        Ok(String),
    }

    /// Health check endpoint (public, no auth required).
    #[Route(method = "get", path = "/health")]
    async fn health() -> HealthResponse {
        HealthResponse::Ok("OK".into())
    }
}

/// Private controller — requires valid base64 token.
#[Controller()]
mod private_controller {
    use axum::response::IntoResponse;
    use groom::response::Response;
    use groom_macros::Response;

    #[Response(format(plain_text))]
    pub enum HelloResponse {
        #[Response(code = 200)]
        Ok(String),
    }

    /// Greets the authenticated user.
    #[Route(method = "get", path = "/hello")]
    async fn hello() -> HelloResponse {
        HelloResponse::Ok("Hello, authenticated user!".into())
    }
}

// 
// Router construction
// 

fn make_router() -> GroomRouterValid {
    let private_router = private_controller::into_router()
        .layer_with_spec(Base64AuthLayer);

    public_controller::into_router()
        .nest("/private", private_router)
        .expect("nest failed")
        .validate()
        .expect("validation failed")
}

pub fn make_axum_router() -> Result<axum::Router> {
    let r = make_router();
    let spec = make_openapi(&r);
    
    Ok(
        r.to_axum_router()
            .route("/spec.yaml", get(get_spec))
            .layer(Extension(Spec(spec.to_yaml()?)))
    )
}

pub fn make_openapi(r: &GroomRouterValid) -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(info(
        title = "Auth Middleware Example",
        description = "Demonstrates per-route auth via OpenApiSpecLayer with base64 token validation",
        version = "0.1.0"
    ))]
    struct ApiDoc;

    r.to_openapi(ApiDoc::openapi())
}

#[derive(Clone)]
struct Spec(String);

async fn get_spec(Extension(Spec(spec)): Extension<Spec>) -> impl IntoResponse {
    (StatusCode::OK, spec)
}

// 
// Tests
// 

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    fn router() -> axum::Router {
        make_axum_router().expect("failed to create router")
    }

    async fn response_body(router: axum::Router, req: Request<Body>) -> (StatusCode, String) {
        let response = router.oneshot(req).await.unwrap();
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    fn get(url: &str) -> Request<Body> {
        Request::get(url).body(Body::empty()).unwrap()
    }

    fn get_with_auth(url: &str, token: &str) -> Request<Body> {
        let mut req = Request::get(url).body(Body::empty()).unwrap();
        req.headers_mut()
            .insert("authorization", format!("Bearer {token}").parse().unwrap());
        req
    }

    // Health endpoint (public)

    #[tokio::test]
    async fn health_returns_200() {
        let (status, body) = response_body(router(), get("/health")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "OK");
    }

    #[tokio::test]
    async fn health_ignores_auth_header() {
        let (status, body) = response_body(router(), get_with_auth("/health", "anything")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "OK");
    }

    // Private endpoint (auth required)

    #[tokio::test]
    async fn private_hello_without_token_returns_401() {
        let (status, body) = response_body(router(), get("/private/hello")).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(body.contains("Unauthorized"));
    }

    #[tokio::test]
    async fn private_hello_with_valid_token_returns_200() {
        let (status, body) = response_body(router(), get_with_auth("/private/hello", VALID_TOKEN)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "Hello, authenticated user!");
    }

    #[tokio::test]
    async fn private_hello_with_invalid_token_returns_401() {
        let (status, body) = response_body(router(), get_with_auth("/private/hello", "wrong-token")).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(body.contains("Unauthorized"));
    }

    #[tokio::test]
    async fn private_hello_with_malformed_header_returns_401() {
        let mut req = Request::get("/private/hello").body(Body::empty()).unwrap();
        req.headers_mut()
            .insert("authorization", "Basic dXNlcjpwYXNz".parse().unwrap());
        let (status, _) = response_body(router(), req).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    // OpenAPI spec

    #[test]
    fn openapi_includes_health_path_without_security() {
        let r = make_router();
        let spec = make_openapi(&r);
        let health = spec.paths.paths.get("/health").expect("/health should exist");
        assert!(health.get.is_some(), "/health should have GET");
        assert!(
            health.get.as_ref().unwrap().security.is_none(),
            "/health should NOT have security requirements"
        );
    }

    #[test]
    fn openapi_includes_private_hello_with_security() {
        let r = make_router();
        let spec = make_openapi(&r);
        let hello = spec.paths.paths.get("/private/hello").expect("/private/hello should exist");
        assert!(hello.get.is_some(), "/private/hello should have GET");
        let security = hello.get.as_ref().unwrap().security.as_ref()
            .expect("/private/hello should have security requirements");
        assert!(!security.is_empty(), "security should not be empty");
    }

    #[test]
    fn openapi_has_base64_auth_security_scheme() {
        let r = make_router();
        let spec = make_openapi(&r);
        let components = spec.components.as_ref().expect("should have components");
        assert!(
            components.security_schemes.contains_key("base64Auth"),
            "should have base64Auth security scheme"
        );
    }
}
