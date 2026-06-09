use accept_header::Accept;
use ::axum::http::header::HeaderMap;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use http::HeaderName;
use mime::Mime;

pub fn parse_accept_header(headers: &HeaderMap) -> Option<Accept> {
    get_header_as_string(headers, ACCEPT).map(|val| {
        val.parse::<Accept>().expect("Failed to parse header `Accept`.")
    })
}

#[derive(Debug)]
pub enum BodyContentType {
    Json,
    FormUrlEncoded,
}

pub fn parse_content_type_header(headers: &HeaderMap) -> Option<Mime> {
    get_header_as_string(headers, CONTENT_TYPE).and_then(|val| {
        val.parse::<Mime>().ok()
    })
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

fn get_header_as_string(headers: &HeaderMap, header_name: HeaderName) -> Option<String>
{
    headers.get(&header_name).map(|content_type| String::from_utf8(content_type.as_bytes().to_vec())
                    .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
                    .unwrap_or_else(|_| panic!("Failed to read header `{}` as a utf-8 string.", header_name)))
}

fn is_form_url_encoded(mime: &Mime) -> bool {
    *mime == mime::APPLICATION_WWW_FORM_URLENCODED
}

fn is_json(mime: &Mime) -> bool {
    // shamelessly taken from axum::json because their function is private
    

    mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|name| name == "json"))
}
