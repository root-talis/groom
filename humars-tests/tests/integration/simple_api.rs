use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};

use utoipa::openapi::path::{OperationBuilder, ParameterBuilder, PathItemBuilder};
use utoipa::openapi::request_body::RequestBodyBuilder;
use utoipa::openapi::{self, ComponentsBuilder, ContentBuilder, PathsBuilder};
use utoipa::{OpenApi, openapi::OpenApiBuilder};

use tower::ServiceExt; // for `call`, `oneshot` and `ready`
use http_body_util::BodyExt;

use crate::humars_macros::Controller;

use serde_json::json;


// region: test bootstrap utils -----------------------------------
//

fn router() -> Router {
    my_api::merge_into_router(Router::new())
}

async fn call_url(url: &str) -> (StatusCode, String) {
    let app = router();

    let response = app
        .oneshot(
            Request::builder()
                .uri(url)
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap()
    ;

    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap().to_owned();

    (status, body)
}


//
// endregion: test bootstrap utils --------------------------------------

// region: api implementation to test ------------------------------------
//

#[Controller]
mod my_api {
    use std::collections::HashMap;

    use axum::extract::{Path, Query};

    use crate::humars_macros::{Response, DTO};

    // region: dumb handlers ---------------------------------------------
    //

    // --- BEGIN GET / ---

    #[Route(method="get", path="/")]
    pub async fn root() -> RootResponse {
        RootResponse::Ok
    }

    #[Response]
    pub enum RootResponse {
        #[Response(code = 202)] // todo: allow using constants like axum::http::Status::ACCEPTED?
        Ok,
    }

    // --- END GET / ---

    // --- BEGIN GET /hello-world ---

    /// This method says "hello, world!"
    /// 
    /// Ha-ha, classic.
    #[Route(method="get", path="/hello-world")]
    pub async fn hello_world() -> HelloWorldResponse {
        HelloWorldResponse::Ok("hello, world!".into())
    }

    /// POST hello world
    #[Route(method="post", path="/hello-world")]
    pub async fn post_hello_world() -> HelloWorldResponse {
        HelloWorldResponse::Ok("hello, world!".into())
    }

    #[Response]
    pub enum HelloWorldResponse {
        #[Response()]
        Ok(String),
    }

    // --- END GET /hello-world ---

    //
    // endregion: dumb handlers -----------------------------------------

    // region: request consumption --------------------------------------
    //

    #[DTO]
    pub struct RqConsPathStruct {
        pub user_id: String,
        pub team_id: i32,
    }

    #[DTO]
    pub struct RqConsQueryParams {
        first_name: String,
        last_name: Option<String>,
    }

    #[DTO]
    pub struct RqConsQueryParams2 {
        title: Option<String>,
    }

    #[Response]
    pub enum RqConsQueryResponse {
        #[Response()]
        Ok(String),

        #[Response(code = 400)]
        BadRequest(String),
    }

    #[Response]
    pub enum RqConsPathResponse {
        #[Response()]
        Ok(String),
    }

    // Request consumption: Query<struct> (stuff after `?`)
    #[Route(method = "get", path = "/greet")]
    pub async fn rq_cons_query_struct(query: Query<RqConsQueryParams>, query2: Query<RqConsQueryParams2>) -> RqConsQueryResponse {
        if query.first_name.is_empty() {
            RqConsQueryResponse::BadRequest("Empty name".into())
        } else {
            RqConsQueryResponse::Ok(format!(
                "Hello, {}{}!",
                query2.title.clone().map_or_else(
                    || "".into(),
                    |t| format!("{} ", t)
                ),
                query.last_name.clone().map_or_else(
                    || query.first_name.clone(),
                    |last_name| format!("{} {}", query.first_name, last_name) 
                    // in RqConsQueryParams,  last_name is Option<String>
                )
            ))
        }
    }

    // Request consumption: Query<HashMap<String, String>> (stuff after `?`)
    #[Route(method = "get", path = "/greet_2")]
    async fn rq_cons_query_hashmap(query: Query<HashMap<String, String>>) -> RqConsQueryResponse {
        match (query.get("first_name"), query.get("last_name")) {
            (Some(first_name), Some(last_name)) => 
                RqConsQueryResponse::Ok(format!("Hello, {first_name} {last_name}!")),

            (_,_) => // I am a grown adult, I'm not gonna joke like this... 
                RqConsQueryResponse::BadRequest("ass".into()), // Well shit.
        }
    }

    // Request consumption: Path<tuple>
    #[Route(method = "get", path = "/user/:user_id/team/:team_id")]
    async fn rq_cons_path_tuple(Path((user_id, team_id)): Path<(i32, String)>) -> RqConsPathResponse {
        RqConsPathResponse::Ok(format!("{} -> {}", user_id, team_id))
    }

    // Request consumption: Path<struct>
    #[Route(method = "get", path = "/team/:team_id/user/:user_id")]
    async fn rq_cons_path_struct(Path(team): Path<RqConsPathStruct>) -> RqConsPathResponse {
        RqConsPathResponse::Ok(format!("{} -> {}", team.user_id, team.team_id))
    }
    // todo: validate that in Path<T> that if T is a struct, it implements DTO, make good error message

    // todo: String

    // todo: Bytes

    // todo: Json<Value>

    // todo: HeaderMap

    // todo: Request

    // todo: Extension<State>

    // todo: axum::form::Form - as a separate feature

    // todo: Method

    // todo: multiple HTTP methods in one handler

    // todo: extractors wrapped in Option<> and Result<>

    //
    // endregion: request consumption ---------------------------------

    #[allow(dead_code)]
    async fn not_a_handler() {}
}

//
// endregion: api implementation to test --------------------------

// region: tests --------------------------------------------------
//

//use ::utoipa::ToSchema;

#[test]
fn api_doc_scratchpad() {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    let api = OpenApiBuilder::from(ApiDoc::openapi());

    // region: should generate this for /hello-world
    let parameter = ParameterBuilder::new()
        .parameter_in(openapi::path::ParameterIn::Query)
        .name("sad")
        .schema(Some(my_api::RqConsQueryParams::default()))
        .build()
    ;

    let components = ComponentsBuilder::new()
        .schema("my_component", my_api::RqConsPathStruct::default())
        .build()
    ;

    let content = ContentBuilder::new()
        .schema(components.schemas["my_component"].clone())
        .build()
    ;

    let request_body = RequestBodyBuilder::new()
        .content("application/json", content)
        .build()
    ;

    let operation = OperationBuilder::new()
        .parameter(parameter)
        .request_body(Some(request_body))
        .description(Some("description"))
        .summary(Some("summary"))
        .build();
    let operation = OperationBuilder::from(operation).build(); 

    let path_item= PathItemBuilder::new()
        .operation(utoipa::openapi::PathItemType::Get, operation)
        .build()
    ;

    let paths = PathsBuilder::new()
        .path("/hello-world", path_item)
        .build()
    ;
    let paths = PathsBuilder::from(paths).build();

    let api = api.paths(paths);
    //let api = api.components(Some(components));

    // endregion: should generate this for /hello-world

    let _json = api.build().to_json().expect("expected a valid json");

    let _breakpoint = false;
}

#[test]
fn api_doc() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "t", description = "d", license(name = "n"), version = "0.0.0")
    )]
    struct ApiDoc;

    let api = OpenApiBuilder::from(ApiDoc::openapi());
    let api = my_api::merge_into_openapi_builder(api);

    let json = api.build().to_json().expect("expected a valid json string");

    assert_eq!(
        json!({
            "openapi": "3.0.3",
            "info": {"title": "t", "description": "d", "license": {"name": "n"}, "version": "0.0.0"},
            "paths": {
                "/": {
                    "get": {
                        "responses": {}
                    },
                },
                "/greet": {
                    "get": {
                        "responses": {}
                    },
                },
                "/greet_2": {
                    "get": {
                        "responses": {}
                    },
                },
                "/hello-world": {
                    "get": {
                        "summary": "This method says \"hello, world!\"",
                        "description": "Ha-ha, classic.",
                        "responses": {}
                    },
                    "post": {
                        "summary": "POST hello world",
                        "responses": {}
                    }
                },
                "/team/:team_id/user/:user_id": {
                    "get": {
                        "responses": {}
                    },
                },
                "/user/:user_id/team/:team_id": {
                    "get": {
                        "responses": {}
                    },
                },
            }
        }),
        json.parse::<serde_json::Value>().expect("expected a parsed json")
    );
}

