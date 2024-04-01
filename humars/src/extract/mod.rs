use utoipa::openapi::path::OperationBuilder;

mod query;
mod path;
mod std_types;

pub trait HumarsExtractor {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder;
}
