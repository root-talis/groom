use axum::extract::Query;
use utoipa::openapi::path::{OperationBuilder, ParameterBuilder};

use crate::{extract::GroomExtractor, DTO};

impl<T: DTO> GroomExtractor for Query<T> {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        let (name, schema) = T::schema();

        let param = ParameterBuilder::new()
            .parameter_in(utoipa::openapi::path::ParameterIn::Query)
            .required(utoipa::openapi::Required::True)
            .name(name)
            .schema(Some(schema))
            .build();

        op.parameter(param)
    }
}
