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

fn is_form_url_encoded(mime: &Mime) -> bool {
    *mime == mime::APPLICATION_WWW_FORM_URLENCODED
}

fn is_json(mime: &Mime) -> bool {
    // shamelessly taken from axum::json because their function is private
    

    mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|name| name == "json"))
}
