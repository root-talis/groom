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
    let mime = mime?;

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

/// True when the client explicitly refuses this media type (`q=0` / `q<=0`).
/// Missing weight is treated as acceptable (RFC 7231 default q=1.0).
fn is_refused_weight(weight: Option<f32>) -> bool {
    matches!(weight, Some(w) if w <= 0.0)
}

/// Negotiates `Accept` against supported Mimes, ignoring Mime parameters (charset and
/// others). `accept-header`'s `negotiate()` compares full `Mime`s including
/// parameters, so a `text/plain; charset=utf-8` supported list would otherwise 406 a
/// plain `Accept: text/plain`. The parser pre-sorts `accept.types` by q-weight, so
/// iterating in order preserves priority. Media types (and `*/*`) with weight
/// `Some(w)` where `w <= 0.0` are skipped as explicit refusals. When only an
/// acceptable `*/*` remains (stored in `accept.wildcard`), this uses `default`
/// when that mime is in `supported` (type_/subtype_ match); otherwise it uses
/// the first supported mime. A refused-only wildcard returns `None` (HTTP 406).
pub fn negotiate_parameter_insensitive<'a>(
    accept: &Accept,
    supported: &'a [Mime],
    default: Option<&'a Mime>,
) -> Option<&'a Mime> {
    for media_type in &accept.types {
        if is_refused_weight(media_type.weight) {
            continue;
        }
        if let Some(supported) = supported.iter().find(|mime| {
            mime.type_() == media_type.mime.type_()
                && mime.subtype() == media_type.mime.subtype()
        }) {
            return Some(supported);
        }
    }
    if accept
        .wildcard
        .as_ref()
        .is_some_and(|w| !is_refused_weight(w.weight))
    {
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

    fn supported_json_html() -> [Mime; 2] {
        [
            "application/json".parse().unwrap(),
            "text/html".parse().unwrap(),
        ]
    }

    /// D-10: concrete JSON with weight 0 must not be selected against [JSON, HTML].
    #[test]
    fn negotiate_skips_concrete_refused_weight() {
        let accept: Accept = "application/json;q=0".parse().unwrap();
        let supported = supported_json_html();
        let chosen = negotiate_parameter_insensitive(&accept, &supported, Some(&supported[0]));
        assert!(
            chosen.is_none() || chosen.map(|m| m.subtype().as_str()) != Some("json"),
            "refused application/json;q=0 must not select JSON; got {chosen:?}"
        );
    }

    /// D-10 / D-08: wildcard-only with weight 0 → None (no default_format fallback).
    #[test]
    fn negotiate_refused_wildcard_only_returns_none() {
        let accept: Accept = "*/*;q=0".parse().unwrap();
        let supported = supported_json_html();
        let chosen = negotiate_parameter_insensitive(&accept, &supported, Some(&supported[0]));
        assert!(
            chosen.is_none(),
            "refused */*;q=0 must return None (406), not default; got {chosen:?}"
        );
    }

    /// D-10: mixed HTML q=1 + JSON q=0 → HTML.
    #[test]
    fn negotiate_mixed_nonzero_type_wins_over_refused() {
        let accept: Accept = "text/html;q=1, application/json;q=0".parse().unwrap();
        let supported = supported_json_html();
        let chosen = negotiate_parameter_insensitive(&accept, &supported, Some(&supported[0]));
        assert_eq!(
            chosen.map(|m| (m.type_().as_str(), m.subtype().as_str())),
            Some(("text", "html")),
            "acceptable HTML must win over refused JSON"
        );
    }

    /// D-10 / D-09: refused JSON + acceptable wildcard → default_format (JSON).
    #[test]
    fn negotiate_refused_json_plus_acceptable_wildcard_uses_default() {
        let accept: Accept = "application/json;q=0, */*".parse().unwrap();
        let supported = supported_json_html();
        let chosen = negotiate_parameter_insensitive(&accept, &supported, Some(&supported[0]));
        assert_eq!(
            chosen.map(|m| (m.type_().as_str(), m.subtype().as_str())),
            Some(("application", "json")),
            "refused JSON + acceptable */* must yield default_format JSON; got {chosen:?}"
        );
    }
}
