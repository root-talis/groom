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
    get_header_as_string(headers, ACCEPT, "Accept")?
        .map(|val| {
            val.parse::<Accept>()
                .map_err(|_| HeaderParseError::UnparseableValue("Accept", val))
        })
        .transpose()
}

#[derive(Debug)]
pub enum BodyContentType {
    Json,
    FormUrlEncoded,
}

pub fn parse_content_type_header(headers: &HeaderMap) -> Result<Option<Mime>, HeaderParseError> {
    get_header_as_string(headers, CONTENT_TYPE, "Content-Type")?
        .map(|val| {
            val.parse::<Mime>()
                .map_err(|_| HeaderParseError::UnparseableValue("Content-Type", val))
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

fn get_header_as_string(headers: &HeaderMap, header_name: HeaderName, error_name: &'static str)
    -> Result<Option<String>, HeaderParseError>
{
    headers.get(&header_name)
        .map(|value| {
            std::str::from_utf8(value.as_bytes())
                .map(|s| s.to_owned())
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
}