#[tokio::test]
pub async fn test_root() {
    let (status, body) = call_url("/").await;

    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body, "");
}

#[tokio::test]
pub async fn test_hello_world() {
    let (status, body) = call_url("/hello-world").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "hello, world!");
}

#[tokio::test]
pub async fn test_query_struct() {
    let (status, body) = call_url("/greet?first_name=").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "Empty name");

    let (status, body) = call_url("/greet?first_name=Max").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello, Max!");

    let (status, body) = call_url("/greet?last_name=Doe&first_name=John").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello, John Doe!");

    let (status, body) = call_url("/greet?first_name=John&title=Sir").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello, Sir John!");

    let (status, body) = call_url("/greet?last_name=Backsword&title=Sir&first_name=John").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello, Sir John Backsword!");
}

#[tokio::test]
pub async fn test_query_struct_hashmap() {
    let (status, body) = call_url("/greet_2?first_name=").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "ass");

    let (status, body) = call_url("/greet_2?last_name=Doe&first_name=John").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello, John Doe!");

    let (status, body) = call_url("/greet_2?last_name=Backsword&title=Sir&first_name=John").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello, John Backsword!");
}

#[tokio::test]
pub async fn test_path() {
    let (status, body) = call_url("/user/1/team/7").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "1 -> 7");

    let (status, body) = call_url("/user/42/team/Hitchhikers").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "42 -> Hitchhikers");
}

//
// endregion: tests ------------------------------------------------
