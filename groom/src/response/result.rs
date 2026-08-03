use accept_header::Accept;
use utoipa::openapi::path::OperationBuilder;
use crate::extract::ComponentsRegistry;
use crate::response::Response;
use crate::runtime_checks::{HTTPCodeSet, HTTPFormatsSet};

/// `Result<T, E>` as a [`Response`] when both arms implement [`Response`].
///
/// Hand-written `Response` impls used in `Result<T, E>` must declare the same
/// format list on `T` and `E`. Startup format checks enforce this. Negotiation
/// uses only `T`; there is no fallback to `E` after a 406.
impl<T, E> Response for Result<T, E>
where T: Response, E: Response
{
    fn __openapi_modify_operation(op: OperationBuilder, c: &mut ComponentsRegistry) -> OperationBuilder {
        let op = T::__openapi_modify_operation(op, c);
        
        E::__openapi_modify_operation(op, c)
    }

    fn __groom_negotiate_content_type(accept: &Accept)
        -> ::core::result::Result<Option<&'static ::mime::Mime>, ::axum::response::Response>
    {
        // Format equality (below) proves Ok/Err share the same list, so E cannot
        // accept what T rejected. Negotiate once with T (P009 / D-26).
        T::__groom_negotiate_content_type(accept)
    }

    fn __groom_into_response(self, negotiated: Option<&::mime::Mime>) -> axum::response::Response {
        match self {
            Ok(t) => t.__groom_into_response(negotiated),
            Err(e) => e.__groom_into_response(negotiated),
        }
    }

    fn __groom_check_response_codes(context: impl ::std::fmt::Display, codes: &mut HTTPCodeSet) {
        T::__groom_check_response_codes(format_args!("{context} / Result<Ok, _>"), codes);
        E::__groom_check_response_codes(format_args!("{context} / Result<_, Err>"), codes);
    }

    fn __groom_check_response_formats(context: impl ::std::fmt::Display, formats: &mut HTTPFormatsSet) {
        let mut ok_formats = HTTPFormatsSet::new();
        let mut err_formats = HTTPFormatsSet::new();
        T::__groom_check_response_formats(format_args!("{context} / Result<Ok, _>"), &mut ok_formats);
        E::__groom_check_response_formats(format_args!("{context} / Result<_, Err>"), &mut err_formats);
        ok_formats.assert_same_as(format_args!("{context} / Result<Ok, _>"), &err_formats);
        formats.merge(&ok_formats);
    }
}

#[cfg(test)]
mod p009_negotiate_gate {
    /// Structural gate (D-29): Result negotiate must not retry E after T 406.
    #[test]
    fn result_negotiate_must_not_retry_e_after_t_406() {
        let src = include_str!("result.rs");
        let impl_src = src.split("#[cfg(test)]").next().expect("impl precedes tests");
        let needle = ["E::", "__groom_negotiate_content_type"].concat();
        let e_retry_lines = impl_src
            .lines()
            .filter(|line| {
                let trimmed = line.trim_start();
                !trimmed.starts_with("//") && line.contains(&needle)
            })
            .count();
        assert_eq!(
            e_retry_lines, 0,
            "Result negotiate must call T only (P009/D-29); no E retry after T 406"
        );
    }
}

#[cfg(test)]
mod p006_negotiate_static_ref_gate {
    /// Structural gate (P006): Result blanket matches Option<&'static Mime> signature.
    #[test]
    fn result_negotiate_signature_is_static_mime_ref() {
        let src = include_str!("result.rs");
        let impl_src = src.split("#[cfg(test)]").next().expect("impl precedes tests");
        assert!(
            impl_src.contains("Option<&'static ::mime::Mime>")
                || impl_src.contains("Option<&'static Mime>"),
            "Result Response negotiate must return Option<&'static Mime> (P006)"
        );
    }
}
