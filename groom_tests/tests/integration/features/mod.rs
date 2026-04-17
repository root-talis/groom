///! Tests of separate features

mod dependency_injection;

// region: requests
//

mod request_axum_request_extractor;
mod request_body;
mod request_headers;
mod request_methods;
mod request_path_params;
mod request_query_params;

//
// endregion: requests

// region: responses
//

mod response_content_negotiation;

mod response_struct;

mod response_type_html;
mod response_type_json;
mod response_type_plaintext;

//
// endregion: responses
