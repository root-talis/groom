use accept_header::Accept;
use ::axum::http::header::HeaderMap;
use mime::Mime;

pub fn parse_accept_header(headers: &HeaderMap) -> Option<Accept> {
    headers
        .get(::axum::http::header::ACCEPT)
        .map(|accept_value| {
            let utf_accept = String::from_utf8(accept_value.as_bytes().to_vec())
                .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
                .expect("Accept header failed to be read as a utf-8 string.");

            let accept_parsed: ::accept_header::Accept = utf_accept
                .parse()
                .expect("Accept header failed to parse.");

            accept_parsed
        })
}

pub enum BodyContentType {
    //Form,
    Json,
}

pub fn parse_content_type(headers: &HeaderMap) -> Option<Mime> {
    headers.get(::axum::http::header::CONTENT_TYPE)
        .map_or(None, |content_type| {
            let utf_content_type = String::from_utf8(content_type.as_bytes().to_vec())
                .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
                .expect("Content-Type header failed to be read as a utf-8 string.");

            if let Ok(mime) = utf_content_type.parse::<Mime>() {
                Some(mime)
            } else {
                None
            }
        })
}

pub fn is_json(mime: Mime) -> bool {
    // taken from axum::json since their function is private
    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"));

    is_json_content_type
}

pub fn get_body_content_type(mime: Option<Mime>) -> Option<BodyContentType> {
    if mime.is_none() {
        return None;
    }

    let mime = mime.unwrap();

    if is_json(mime) {
        return Some(BodyContentType::Json)
    }

    None
}
