use axum::extract::{Path, Query};
use utoipa::{IntoParams, openapi::{RefOr, Schema, path::{OperationBuilder, Parameter, ParameterIn}}};

use crate::{DTO, extract::{ComponentsRegistry, GroomExtractor, components_registry::ComponentEntry}};

fn get_schemas<T: DTO>(c: &mut ComponentsRegistry) -> Vec<(String, ComponentEntry)> {
    c.add_subcomponents::<T>();

    let mut schemas = Vec::<(String, RefOr<Schema>)>::new();
    T::schemas(&mut schemas);

    schemas
        .into_iter()
        .map(|(name, ref_or)| (name.clone(), c.add_component(name, ref_or, None)))
        .collect()
}

fn fold_parameter(op: OperationBuilder, mut p: Parameter, schemas: &[(String, ComponentEntry)]) -> OperationBuilder {
    if let Some(ref schema) = p.schema {
        match schema {
            RefOr::T(schema)   => {
                if let Some(s) = schemas.iter().find(|&(_, component)| component.schema == *schema) {
                    p.schema = Some(s.1.clone().into());
                }
            },
            RefOr::Ref(r) => {
                let c = schemas
                    .iter()
                    .find(|&(_, component)| 
                        component.reference.is_some() &&
                        component.reference.as_ref().unwrap().ref_location == *r.ref_location
                    );
                
                if let Some(s) = c {
                    p.schema = Some(s.1.clone().into());
                }
            },
        };
    }

    op.parameter(p)
}

impl<T: DTO + IntoParams> GroomExtractor for Path<T> {
    fn __openapi_modify_operation(op: OperationBuilder, registry: &mut ComponentsRegistry) -> OperationBuilder {
        let schemas = get_schemas::<T>(registry);

        T::into_params(|| Some(ParameterIn::Path))
            .into_iter()
            .fold(op, |op, p| {
                fold_parameter(op, p, &schemas)
            })
    }
}

impl<T: DTO + IntoParams> GroomExtractor for Query<T> {
    fn __openapi_modify_operation(op: OperationBuilder, registry: &mut ComponentsRegistry) -> OperationBuilder {
        let schemas = get_schemas::<T>(registry);

        T::into_params(|| Some(ParameterIn::Query))
            .into_iter()
            .fold(op, |op, p| {
                fold_parameter(op, p, &schemas)
            })
    }
}

#[cfg(feature="axum-extra-query")]
impl<T: DTO + IntoParams> GroomExtractor for axum_extra::extract::Query<T> {
    fn __openapi_modify_operation(op: OperationBuilder, registry: &mut ComponentsRegistry) -> OperationBuilder {
        let schemas = get_schemas::<T>(registry);

        T::into_params(|| Some(ParameterIn::Query))
            .into_iter()
            .fold(op, |op, p| {
                fold_parameter(op, p, &schemas)
            })
    }
}
