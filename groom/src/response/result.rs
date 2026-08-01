use accept_header::Accept;
use utoipa::openapi::path::OperationBuilder;
use crate::extract::ComponentsRegistry;
use crate::response::Response;
use crate::runtime_checks::{HTTPCodeSet, HTTPFormatsSet};

impl<T, E> Response for Result<T, E>
where T: Response, E: Response
{
    fn __openapi_modify_operation(op: OperationBuilder, c: &mut ComponentsRegistry) -> OperationBuilder {
        let op = T::__openapi_modify_operation(op, c);
        
        E::__openapi_modify_operation(op, c)
    }

    fn __groom_negotiate_content_type(accept: &Accept)
        -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response>
    {
        match T::__groom_negotiate_content_type(accept) {
            Ok(mime) => Ok(mime),
            Err(_) => E::__groom_negotiate_content_type(accept),
        }
    }

    fn __groom_into_response(self, negotiated: Option<&::mime::Mime>) -> axum::response::Response {
        match self {
            Ok(t) => t.__groom_into_response(negotiated),
            Err(e) => e.__groom_into_response(negotiated),
        }
    }

    fn __groom_check_response_codes(context: &str, codes: &mut HTTPCodeSet) {
        T::__groom_check_response_codes(&format!("{context} / Result<Ok, _>"), codes);
        E::__groom_check_response_codes(&format!("{context} / Result<_, Err>"), codes);
    }

    fn __groom_check_response_formats(context: &str, formats: &mut HTTPFormatsSet) {
        let mut ok_formats = HTTPFormatsSet::new();
        let mut err_formats = HTTPFormatsSet::new();
        T::__groom_check_response_formats(&format!("{context} / Result<Ok, _>"), &mut ok_formats);
        E::__groom_check_response_formats(&format!("{context} / Result<_, Err>"), &mut err_formats);
        ok_formats.assert_same_as(&format!("{context} / Result<Ok, _>"), &err_formats);
        formats.merge(&ok_formats);
    }
}
