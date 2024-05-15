use utoipa::openapi::path::OperationBuilder;

mod query;
mod path;
mod std_types;

pub trait HumarsExtractor {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder;
}

/// Creates a newtype for axum::body::Bytes with custom content type specified in the openapi spec.
///
/// Does not affect content-type negotiation.
#[macro_export]
macro_rules! binary_request_body {
    ($name:ident with content_type $content_type:literal) => {
        #[derive(::axum::extract::FromRequest)]
        struct $name(::axum::body::Bytes);

        impl ::humars::extract::HumarsExtractor for $name {
            fn __openapi_modify_operation(op: ::utoipa::openapi::path::OperationBuilder) -> ::utoipa::openapi::path::OperationBuilder {
                op.request_body(Some(
                    ::utoipa::openapi::request_body::RequestBodyBuilder::new()
                        .content(
                            $content_type,
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    ::utoipa::openapi::ObjectBuilder::new()
                                        .format(Some(::utoipa::openapi::SchemaFormat::KnownFormat(::utoipa::openapi::KnownFormat::Binary)))
                                        .schema_type(::utoipa::openapi::SchemaType::String)
                                        .build()
                                )
                                .build()
                        )
                        .required(Some(::utoipa::openapi::Required::True))
                        .build()
                ))
            }
        }
    };
}
