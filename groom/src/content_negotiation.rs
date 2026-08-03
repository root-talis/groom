use accept_header::Accept;
use ::axum::http::header::HeaderMap;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use http::HeaderName;
use mime::Mime;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum HeaderParseError {
    #[error("Failed to parse header `{0}` as a valid header value.")]
    UnparseableValue(&'static str, String),

    #[error("Failed to read header `{0}` as a utf-8 string.")]
    NonUtf8HeaderBytes(&'static str),
}

pub fn parse_accept_header(headers: &HeaderMap) -> Result<Option<Accept>, HeaderParseError> {
    get_header_str(headers, ACCEPT, "Accept")?
        .map(|val| {
            val.parse::<Accept>()
                .map_err(|_| HeaderParseError::UnparseableValue("Accept", val.to_owned()))
        })
        .transpose()
}

#[derive(Debug)]
pub enum BodyContentType {
    Json,
    FormUrlEncoded,
}

pub fn parse_content_type_header(headers: &HeaderMap) -> Result<Option<Mime>, HeaderParseError> {
    get_header_str(headers, CONTENT_TYPE, "Content-Type")?
        .map(|val| {
            val.parse::<Mime>()
                .map_err(|_| {
                    HeaderParseError::UnparseableValue("Content-Type", val.to_owned())
                })
        })
        .transpose()
}

pub fn get_body_content_type(mime: Option<Mime>) -> Option<BodyContentType> {
    mime.as_ref()?;

    let mime = mime.unwrap();

    if is_form_url_encoded(&mime) {
        Some(BodyContentType::FormUrlEncoded)
    } else if is_json(&mime) {
        Some(BodyContentType::Json)
    } else {
        None
    }
}

/// Reads a header as borrowed text via `HeaderValue::to_str`.
/// Allocates nothing on the success path; `to_str` failure maps to
/// [`HeaderParseError::NonUtf8HeaderBytes`] (same 400 path as before).
fn get_header_str<'a>(
    headers: &'a HeaderMap,
    header_name: HeaderName,
    error_name: &'static str,
) -> Result<Option<&'a str>, HeaderParseError> {
    headers
        .get(&header_name)
        .map(|value| {
            value
                .to_str()
                .map_err(|_| HeaderParseError::NonUtf8HeaderBytes(error_name))
        })
        .transpose()
}

/// Negotiates `Accept` against supported Mimes, ignoring Mime parameters (charset and
/// others). `accept-header`'s `negotiate()` compares full `Mime`s including
/// parameters, so a `text/plain; charset=utf-8` supported list would otherwise 406 a
/// plain `Accept: text/plain`. The parser pre-sorts `accept.types` by q-weight, so
/// iterating in order preserves priority. When only `*/*` matches (stored in
/// `accept.wildcard`), this uses `default` when that mime is in `supported`
/// (type_/subtype_ match); otherwise it uses the first supported mime.
pub fn negotiate_parameter_insensitive<'a>(
    accept: &Accept,
    supported: &'a [Mime],
    default: Option<&'a Mime>,
) -> Option<&'a Mime> {
    for media_type in &accept.types {
        if let Some(supported) = supported.iter().find(|mime| {
            mime.type_() == media_type.mime.type_()
                && mime.subtype() == media_type.mime.subtype()
        }) {
            return Some(supported);
        }
    }
    if accept.wildcard.is_some() {
        return default
            .and_then(|d| {
                supported.iter().find(|mime| {
                    mime.type_() == d.type_() && mime.subtype() == d.subtype()
                })
            })
            .or_else(|| supported.first());
    }
    None
}

fn is_form_url_encoded(mime: &Mime) -> bool {
    mime.type_() == mime::APPLICATION
        && mime.subtype() == mime::WWW_FORM_URLENCODED
}

fn is_json(mime: &Mime) -> bool {
    // shamelessly taken from axum::json because their function is private
    

    mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|name| name == "json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn get_body_content_type_accepts_bare_form_urlencoded() {
        let mime: Mime = "application/x-www-form-urlencoded".parse().unwrap();
        let result = get_body_content_type(Some(mime));
        assert!(matches!(result, Some(BodyContentType::FormUrlEncoded)));
    }

    #[test]
    fn get_body_content_type_accepts_form_urlencoded_with_charset() {
        let mime: Mime = "application/x-www-form-urlencoded; charset=utf-8"
            .parse()
            .unwrap();
        let result = get_body_content_type(Some(mime));
        assert!(
            matches!(result, Some(BodyContentType::FormUrlEncoded)),
            "form Content-Type with charset must classify as FormUrlEncoded"
        );
    }

    #[test]
    fn parse_accept_header_returns_some_for_valid_accept() {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let result = parse_accept_header(&headers).expect("valid Accept must parse");
        assert!(result.is_some(), "valid Accept must yield Some(Accept)");
    }

    #[test]
    fn parse_content_type_header_returns_some_for_valid_content_type() {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let result =
            parse_content_type_header(&headers).expect("valid Content-Type must parse");
        assert!(result.is_some(), "valid Content-Type must yield Some(Mime)");
    }

    #[test]
    fn parse_accept_header_maps_non_utf8_to_non_utf8_header_bytes() {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_bytes(&[0xff, 0xfe]).expect("raw header bytes"),
        );
        let err = parse_accept_header(&headers).expect_err("non-UTF8 Accept must err");
        assert!(
            matches!(err, HeaderParseError::NonUtf8HeaderBytes("Accept")),
            "non-UTF8 Accept must map to NonUtf8HeaderBytes (same 400 path)"
        );
    }

    #[test]
    fn parse_accept_header_maps_unparseable_to_unparseable_value() {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("%%%not-a-media-type%%%"));
        let err = parse_accept_header(&headers).expect_err("bad Accept must err");
        assert!(
            matches!(
                err,
                HeaderParseError::UnparseableValue("Accept", ref s) if s == "%%%not-a-media-type%%%"
            ),
            "unparseable Accept must map to UnparseableValue with owned text"
        );
    }
}
