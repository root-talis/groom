use axum::extract::Path;
use utoipa::openapi::{RefOr, path::{OperationBuilder, ParameterBuilder}};

use crate::{DTO, extract::{ComponentsRegistry, GroomExtractor}};

impl<T: DTO> GroomExtractor for Path<T> {
    fn __openapi_modify_operation(op: OperationBuilder, c: &mut ComponentsRegistry) -> OperationBuilder {
        c.add_components::<T>();

        let schema = match T::schema() {
            RefOr::T(s) => s,
            _ => return op,
        };

        let param = ParameterBuilder::new()
            .parameter_in(utoipa::openapi::path::ParameterIn::Path)
            .required(utoipa::openapi::Required::True)
            .name(T::name())
            .schema(Some(schema))
            .build();

        op.parameter(param)
    }
}
