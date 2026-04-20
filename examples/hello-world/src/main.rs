use color_eyre::eyre::{self, Result, eyre};

use tower_http::trace::TraceLayer;
use tokio::{net::TcpListener, signal};
use axum::{Extension, Router, http::StatusCode, response::IntoResponse, routing::get};
use groom_macros::Controller;
use utoipa::{OpenApi, openapi::OpenApiBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    run_server(
        String::from("127.0.0.1:8888"),
        make_router()?,
        shutdown_signal()
    ).await
}

// region: HTTP controller
//

fn make_router() -> Result<Router> {
    Ok(
        controller::merge_into_router(Router::new())
            .route("/spec.yaml", get(get_spec))
            .layer(Extension(make_spec()?))
            .layer(TraceLayer::new_for_http())
    )
}

#[Controller()]
mod controller {
    use axum::{extract::Query,response::IntoResponse};

    use groom::{
        // GroomExtractor is the trait that enables types to describe themselves into openapi spec.
        //
        // Importing it fixes this error: 
        //    "required methods no associated item named '__openapi_modify_operation' found 
        //     for struct `axum::extract::Query<GreetParams>` in the current scope".
        extract::GroomExtractor,

        // Response is the trait that enables enums and structs to turn themselves into HTTP responses
        // and into openapi spec.
        response::Response
    };
    
    use groom_macros::{
        // DTO macro generates implementations for DTOs.
        DTO,

        // Response macro generates implementations for enums and structs as responses.
        Response
    };

    /// Parameters to personalize greetings.
    #[DTO(request)]
    pub struct GreetParams {
        name: Option<String>,
    }

    /// Greeting. Personalized or generalized.
    #[Response(format(plain_text))]
    pub enum HelloResponse {
        #[Response(code = 200)]
        Hello(String),
    }

    /// Greets client. 
    /// 
    /// Uses `name` for personalization. If `name` is omitted, then greets the whole World.
    #[Route(method="get", path="/hello")]
    pub async fn greet(Query(p): Query<GreetParams>) -> HelloResponse {
        HelloResponse::Hello(
            format!("Hello, {}!", p.name.unwrap_or(String::from("world")))
        )
    }
}

//
// endregion

// region: OpenAPI spec (built once at startup and injected as Extension)
//

#[derive(Clone)]
struct Spec(String);

fn make_spec() -> Result<Spec> {
    #[derive(utoipa::OpenApi)]
    #[openapi(
        info(
            title = "Hello world example (Groom)",
            description = "Provides minimalistic implementation of a Groom app",
            version = "0.0.1",
            contact(name = "name", email = "mail@example.com")
        )
    )]
    struct ApiDoc;

    Ok(Spec(
        controller::merge_into_openapi_builder(
            OpenApiBuilder::from(ApiDoc::openapi())
        )
        .build()
        .to_yaml()?
    ))
}

async fn get_spec(Extension(Spec(spec)): Extension<Spec>) -> impl IntoResponse {
    (StatusCode::OK, spec)
}

//
// endregion: OpenAPI spec

// region: HTTP server
//

async fn run_server<Stop>(addr: String, router: Router, stop: Stop) -> Result<()> 
where
    Stop: Future<Output = ()> + Send + 'static
{
    let listener = TcpListener::bind(addr.clone())
        .await
        .map_err(|e| -> eyre::Report {
            eyre!("failed to listen on {}: {}", addr, e)
        })?;

    tracing::info!(
        addr=listener.local_addr().expect("failed to get local_addr from tcp listener").to_string(),
        "accepting connections"
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(stop)
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    let signal = tokio::select! {
        _ = ctrl_c => "ctrl+c",
        _ = terminate => "termination signal",
    };

    tracing::info!(reason=signal, "shutting down")
}

//
// endregion: HTTP server
