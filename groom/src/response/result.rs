use accept_header::Accept;
use utoipa::openapi::path::OperationBuilder;
use crate::response::Response;
use crate::runtime_checks::HTTPCodeSet;

impl<T, E> Response for Result<T, E>
where T: Response, E: Response
{
    fn __openapi_modify_operation(op: OperationBuilder) -> OperationBuilder {
        let op = T::__openapi_modify_operation(op);
        let op = E::__openapi_modify_operation(op);
        op
    }

    fn __groom_into_response(self, accept: Option<Accept>) -> axum::response::Response {
        match self {
            Ok(t) => t.__groom_into_response(accept),
            Err(e) => e.__groom_into_response(accept),
        }
    }

    fn __groom_check_response_codes(context: &String, codes: &mut HTTPCodeSet) {
        T::__groom_check_response_codes(&format!("{context} / Result<Ok, _>"), codes);
        E::__groom_check_response_codes(&format!("{context} / Result<_, Err>"), codes);
    }
}
