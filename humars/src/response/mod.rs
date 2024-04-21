use ::axum::http::HeaderValue;
use utoipa::openapi::path::OperationBuilder;

pub mod dto;
pub use dto::DTO_Response;

pub trait Response {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder;

    fn __humars_into_response(self, accept: Option<&HeaderValue>) -> ::axum::response::Response;
}
