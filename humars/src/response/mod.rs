use accept_header::Accept;
use utoipa::openapi::path::OperationBuilder;

pub trait Response {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder;

    fn __humars_into_response(self, accept: Option<Accept>) -> ::axum::response::Response;

    // todo: __humars_content_type_supported() to determine that Accept header is OK beforehand
}
