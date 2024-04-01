use axum::extract::Query;
use utoipa::openapi::path::OperationBuilder;

use crate::extract::HumarsExtractor;

impl<T: crate::DTO> HumarsExtractor for Query<T> {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        op
    }
}
