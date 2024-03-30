#[macro_use]
extern crate humars_macros;
pub mod api_root {
    use ::static_assertions::assert_impl_all;
    use axum::extract::{Path, Query};
    /// Summary
    ///
    /// HTTP handler: GET /
    ///
    /// Description
    pub async fn get_root() -> GetRootResponse {
        let a = 1;
    }
    /// HTTP handler: POST /
    pub async fn post_root() {
        let a = 2;
    }
    /// Query<struct>
    ///
    /// HTTP handler: GET /greet
    pub async fn rq_cons_query_struct(
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
    /// Query<HashMap<String, String>>
    ///
    /// HTTP handler: GET /greet_2
    pub async fn rq_cons_query_struct(
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
    /// Path<tuple>
    ///
    /// HTTP handler: GET /user/:user_id/team/:team_id
    pub async fn rq_cons_path_tuple(
        Path((user_id, team_id)): Path<(i32, String)>,
    ) -> RqConsPathResponse {
        RqConsPathResponse::Ok("ok".into())
    }
    /// Path<struct>
    ///
    /// HTTP handler: GET /team/:team_id/user/:user_id
    pub async fn rq_cons_path_struct(
        Path(team): Path<RqConsPathStruct>,
    ) -> RqConsPathResponse {
        RqConsPathResponse::Ok("ok".into())
    }
    async fn not_a_handler() {
        let a = 1;
    }
    pub enum GetRootResponse {
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
    pub struct RqConsQueryStruct {
        name: String,
    }
    #[automatically_derived]
    impl ::core::default::Default for RqConsQueryStruct {
        #[inline]
        fn default() -> RqConsQueryStruct {
            RqConsQueryStruct {
                name: ::core::default::Default::default(),
            }
        }
    }
    impl ::humars::DTO for RqConsQueryStruct {}
    pub enum RqConsQueryResponse {
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
    pub struct RqConsPathStruct {
        user_id: String,
        team_id: i32,
    }
    #[automatically_derived]
    impl ::core::default::Default for RqConsPathStruct {
        #[inline]
        fn default() -> RqConsPathStruct {
            RqConsPathStruct {
                user_id: ::core::default::Default::default(),
                team_id: ::core::default::Default::default(),
            }
        }
    }
    impl ::humars::DTO for RqConsPathStruct {}
    pub enum RqConsPathResponse {
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
    pub fn merge_into_router(other: ::axum::Router) -> ::axum::Router {
        let this_router = ::axum::Router::new()
            .route("/", ::axum::routing::get(get_root))
            .route("/", ::axum::routing::post(post_root))
            .route("/greet", ::axum::routing::get(rq_cons_query_struct))
            .route("/greet_2", ::axum::routing::get(rq_cons_query_struct))
            .route(
                "/user/:user_id/team/:team_id",
                ::axum::routing::get(rq_cons_path_tuple),
            )
            .route(
                "/team/:team_id/user/:user_id",
                ::axum::routing::get(rq_cons_path_struct),
            );
        other.merge(this_router)
    }
    pub fn merge_into_openapi_builder(
        other: ::utoipa::openapi::OpenApiBuilder,
    ) -> ::utoipa::openapi::OpenApiBuilder {
        let mut paths = ::utoipa::openapi::path::PathsBuilder::new();
        paths = paths
            .path(
                "/",
                ::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        ::utoipa::openapi::PathItemType::Get,
                        ::utoipa::openapi::path::OperationBuilder::new()
                            .summary(Some("Summary"))
                            .description(Some("Description"))
                            .build(),
                    )
                    .build(),
            );
        paths = paths
            .path(
                "/",
                ::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        ::utoipa::openapi::PathItemType::Post,
                        ::utoipa::openapi::path::OperationBuilder::new()
                            .summary(None as Option<String>)
                            .description(None as Option<String>)
                            .build(),
                    )
                    .build(),
            );
        paths = paths
            .path(
                "/greet",
                ::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        ::utoipa::openapi::PathItemType::Get,
                        ::utoipa::openapi::path::OperationBuilder::new()
                            .summary(Some("Query<struct>"))
                            .description(None as Option<String>)
                            .build(),
                    )
                    .build(),
            );
        paths = paths
            .path(
                "/greet_2",
                ::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        ::utoipa::openapi::PathItemType::Get,
                        ::utoipa::openapi::path::OperationBuilder::new()
                            .summary(Some("Query<HashMap<String, String>>"))
                            .description(None as Option<String>)
                            .build(),
                    )
                    .build(),
            );
        paths = paths
            .path(
                "/user/:user_id/team/:team_id",
                ::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        ::utoipa::openapi::PathItemType::Get,
                        ::utoipa::openapi::path::OperationBuilder::new()
                            .summary(Some("Path<tuple>"))
                            .description(None as Option<String>)
                            .build(),
                    )
                    .build(),
            );
        paths = paths
            .path(
                "/team/:team_id/user/:user_id",
                ::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        ::utoipa::openapi::PathItemType::Get,
                        ::utoipa::openapi::path::OperationBuilder::new()
                            .summary(Some("Path<struct>"))
                            .description(None as Option<String>)
                            .build(),
                    )
                    .build(),
            );
        other.paths(paths)
    }
}
