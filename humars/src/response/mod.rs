use utoipa::openapi::path::OperationBuilder;

pub trait Response {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder;
}
