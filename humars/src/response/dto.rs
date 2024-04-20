use utoipa::{openapi::{ContentBuilder, ResponseBuilder}, PartialSchema};
use axum::response::Html;

/// Indicates that type is annotated with `#[DTO(response(...))]`.
/// 
/// Do not implement this manually.
#[allow(non_camel_case_types)]
pub trait DTO_Response {
    fn __openapi_build_responses(rb: ResponseBuilder) -> ResponseBuilder;
}

macro_rules! implement_dto_response {
    (for $ty:ident with $content_type:expr) => {
        impl DTO_Response for $ty {
            fn __openapi_build_responses(rb: ResponseBuilder) -> ResponseBuilder {
                rb.content(
                    $content_type,
                    ContentBuilder::new()
                        .schema(Self::schema())
                        //.example(Some("Hello, world!".into()))
                        .build()
                )
            }
        }
    };

    (for $ty:ident <$gp:ident> as $rt:ident with $content_type:expr) => {
        impl <$gp> DTO_Response for $ty<$gp> {
            fn __openapi_build_responses(rb: ResponseBuilder) -> ResponseBuilder {
                rb.content(
                    $content_type,
                    ContentBuilder::new()
                        .schema($rt::schema())
                        //.example(Some("Hello, world!".into()))
                        .build()
                )
            }
        }
    };
}

implement_dto_response!(for String with "text/plain");
implement_dto_response!(for Html<T> as String with "text/html");
