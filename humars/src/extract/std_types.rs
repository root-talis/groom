//use std::collections::HashMap;

// For Query<HashMap<...>>
//impl<K, V, S> crate::DTO for HashMap<K, V, S> {}

use utoipa::openapi::{ContentBuilder, Required};
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
