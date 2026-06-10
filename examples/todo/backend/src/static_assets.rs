use rust_embed::RustEmbed;
use axum::{Router, body::Body, http::Uri, routing::get};
use tracing::{info,debug};

#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Assets;

pub fn with_assets_route(router: Router) -> Router {
    info!("Serving static assets from embedded resources");

    router
        .route("/", get(assets_handler))
        .fallback(assets_handler)
}

async fn assets_handler(uri: Uri) -> impl axum::response::IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    debug!("assets_handler: {}", path);

    match Assets::get(&path) {
        Some(content) => {
            let body = axum::body::Body::from(content.data);
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            axum::response::Response::builder()
                .header("Content-Type", mime.as_ref())
                .body(body)
        },
        None => axum::response::Response::builder()
            .status(404)
            .body(Body::empty())
    }.expect("static response is expected to be built")
}
