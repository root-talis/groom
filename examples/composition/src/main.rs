use groom::router::GroomRouter;
use groom_macros::Controller;
use utoipa::OpenApi;

// --- Controller A ---

#[Controller()]
mod controller_a {
    use axum::response::IntoResponse;
    use groom::{extract::GroomExtractor, response::Response};
    use groom_macros::{DTO, Response};

    #[DTO(response)]
    pub struct Item {
        pub id: u32,
        pub name: String,
    }

    #[Response(format(json))]
    pub enum ItemResponse {
        #[Response(code = 200)]
        Ok(Item),
    }

    #[Route(method = "get", path = "/item")]
    pub async fn get_item() -> ItemResponse {
        ItemResponse::Ok(Item { id: 1, name: "example".into() })
    }
}

// --- Controller B ---

#[Controller()]
mod controller_b {
    use axum::response::IntoResponse;
    use groom::{extract::GroomExtractor, response::Response};
    use groom_macros::{DTO, Response};

    #[DTO(response)]
    pub struct Info {
        pub version: String,
        pub count: u32,
    }

    #[Response(format(json))]
    pub enum InfoResponse {
        #[Response(code = 200)]
        Ok(Info),
    }

    #[Route(method = "get", path = "/status")]
    pub async fn get_info() -> InfoResponse {
        InfoResponse::Ok(Info { version: "1.0.0".into(), count: 42 })
    }
}

// --- OpenAPI doc definition ---

#[derive(OpenApi)]
#[openapi(info(title = "Composition Example", version = "0.1.0"))]
struct ApiDoc;

// --- Composition ---

fn main() {
    let router_a = controller_a::into_router();
    let router_b = controller_b::into_router();

    let router_a_2 = router_a.clone();
    let router_a_3 = router_a.clone();

    let composed = GroomRouter::new()
        .merge(router_a)
        .expect("merge controller_a failed")
        .nest("/api/v2", router_b)
        .expect("nest controller_b failed")
        .nest("/api/v2", router_a_2)
        .expect("nest controller_a_2 under a different route has failed")
        .validate()
        .expect("GroomRouter validation failed for composition");

    let api = composed.to_openapi(ApiDoc::openapi());
    let value = serde_json::to_value(&api).unwrap();
    let yaml = serde_json::to_string_pretty(&value).unwrap();
    println!("{}", yaml);

    let _router: axum::Router = composed.to_axum_router();
    println!("Composition example completed successfully.");
}
