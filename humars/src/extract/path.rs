use axum::extract::Path;
use utoipa::openapi::path::{OperationBuilder, ParameterBuilder};

use crate::{extract::HumarsExtractor, DTO};

impl<T: DTO> HumarsExtractor for Path<T> {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        let (name, schema) = T::schema();

        let param = ParameterBuilder::new()
            .parameter_in(utoipa::openapi::path::ParameterIn::Path)
            .required(utoipa::openapi::Required::True)
            .name(name)
            .schema(Some(schema))
            .build();

        op.parameter(param)
    }
}
