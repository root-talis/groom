use axum::extract::Query;
use utoipa::openapi::{RefOr, path::{OperationBuilder, ParameterBuilder}};

use crate::{DTO, extract::{ComponentsRegistry, GroomExtractor}};

impl<T: DTO> GroomExtractor for Query<T> {
    fn __openapi_modify_operation(op: OperationBuilder, c: &mut ComponentsRegistry) -> OperationBuilder {
        c.add_components::<T>();

        let schema = match T::schema() {
            RefOr::T(s) => s,
            RefOr::Ref(_) => panic!("reference instead of schema when building Query"),
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
