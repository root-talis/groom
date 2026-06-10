use std::str::FromStr;

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

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct CorsOriginParseError(String);

#[derive(Debug, Clone)]
pub enum CorsOrigin {
    Any,
    List(Vec<HeaderValue>),
}

impl FromStr for CorsOrigin {
    type Err = CorsOriginParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "any" {
            return Ok(Self::Any);
        }

        let items: Result<Vec<_>, _> = value
            .split(',')
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(|origin| {
                origin.parse::<HeaderValue>().map_err(|_| {
                    CorsOriginParseError(format!("invalid CORS origin `{origin}`"))
                })
            })
            .collect();

        let items = items?;
        if items.is_empty() {
            return Err(CorsOriginParseError(
                "expected `any` or a comma-separated list of origins".into(),
            ));
        }

        Ok(Self::List(items))
    }
}

impl From<CorsOrigin> for AllowOrigin {
    fn from(val: CorsOrigin) -> Self {
        match val {
            CorsOrigin::Any => AllowOrigin::mirror_request(),
            CorsOrigin::List(items) => AllowOrigin::list(items),
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
