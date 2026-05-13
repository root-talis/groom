use color_eyre::eyre::Result;

use tower_http::trace::TraceLayer;
use axum::{Extension, Router};

use crate::bootstrap::Bootstrap;

pub mod bootstrap;
pub mod server;

pub mod controller;
pub mod service;
pub mod repository;

pub fn make_router(app: Bootstrap) -> Result<Router> {
    Ok(
        controller::setup_router(Router::new(), true)?
            .layer(Extension(app.task_service))
            .layer(TraceLayer::new_for_http())
    )
}
