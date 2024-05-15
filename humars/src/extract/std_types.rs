//use std::collections::HashMap;

// For Query<HashMap<...>>
//impl<K, V, S> crate::DTO for HashMap<K, V, S> {}

use utoipa::openapi::{ContentBuilder, KnownFormat, ObjectBuilder, Required, SchemaFormat, SchemaType};
use utoipa::openapi::path::{OperationBuilder};
use utoipa::openapi::request_body::RequestBodyBuilder;
use crate::extract::HumarsExtractor;
use utoipa::PartialSchema;

impl HumarsExtractor for String {
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

impl HumarsExtractor for axum::body::Bytes {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        op.request_body(Some(
            RequestBodyBuilder::new()
                .content(
                    "application/octet-stream", // todo: ability to override Bytes type to define custom content_type
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
