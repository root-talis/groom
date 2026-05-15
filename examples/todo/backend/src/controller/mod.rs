use axum::{Extension, Router, http::StatusCode, response::IntoResponse, routing::get};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utoipa::{OpenApi, openapi::OpenApiBuilder};
use color_eyre::eyre::Result;

mod todos;

/// Sets up router to serve everything from Controllers layer.
pub fn setup_router(router: Router, serve_spec: bool) -> Result<Router> {
    let router = todos::setup_router(router);

    if !serve_spec {
        tracing::debug!("not serving api spec");
        return Ok(router);
    }

    tracing::warn!("serving api spec");
    Ok(
        router
            .route("/spec.yaml", get(get_spec))
            .layer(Extension(make_spec()?))
    )
    
}

/// HTTP handler to serve OpenAPI spec.
async fn get_spec(Extension(Spec(spec)): Extension<Spec>) -> impl IntoResponse {
    (StatusCode::OK, spec)
}

#[derive(Clone)]
pub struct Spec(String);

impl Spec {
    pub fn get(self) -> String {
        self.0
    }
}

pub fn make_spec() -> Result<Spec> {
    #[derive(utoipa::OpenApi)]
    #[openapi(
        info(
            title = "TODO example (Groom)",
            description = "Shows how to structure backend with three-layer architecture and generate spec for frontend.",
            version = "0.0.1",
            contact(name = "name", email = "mail@example.com")
        )
    )]
    struct ApiDoc;

    let spec_builder = OpenApiBuilder::from(OpenApiBuilder::from(ApiDoc::openapi()));
    Ok(Spec(todos::setup_spec(spec_builder).build().to_yaml()?))
}
