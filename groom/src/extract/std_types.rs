//use std::collections::HashMap;

// For Query<HashMap<...>>
//impl<K, V, S> crate::DTO for HashMap<K, V, S> {}

use utoipa::openapi::{ContentBuilder, KnownFormat, ObjectBuilder, Required, SchemaFormat, SchemaType};
use utoipa::openapi::path::{OperationBuilder};
use utoipa::openapi::request_body::RequestBodyBuilder;
use crate::extract::GroomExtractor;
use utoipa::PartialSchema;

impl GroomExtractor for String {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        op.request_body(Some(
            RequestBodyBuilder::new()
                .content(
                    "text/plain; charset=utf-8",
                    ContentBuilder::new()
                        .schema(String::schema())
                        .build()
                )
                .required(Some(Required::True))
                .build()
        ))
    }
}

impl GroomExtractor for axum::body::Bytes {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        op.request_body(Some(
            RequestBodyBuilder::new()
                .content(
                    "application/octet-stream",
                    ContentBuilder::new()
                        .schema(
                            ObjectBuilder::new()
                                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Binary)))
                                .schema_type(SchemaType::String)
                                .build()
                        )
                        .build()
                )
                .required(Some(Required::True))
                .build()
        ))
    }
}

/// Implements an empty GroomExtractor to allow any type to be used as a handler argument
/// without affecting OpenAPI definition.
#[macro_export]
macro_rules! groom_empty_extractor {
    ($ty:ty) => {
        impl ::groom::extract::GroomExtractor for $ty {
            fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
                op
            }
        }
    };
}

/// Implements an empty GroomExtractor to allow any type to be used as a handler argument
/// without affecting OpenAPI definition.
///
/// This macro is to define standard implementations.
macro_rules! _groom_empty_extractor_crate {
    ($ty:ty) => {
        impl GroomExtractor for $ty {
            fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
                op
            }
        }
    };
}

_groom_empty_extractor_crate!(axum::extract::Request);
_groom_empty_extractor_crate!(axum::http::HeaderMap);


impl<T> crate::extract::GroomExtractor for axum::extract::Extension<T> {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        op
    }
}

impl<T> crate::extract::GroomExtractor for axum::extract::State<T> {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        op
    }
}
