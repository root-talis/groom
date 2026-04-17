use axum::extract::Query;
use utoipa::openapi::{RefOr, path::{OperationBuilder, ParameterBuilder}};

use crate::{extract::GroomExtractor, DTO};

impl<T: DTO> GroomExtractor for Query<T> {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        let schema = match T::schema() {
            RefOr::T(s) => s,
            _ => return op,
        };

        let param = ParameterBuilder::new()
            .parameter_in(utoipa::openapi::path::ParameterIn::Query)
            .required(utoipa::openapi::Required::True)
            .name(T::name())
            .schema(Some(schema))
            .build();

        op.parameter(param)
    }
}
