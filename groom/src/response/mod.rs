use accept_header::Accept;
use ::axum::response::IntoResponse;
use utoipa::openapi::path::OperationBuilder;

/// Response is the trait that enables enums and structs to turn themselves into HTTP responses
/// and into openapi spec.
pub trait Response {

    fn __openapi_modify_operation(op: OperationBuilder, _c: &mut ComponentsRegistry) -> OperationBuilder;

    /// Consumes the pre-negotiated mime (produced by [`Response::__groom_negotiate_content_type`])
    /// into an HTTP response. `None` means no `Accept` header was sent — the `default_format`
    /// applies, matching the previous no-`Accept` behavior.
    fn __groom_into_response(self, negotiated: Option<&::mime::Mime>) -> ::axum::response::Response;

    /// Negotiates the `Accept` header against this type's supported content types.
    /// Returns the negotiated mime on success (the single negotiation site per request),
    /// or a ready-to-return response (406) when the request cannot be satisfied.
    fn __groom_negotiate_content_type(accept: &Accept)
        -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response>;

    /// Performs runtime checks of response codes of this Response.
    /// Used to detect duplicated codes in composite types like Result<T, E>
    fn __groom_check_response_codes(context: impl ::std::fmt::Display, codes: &mut HTTPCodeSet);

    /// Performs runtime checks of the supported response formats of this Response.
    /// Used to detect format-list mismatches in composite types like Result<T, E>
    /// (both variants must support the same list of formats).
    fn __groom_check_response_formats(context: impl ::std::fmt::Display, formats: &mut HTTPFormatsSet);
}

/// Builds the 406 Not Acceptable response: `Vary: Accept`, text/plain body listing supported mimes.
pub fn not_acceptable(supported_mimes: &[::mime::Mime]) -> ::axum::response::Response {
    // One String build — no intermediate Vec of mime refs (P012 / review option 3).
    let mut body = String::from("Supported content types: ");
    let mut first = true;
    for mime in supported_mimes {
        if !first {
            body.push_str(", ");
        }
        body.push_str(mime.as_ref());
        first = false;
    }
    (
        ::axum::http::StatusCode::NOT_ACCEPTABLE,
        [(::axum::http::header::VARY, "Accept")],
        body,
    ).into_response()
}

/// Builds the 400 Bad Request response for a malformed Accept header (D-08 body text).
pub fn bad_accept_header() -> ::axum::response::Response {
    (::axum::http::StatusCode::BAD_REQUEST, "Invalid Accept header.").into_response()
}

pub mod html_response;
pub use html_response::{HtmlFormat, html_format};
use crate::{extract::ComponentsRegistry, runtime_checks::{HTTPCodeSet, HTTPFormatsSet}};

pub mod result;
