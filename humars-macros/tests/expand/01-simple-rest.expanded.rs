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
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Summary"))
                        .description(Some("Description"));
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        paths = paths
            .path(
                "/",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Post,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        paths = paths
            .path(
                "/greet",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Query<struct>"))
                        .description(None as Option<String>);
                    op_builder = <Query<
                        RqConsQueryStruct,
                    >>::__openapi_modify_operation(op_builder);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        paths = paths
            .path(
                "/greet_2",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Query<HashMap<String, String>>"))
                        .description(None as Option<String>);
                    op_builder = <Query<
                        HashMap<String, String>,
                    >>::__openapi_modify_operation(op_builder);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        paths = paths
            .path(
                "/team/:team_id/user/:user_id",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Path<struct>"))
                        .description(None as Option<String>);
                    op_builder = <Path<
                        RqConsPathStruct,
                    >>::__openapi_modify_operation(op_builder);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        other.paths(paths)
    }
}
