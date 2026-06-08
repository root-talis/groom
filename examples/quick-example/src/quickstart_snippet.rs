use axum::Router;
use groom_macros::Controller;
use utoipa::{OpenApi, openapi::OpenApiBuilder};

#[Controller()]
mod api {
    use axum::{extract::Query, response::IntoResponse};
    use groom::{extract::GroomExtractor, response::Response};
    use groom_macros::{DTO, Response};

    #[DTO(parameters)]
    pub struct GreetParams {
        name: Option<String>,
    }

    #[DTO(response)]
    pub struct GreetMessage {
        message: String,
    }

    #[DTO(response)]
    pub struct ErrorMessage {
        error: &'static str,
    }

    #[Response(format(json))]
    pub enum HelloResponse {
        #[Response(code = 200)]
        Hello(GreetMessage),

        #[Response(code = 400)]
        BadRequest(ErrorMessage),
    }

    #[Route(method = "get", path = "/hello")]
    pub async fn greet(Query(p): Query<GreetParams>) -> HelloResponse {
        let name = p.name.unwrap_or_else(|| "world".into());
        if name.is_empty() {
            HelloResponse::BadRequest(ErrorMessage {
                error: "`name` must be omitted or non-empty",
            })
        } else {
            HelloResponse::Hello(GreetMessage {
                message: format!("Hello, {name}!"),
            })
        }
    }
}

fn make_router() -> Router {
    api::merge_into_router(Router::new())
}

fn make_openapi() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(info(title = "My API", version = "0.1.0"))]
    struct ApiDoc;

    api::merge_into_openapi_builder(OpenApiBuilder::from(ApiDoc::openapi()))
        .build()
}
