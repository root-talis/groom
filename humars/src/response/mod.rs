use accept_header::Accept;
use utoipa::openapi::path::OperationBuilder;

pub trait Response {
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder;

    fn __humars_into_response(self, accept: Option<Accept>) -> ::axum::response::Response;
}

// todo: content-type should be negotiated and set at ::humars::Response level, not DTO_Response
