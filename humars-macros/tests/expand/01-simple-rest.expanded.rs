#[macro_use]
extern crate humars_macros;
use axum::extract::{Query, Path};
struct ApiRoot;
impl ApiRoot {
    /// HTTP handler: GET /
    async fn get_root() -> GetRootResponse {
        let a = 1;
    }
    /// HTTP handler: POST /
    async fn post_root() {
        let a = 2;
    }
    /// HTTP handler: GET /greet
    /// Query<struct>
    async fn rq_cons_query_struct(
        query: Query<RqConsQueryStruct>,
    ) -> RqConsQueryResponse {
        if query.name.is_empty() {
            RqConsQueryResponse::BadRequest("Empty string".into())
        } else {
            let mut result = "Hello, ".to_owned();
            result.push_str(query.name);
            RqConsQueryResponse::Ok(result)
        }
    }
    /// HTTP handler: GET /greet_2
    /// Query<HashMap<String, String>>
    async fn rq_cons_query_struct(
        query: Query<HashMap<String, String>>,
    ) -> RqConsQueryResponse {
        if let Some(name) = query.get("name") {
            let mut result = "Hello, ".to_owned();
            result.push_str(name);
            RqConsQueryResponse::Ok(result)
        } else {
            RqConsQueryResponse::BadRequest("Empty string".into())
        }
    }
    /// HTTP handler: GET /user/:user_id/team/:team_id
    /// Path<tuple>
    async fn rq_cons_path_tuple(
        Path((user_id, team_id)): Path<(i32, String)>,
    ) -> RqConsPathResponse {
        RqConsPathResponse::Ok("ok".into())
    }
    /// HTTP handler: GET /team/:team_id/user/:user_id
    /// Path<struct>
    async fn rq_cons_path_struct(
        Path(team): Path<RqConsPathStruct>,
    ) -> RqConsPathResponse {
        RqConsPathResponse::Ok("ok".into())
    }
    async fn not_a_handler() {
        let a = 1;
    }
}
impl ::humars::Controller for ApiRoot {
    fn merge_into_router(other: ::axum::Router) -> ::axum::Router {
        let this_router = ::axum::Router::new()
            .route("/", ::axum::routing::get(Self::get_root))
            .route("/", ::axum::routing::post(Self::post_root))
            .route("/greet", ::axum::routing::get(Self::rq_cons_query_struct))
            .route("/greet_2", ::axum::routing::get(Self::rq_cons_query_struct))
            .route(
                "/user/:user_id/team/:team_id",
                ::axum::routing::get(Self::rq_cons_path_tuple),
            )
            .route(
                "/team/:team_id/user/:user_id",
                ::axum::routing::get(Self::rq_cons_path_struct),
            );
        other.merge(this_router)
    }
}
enum GetRootResponse {
    Ok(String),
    BadRequest(String),
    Forbidden,
}
impl ::axum::response::IntoResponse for GetRootResponse {
    fn into_response(self) -> ::axum::response::Response {
        match self {
            Self::Ok(body) => {
                (::axum::http::StatusCode::from_u16(200u16).unwrap(), body)
                    .into_response()
            }
            Self::BadRequest(body) => {
                (::axum::http::StatusCode::from_u16(400u16).unwrap(), body)
                    .into_response()
            }
            Self::Forbidden => {
                (::axum::http::StatusCode::from_u16(401u16).unwrap()).into_response()
            }
        }
    }
}
struct RqConsQueryStruct {
    name: String,
}
impl ::humars::DTO for RqConsQueryStruct {}
enum RqConsQueryResponse {
    Ok(String),
    BadRequest(String),
}
impl ::axum::response::IntoResponse for RqConsQueryResponse {
    fn into_response(self) -> ::axum::response::Response {
        match self {
            Self::Ok(body) => {
                (::axum::http::StatusCode::from_u16(200u16).unwrap(), body)
                    .into_response()
            }
            Self::BadRequest(body) => {
                (::axum::http::StatusCode::from_u16(400u16).unwrap(), body)
                    .into_response()
            }
        }
    }
}
struct RqConsPathStruct {
    user_id: String,
    team_id: i32,
}
impl ::humars::DTO for RqConsPathStruct {}
enum RqConsPathResponse {
    Ok(String),
}
impl ::axum::response::IntoResponse for RqConsPathResponse {
    fn into_response(self) -> ::axum::response::Response {
        match self {
            Self::Ok(body) => {
                (::axum::http::StatusCode::from_u16(200u16).unwrap(), body)
                    .into_response()
            }
        }
    }
}
