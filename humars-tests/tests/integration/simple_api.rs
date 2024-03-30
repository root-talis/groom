use std::collections::HashMap;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};

use static_assertions::assert_impl_all;
use axum::extract::{Query, Path};

use tower::ServiceExt; // for `call`, `oneshot` and `ready`
use http_body_util::BodyExt;


use crate::humars_macros::{Controller, Response, DTO};
use crate::humars::Controller;


// region: test bootstrap utils -----------------------------------
//

fn app() -> Router {
    MyApi::merge_into_router(Router::new())
}

async fn call_url(url: &str) -> (StatusCode, String) {
    let app = app();

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

/*
fn api_doc() -> OpenApiBuilder {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            MyApi::root,
            MyApi::hello_world
        ),
        tags(
            (name = "todo", description = "ToDo")
        )
    )]
    struct ApiDoc;

    let doc = ApiDoc::openapi();

    let api = OpenApiBuilder::from(doc);

    return api;
}
*/

//
// endregion: test bootstrap utils --------------------------------------

// region: api implementation to test ------------------------------------
//

struct MyApi;

#[Controller]
impl MyApi {
    // region: dumb handlers ---------------------------------------------
    //

    #[Route(method="get", path="/")]
    async fn root() -> RootResponse {
        RootResponse::Ok
    }

    #[Route(method="get", path="/hello-world")]
    async fn hello_world() -> HelloWorldResponse {
        HelloWorldResponse::Ok("hello, world!".into())
    }

    //
    // endregion: dumb handlers -----------------------------------------

    // region: request consumption --------------------------------------
    //

    // Request consumption: Query<struct> (stuff after `?`)
    #[Route(method = "get", path = "/greet")]
    async fn rq_cons_query_struct(query: Query<RqConsQueryParams>, query2: Query<RqConsQueryParams2>) -> RqConsQueryResponse {
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
    async fn not_a_handler(&self) {}
}

#[DTO]
struct RqConsPathStruct {
    user_id: String,
    team_id: i32,
}

#[Response]
enum RootResponse {
    #[Response(code = 202)] // todo: allow using constants like axum::http::Status::ACCEPTED?
    Ok,
}

#[Response]
enum HelloWorldResponse {
    #[Response()]
    Ok(String),
}

#[DTO]
struct RqConsQueryParams {
    first_name: String,
    last_name: Option<String>,
}

#[DTO]
struct RqConsQueryParams2 {
    title: Option<String>,
}

#[Response]
enum RqConsQueryResponse {
    #[Response()]
    Ok(String),

    #[Response(code = 400)]
    BadRequest(String),
}

#[Response]
enum RqConsPathResponse {
    #[Response()]
    Ok(String),
}

//
// endregion: api implementation to test --------------------------

// region: tests --------------------------------------------------
//

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
