use groom_macros::Controller;
use tokio::net::TcpListener;
use utoipa::OpenApi;

// ────────────────────────────────────────────────────────────
// Auth middleware: runtime + OpenAPI spec contribution
// ────────────────────────────────────────────────────────────

/// Tower layer + OpenApiSpecLayer that checks for a Bearer token
/// and documents the security scheme in the OpenAPI spec.
#[derive(Clone)]
struct BearerAuthLayer;

impl tower::Layer<axum::routing::Route> for BearerAuthLayer {
    type Service = BearerAuthService;

    fn layer(&self, inner: axum::routing::Route) -> Self::Service {
        BearerAuthService { inner }
    }
}

impl groom::router::OpenApiSpecLayer for BearerAuthLayer {
    fn modify_openapi(&self, api: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

        let scheme = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );

        let components = api
            .components
            .get_or_insert_with(utoipa::openapi::Components::new);
        components
            .security_schemes
            .insert("bearerAuth".to_string(), scheme);
    }

    fn modify_operation(
        &self,
        _path: &str,
        _method: &utoipa::openapi::path::HttpMethod,
        operation: &mut utoipa::openapi::path::Operation,
    ) {
        use utoipa::openapi::security::SecurityRequirement;
        operation.security = Some(vec![
            SecurityRequirement::new("bearerAuth", [] as [&str; 0])
        ]);
    }

    fn mount<S>(&self, r: axum::Router<S>) -> axum::Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        use axum::middleware::from_fn;
        async fn check_auth(
            req: axum::extract::Request,
            next: axum::middleware::Next,
        ) -> axum::response::Response {
            next.run(req).await
        }
        r.layer(from_fn(check_auth))
    }
}

#[derive(Clone)]
struct BearerAuthService {
    inner: axum::routing::Route,
}

impl tower::Service<axum::extract::Request> for BearerAuthService {
    type Response = axum::response::Response;
    type Error = std::convert::Infallible;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        <axum::routing::Route as tower::Service<axum::extract::Request>>::poll_ready(
            &mut self.inner,
            cx,
        )
    }

    fn call(&mut self, req: axum::extract::Request) -> Self::Future {
        let has_auth = req
            .headers()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.starts_with("Bearer "))
            .unwrap_or(false);

        if has_auth {
            let fut = self.inner.call(req);
            Box::pin(async move { fut.await.map_err(|e| match e {}) })
        } else {
            let response = axum::response::Response::builder()
                .status(401)
                .header("www-authenticate", "Bearer")
                .body(axum::body::Body::from(
                    "Unauthorized: missing or invalid Bearer token",
                ))
                .unwrap();
            Box::pin(async { Ok(response) })
        }
    }
}

// ────────────────────────────────────────────────────────────
// Controller
// ────────────────────────────────────────────────────────────

#[Controller()]
mod api {
    use axum::response::IntoResponse;
    use groom::response::Response;
    use groom_macros::Response;

    #[Response(format(plain_text))]
    pub enum HelloResponse {
        #[Response(code = 200)]
        Ok(String),

        #[Response(code = 401)]
        Unauthorized(String),
    }

    /// Greets the authenticated user.
    #[Route(method = "get", path = "/hello")]
    async fn hello() -> HelloResponse {
        HelloResponse::Ok("Hello, authenticated user!".into())
    }
}

// ────────────────────────────────────────────────────────────
// Main
// ────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    #[derive(OpenApi)]
    #[openapi(info(
        title = "Auth Middleware Example",
        description = "Demonstrates OpenApiSpecLayer for middleware documentation",
        version = "0.1.0"
    ))]
    struct ApiDoc;

    let r = api::into_router()
        .layer_with_spec(BearerAuthLayer)
        .validate()
        .expect("validation failed");

    let spec = r.to_openapi(ApiDoc::openapi());
    println!("OpenAPI spec:\n{}", spec.to_yaml().unwrap());

    let router = r.to_axum_router();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
