use accept_header::Accept;
use utoipa::openapi::path::OperationBuilder;

pub trait Response {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder;

    fn __groom_into_response(self, accept: Option<Accept>) -> ::axum::response::Response;

    // todo: __groom_content_type_supported() to determine that Accept header is OK beforehand
}

pub mod html_response;
pub use html_response::{HtmlFormat, html_format};
