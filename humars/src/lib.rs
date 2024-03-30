pub mod extract;

pub trait Controller {
    fn merge_into_router(other: ::axum::Router) -> ::axum::Router;

    fn merge_into_openapi_builder(other: ::utoipa::openapi::OpenApiBuilder) -> ::utoipa::openapi::OpenApiBuilder;
}

pub trait DTO {}
