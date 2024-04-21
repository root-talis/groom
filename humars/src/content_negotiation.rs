use accept_header::Accept;
use ::axum::http::header::HeaderMap;

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
