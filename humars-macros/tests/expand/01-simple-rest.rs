#[macro_use]
extern crate humars_macros;

use axum::extract::{Query,Path};

struct ApiRoot;

#[Controller]
impl ApiRoot {
    #[Route(method = "get", path = "/")]
    async fn get_root() -> GetRootResponse {
        let a = 1;
    }

    #[Route(method = "post", path = "/")]
    async fn post_root() {
        let a = 2;
    }

    /// Query<struct>
    #[Route(method = "get", path = "/greet")]
    async fn rq_cons_query_struct(query: Query<RqConsQueryStruct>) -> RqConsQueryResponse {
        if query.name.is_empty() {
            RqConsQueryResponse::BadRequest("Empty string".into())
        } else {
            let mut result = "Hello, ".to_owned();
            result.push_str(query.name);
            RqConsQueryResponse::Ok(result)
        }
    }

    /// Query<HashMap<String, String>>
    #[Route(method = "get", path = "/greet_2")]
    async fn rq_cons_query_struct(query: Query<HashMap<String, String>>) -> RqConsQueryResponse {
        if let Some(name) = query.get("name") {
            let mut result = "Hello, ".to_owned();
            result.push_str(name);
            RqConsQueryResponse::Ok(result)
        } else {
            RqConsQueryResponse::BadRequest("Empty string".into())
        }
    }

    /// Path<tuple>
    #[Route(method = "get", path = "/user/:user_id/team/:team_id")]
    async fn rq_cons_path_tuple(Path((user_id, team_id)): Path<(i32, String)>) -> RqConsPathResponse {
        RqConsPathResponse::Ok("ok".into())
    }

    /// Path<struct>
    #[Route(method = "get", path = "/team/:team_id/user/:user_id")]
    async fn rq_cons_path_struct(Path(team): Path<RqConsPathStruct>) -> RqConsPathResponse {
        RqConsPathResponse::Ok("ok".into())
    }

    async fn not_a_handler() {
        let a = 1;
    }
}

#[Response]
enum GetRootResponse {
    #[Response()]
    Ok(String),

    #[Response(code = 400)]
    BadRequest(String),

    #[Response(code = 401)]
    Forbidden,
}

#[DTO]
struct RqConsQueryStruct {
    name: String,
}

#[Response]
enum RqConsQueryResponse {
    #[Response()]
    Ok(String),

    #[Response(code = 400)]
    BadRequest(String),
}

#[DTO]
struct RqConsPathStruct {
    user_id: String,
    team_id: i32,
}

#[Response]
enum RqConsPathResponse {
    #[Response()]
    Ok(String),
}
