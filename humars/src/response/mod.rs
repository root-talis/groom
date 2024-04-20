use utoipa::openapi::path::OperationBuilder;

pub mod dto;
pub use dto::DTO_Response;

pub trait Response {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder;
}
