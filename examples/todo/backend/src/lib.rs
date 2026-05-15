use color_eyre::eyre::Result;

use tower_http::{cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer}, trace::TraceLayer};
use axum::{Extension, Router, http::HeaderValue};

use crate::bootstrap::Bootstrap;

pub mod bootstrap;
pub mod server;

pub mod controller;
pub mod service;
pub mod repository;

#[derive(Debug)]
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
                .filter(|v| v.len() > 0)
                .map(String::from)
                .collect()
        )
    }
}

impl Into<AllowOrigin> for CorsOrigin {
    fn into(self) -> AllowOrigin {
        match self {
            CorsOrigin::Any => AllowOrigin::any(),
            CorsOrigin::List(items) => AllowOrigin::list(
                items
                    .into_iter()
                    .map(|s| 
                        s.parse::<HeaderValue>()
                            .expect(format!("Error parsing `{}` as CORS header value.", s).as_str())
                    )
            ),
        }
    }
}

pub fn make_router(app: Bootstrap, serve_spec: bool, origin: CorsOrigin) -> Result<Router> {
    Ok(
        controller::setup_router(Router::new(), serve_spec)?
            .layer(Extension(app.task_service))
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(origin)
                    .allow_methods(AllowMethods::any())
                    .allow_headers(AllowHeaders::any())
            )
    )
}
