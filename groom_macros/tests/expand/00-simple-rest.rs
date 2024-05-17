//! This is an old expansion preview that is too messy and should be broken down into separate
//! files and then removed.

#[macro_use]
extern crate groom_macros;
#[Controller]
pub mod api_root {
    use axum::extract::{Path,Query};

    /// Summary
    /// 
    /// Description
    #[Route(method = "get", path = "/")]
    pub async fn get_root() -> GetRootResponse {
        let a = 1;
    }

    #[Route(method = "post", path = "/")]
    pub async fn post_root() -> GetRootResponse {
        let a = 2;
    }

    /// Query<struct>
    #[Route(method = "get", path = "/greet")]
    pub async fn rq_cons_query_struct(query: Query<RqConsQueryStruct>) -> RqConsQueryResponse {
        if query.name.is_empty() {
            RqConsQueryResponse::BadRequest("Empty string".into())
        } else {
            let mut result = "Hello, ".to_owned();
            result.push_str(query.name);
            RqConsQueryResponse::Ok(result)
        }
    }

    /*
    /// Path<tuple>
    #[Route(method = "get", path = "/user/:user_id/team/:team_id")]
    pub async fn rq_cons_path_tuple(Path((user_id, team_id)): Path<(i32, String)>) -> RqConsPathResponse {
        RqConsPathResponse::Ok("ok".into())
    }*/

    /// Path<struct>
    #[Route(method = "get", path = "/team/:team_id/user/:user_id")]
    pub async fn rq_cons_path_struct(Path(team): Path<RqConsPathStruct>) -> RqConsPathResponse {
        RqConsPathResponse::Ok("ok".into())
    }

    #[Route(method = "get", path = "/json")]
    pub async fn resp_json() -> RespJsonResponse {
        RespJsonResponse::Ok(StructJson{
            success: true
        })
    }

    async fn not_a_handler() {
        let a = 1;
    }

    #[Response(format(plain_text))]
    pub enum GetRootResponse {
        /// There you go mate.
        #[Response()]
        Ok(String),

        /// Are you insane?
        /// 
        /// Bad request.
        #[Response(code = 400)]
        BadRequest(String),

        /// You shall not pass!
        #[Response(code = 401)]
        Forbidden,
    }

    #[DTO(request)]
    pub struct RqConsQueryStruct {
        name: String,
    }

    #[Response(format(plain_text))]
    pub enum RqConsQueryResponse {
        #[Response()]
        Ok(String),

        #[Response(code = 400)]
        BadRequest(String),
    }

    #[DTO(request)]
    pub struct RqConsPathStruct {
        user_id: String,
        team_id: i32,
    }

    #[Response(format(plain_text))]
    pub enum RqConsPathResponse {
        #[Response()]
        Ok(String),
    }

    #[DTO(response)]
    pub struct StructJson {
        success: bool
    }

    #[Response(format(json))]
    pub enum RespJsonResponse {
        #[Response()]
        Ok(StructJson),
    }

    /*#[RequestBody(formats(json, form))]
    pub struct RequestBodySimple(String);

    #[RequestBody(formats(json, form))]
    pub struct RequestBodyStruct {
        name: String,
        age: u8,
    }*/
}
