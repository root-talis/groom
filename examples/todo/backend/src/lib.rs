use color_eyre::eyre::Result;

use tower_http::{cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer}, trace::TraceLayer};
use axum::{Extension, Router, http::HeaderValue};

use crate::bootstrap::Bootstrap;

pub mod bootstrap;
pub mod server;

pub mod controller;
pub mod service;
pub mod repository;

#[cfg(feature = "static-assets")]
pub mod static_assets;

#[derive(Debug, Clone)]
pub enum CorsOrigin {
    Any,
    List(Vec<String>),
}

impl From<String> for CorsOrigin {
    fn from(value: String) -> Self {
        if value == "any" {
            return Self::Any;
        }

        Self::List(
            value
                .split(',')
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(String::from)
                .collect()
        )
    }
}

impl From<CorsOrigin> for AllowOrigin {
    fn from(val: CorsOrigin) -> Self {
        match val {
            CorsOrigin::Any => AllowOrigin::mirror_request(),
            CorsOrigin::List(items) => AllowOrigin::list(
                items
                    .into_iter()
                    .map(|s| 
                        s.parse::<HeaderValue>()
                            .unwrap_or_else(|_| panic!("Error parsing `{}` as CORS header value.", s))
                    )
            ),
        }
    }
}

pub fn make_router(app: Bootstrap, serve_spec: bool, origin: Option<CorsOrigin>) -> Result<Router> {
    let router = controller::setup_router(Router::new(), serve_spec)?;
    
    #[cfg(feature = "static-assets")]
    let router = static_assets::with_assets_route(router);

    let router = router
        .layer(Extension(app.task_service))
    ;

    let router = if let Some(origin) = origin {
        router.layer(
            CorsLayer::new()
                .allow_origin(origin)
                .allow_methods(AllowMethods::any())
                .allow_headers(AllowHeaders::any())
        )
    } else {
        router
    };

    Ok(router.layer(TraceLayer::new_for_http()))
}

#[cfg(test)]
mod tests;
