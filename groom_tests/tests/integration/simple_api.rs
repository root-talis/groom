use std::collections::HashMap;
use axum::{Extension, Router};
use axum::body::Body;
use axum::http::{HeaderMap, Request, StatusCode};

use utoipa::{OpenApi, openapi::OpenApiBuilder};

use tower::ServiceExt; // for `call`, `oneshot` and `ready`
use http_body_util::BodyExt;

use crate::groom_macros::Controller;

use crate::integration::simple_api::my_api::MyState;

use serde_json::json;

#[cfg(test)]
use pretty_assertions::{assert_eq, /*assert_ne*/};


// region: test bootstrap utils -----------------------------------
//

/// Setup router
fn router() -> Router {
    let router = Router::new();

    my_api::merge_into_router(router)
        .layer(Extension(MyState {
            name: "Luca"
        }))
        .with_state(MyState {
            name: "Victoria"
        })
}

/// Send a GET request
async fn get(url: &str, accept: Option<&'static str>) -> (StatusCode, HeaderMap, String) {
    get_headers(url, accept.map_or(
        HashMap::new(),
        |a| HashMap::from([("accept", a)]))
    ).await
}

/// Send a GET request with custom headers
async fn get_headers(url: &str, headers: HashMap<&'static str, &'static str>) -> (StatusCode, HeaderMap, String) {
    let app = router();

    let mut request = Request::builder().uri(url);

    for h in headers {
        request = request.header(h.0, h.1);
    }

    let request = request.body(Body::empty()).unwrap();

    let response = app
        .oneshot(request)
        .await
        .unwrap()
        ;

    let status = response.status();
    let headers = response.headers().to_owned();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap().to_owned();

    (status, headers, body)
}

/// Send a POST request
async fn post(url: &str, accept: Option<&'static str>, content_type: Option<&'static str>, body: Body) -> (StatusCode, HeaderMap, String) {
    let app = router();

    let mut request = Request::builder().uri(url).method("POST");

    if accept.is_some() {
        request = request.header("accept", accept.unwrap());
    }

    if content_type.is_some() {
        request = request.header("content-type", content_type.unwrap());
    }

    let request = request.body(body).unwrap();

    let response = app
        .oneshot(request)
        .await
        .unwrap()
    ;

    let status = response.status();
    let headers = response.headers().to_owned();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body).unwrap().to_owned();

    (status, headers, body)
}

/// Assert Content-Type value in response
fn assert_content_type(headers: &HeaderMap, expected: &str) {
    assert_eq!(headers.get("content-type").expect("should respond with content-type header"), expected);
}

/// Assert that Content-Type is missing from response
fn assert_no_content_type(headers: &HeaderMap) {
    assert_eq!(headers.get("content-type"), None, "should respond without content-type header");
}

//
// endregion: test bootstrap utils --------------------------------------

// region: api implementation to test ------------------------------------
//

/// This is a huge API with to test every aspect of Groom
#[Controller(state_type = crate::integration::simple_api::my_api::MyState)]
mod my_api {
    use axum::Extension;
    use axum::extract::{Path, Query, Request, State};
    use axum::http::HeaderMap;
    use axum::response::IntoResponse;

    use utoipa::{ToSchema, PartialSchema};

    use groom::extract::GroomExtractor;
    use groom::response::Response;
    use groom::response::html_format;
    use groom::schema::GroomSchema;

    use crate::groom_macros::{Response, DTO};
    use groom_macros::RequestBody;

    #[derive(Clone)]
    pub struct MyState {
        pub name: &'static str
    }

    // region: dumb handlers ---------------------------------------------
    //

    // --- BEGIN GET / ---

    #[Route(method="get", path="/")]
    pub async fn root() -> RootResponse {
        RootResponse::Ok
    }

    #[Response()]
    pub enum RootResponse {
        #[Response(code = 202)] // todo: allow using constants like axum::http::Status::ACCEPTED?
        Ok,
    }

    // --- END GET / ---

    // --- BEGIN GET /hello-world ---

    /// This method says "hello, world!"
    /// 
    /// Ha-ha, classic.
    #[Route(method="get", path="/hello-world")]
    pub async fn hello_world() -> HelloWorldResponse {
        HelloWorldResponse::Ok("hello, world!".into())
    }

    /// POST hello world
    #[Route(method="post", path="/hello-world")]
    pub async fn post_hello_world() -> HelloWorldResponse {
        HelloWorldResponse::Ok("hello, world!".into())
    }

    #[Response(format(plain_text))]
    pub enum HelloWorldResponse {
        #[Response()]
        Ok(String),
    }

    // --- END GET /hello-world ---

    //
    // endregion: dumb handlers -----------------------------------------

    // region: request consumption --------------------------------------
    //

    #[DTO(request)]
    #[serde(rename="RqConsPathStructRenamed")]
    pub struct RqConsPathStruct {
        pub team_id: String,
        pub user_id: i32,
    }

    #[DTO(request)]
    pub struct RqConsQueryParams {
        #[serde(rename="first_name_renamed")]
        first_name: String,
        last_name: Option<String>,
    }

    #[DTO(request)]
    pub struct RqConsQueryParams2 {
        title: Option<String>,
    }

    #[Response(format(plain_text))]
    pub enum RqConsQueryResponse {
        /// A quick brown fox jumped over a lazy dog.
        #[Response()]
        Ok(String),

        /// What did you say?
        /// 
        /// Bad request bro.
        #[Response(code = 400)]
        BadRequest(String),
    }

    #[Response(format(plain_text))]
    pub enum RqConsGenericResponse {
        #[Response()]
        Ok(String),
    }

    // Request consumption: Query<struct> (stuff after `?`)
    #[Route(method = "get", path = "/greet")]
    pub async fn rq_cons_query_struct(query: Query<RqConsQueryParams>, query2: Query<RqConsQueryParams2>) -> RqConsQueryResponse {
        if query.first_name.is_empty() {
            RqConsQueryResponse::BadRequest("Empty name".into())
        } else {
            RqConsQueryResponse::Ok(format!(
                "Hello, {}{}!",
                query2.title.clone().map_or_else(
                    || "".into(),
                    |t| format!("{} ", t)
                ),
                query.last_name.clone().map_or_else(
                    || query.first_name.clone(),
                    |last_name| format!("{} {}", query.first_name, last_name) 
                    // in RqConsQueryParams,  last_name is Option<String>
                )
            ))
        }
    }

    /*
    // Request consumption: Query<HashMap<String, String>> (stuff after `?`)
    #[Route(method = "get", path = "/greet_2")]
    async fn rq_cons_query_hashmap(query: Query<HashMap<String, String>>) -> RqConsQueryResponse {
        match (query.get("first_name"), query.get("last_name")) {
            (Some(first_name), Some(last_name)) => 
                RqConsQueryResponse::Ok(format!("Hello, {first_name} {last_name}!")),

            (_,_) => // I am a grown adult, I'm not gonna joke like this... 
                RqConsQueryResponse::BadRequest("ass".into()), // Well shit.
        }
    }
    */

    /*
    // Request consumption: Path<tuple>
    #[Route(method = "get", path = "/user/:user_id/team/:team_id")]
    async fn rq_cons_path_tuple(Path((user_id, team_id)): Path<(i32, String)>) -> RqConsPathResponse {
        RqConsPathResponse::Ok(format!("{} -> {}", user_id, team_id))
    }*/

    // Request consumption: Path<struct>
    #[Route(method = "get", path = "/team/:team_id/user/:user_id")]
    async fn rq_cons_path_struct(Path(team): Path<RqConsPathStruct>) -> RqConsGenericResponse {
        RqConsGenericResponse::Ok(format!("{} -> {}", team.user_id, team.team_id))
    }

    // Request consumption: String body
    #[Route(method = "post", path = "/string_body")]
    async fn rq_cons_string_body(body: String) -> RqConsGenericResponse {
        RqConsGenericResponse::Ok(format!("body: {body}"))
    }

    // Request consumption: Bytes body
    #[Route(method = "post", path = "/bytes_body")]
    async fn rq_cons_bytes_body(body: axum::body::Bytes) -> RqConsGenericResponse {
        RqConsGenericResponse::Ok(format!("bytes count: {}", body.iter().count()))
    }

    // Request consumption: ImageJpeg body
    groom::binary_request_body!(ImageJpeg with content_type "image/jpeg");

    #[Route(method = "post", path = "/image_body")]
    async fn rq_cons_image_body(body: ImageJpeg) -> RqConsGenericResponse {
        RqConsGenericResponse::Ok(format!("bytes count: {}", body.0.iter().count()))
    }

    // todo: Request bodies with content-type negotiation:
        // done: Json       - as a separate feature
        // done: Form       - as a separate feature
        // todo: Multipart  - as a separate feature
        // todo: XML        - as a separate feature
        // todo: BSON       - as a separate feature
        // todo: CBOR       - as a separate feature

    /// Request body as a named struct.
    #[RequestBody(format(json, url_encoded))]
    pub struct MultiFormatRequestBody {
        name: String,
        age: Option<u8>,
    }

    #[Route(method = "post", path = "/multi_format")]
    async fn rq_cons_multi_format_body(body: MultiFormatRequestBody) -> RqConsGenericResponse {
        RqConsGenericResponse::Ok(format!("someone named {} is {} years old", body.name, body.age.map_or(
            "who knows how many".into(),
            |v| format!("{v}")
        )))
    }

    /// Some DTO
    #[DTO(request)]
    pub struct MultiFormatDto {
        name: String,
        age: Option<u8>,
    }

    /// Request body as an unnamed struct that wraps around a DTO
    #[RequestBody(format(json, url_encoded))]
    pub struct MultiFormatRequestBodyDto(MultiFormatDto);

    #[Route(method = "post", path = "/multi_format_dto")]
    async fn rq_cons_multi_format_body_dto(MultiFormatRequestBodyDto(body): MultiFormatRequestBodyDto) -> RqConsGenericResponse {
        RqConsGenericResponse::Ok(format!("someone named {} is {} years old", body.name, body.age.map_or(
            "who knows how many".into(),
            |v| format!("{v}")
        )))
    }

    // todo: Multipart (for multipart/form-data)

    // todo: HeaderMap

    // todo: Extension<State>

    // todo: Method

    // todo: multiple HTTP methods in one handler

    // todo: extractors wrapped in Option<> and Result<>

    #[Route(method = "get", path = "/request-extractor")]
    async fn rq_cons_request(req: Request) -> RqConsGenericResponse {
        let uri = req.uri().to_string();
        RqConsGenericResponse::Ok(format!("uri: {uri}"))
    }

    #[Route(method = "get", path = "/header-map")]
    async fn rq_cons_header_map(h: HeaderMap) -> RqConsGenericResponse {
        let token = h.get("x-access-token");
        RqConsGenericResponse::Ok(format!(
            "token: {}",
            token.map_or("none", |t| t.to_str().unwrap())
        ))
    }

    #[Route(method = "get", path = "/extension")]
    async fn rq_cons_extension(e: Extension<MyState>) -> RqConsGenericResponse {
        RqConsGenericResponse::Ok(format!("name from extension: {}", e.name))
    }

    #[Route(method = "get", path = "/state")]
    async fn rq_cons_state(e: State<MyState>) -> RqConsGenericResponse {
        RqConsGenericResponse::Ok(format!("name from state: {}", e.name))
    }

    //
    // endregion: request consumption ---------------------------------

    // region: responses ------------------------------------------------
    //

    #[Route(method = "get", path = "/html")]
    async fn resp_html() -> GetHtmlBodyResult {
        GetHtmlBodyResult::Ok("<h1>Hello, world!</h1>")
    }

    #[Response(format(html))]
    pub enum GetHtmlBodyResult {
        /// Home page
        #[Response()]
        Ok(&'static str),
        
        /// Not Found
        #[Response(code = 404)]
        #[allow(dead_code)]
        NotFound(String),
    }

    // --

    #[Route(method = "get", path = "/html-or-text")]
    async fn resp_html_or_text() -> GetHtmlOrTextBodyResult {
        GetHtmlOrTextBodyResult::Ok(PageData("Hello, world!".to_string()))
    }

    #[DTO(response)]
    pub struct PageData(String);

    impl Into<String> for PageData {
        fn into(self) -> String {
            self.0
        }
    }
    html_format!(PageData, self {
        format!("<h1>{}</h1>", self.0)
    });

    #[Response(format(plain_text, html), default_format="html")]
    pub enum GetHtmlOrTextBodyResult {
        /// Home page
        #[Response()]
        Ok(PageData),
        //Ok(&'static str),
    }
    
    // --

    #[Route(method = "get", path = "/struct-in-enum")]
    async fn resp_struct_body() -> GetStructBodyResult {
        GetStructBodyResult::Ok(StructBodyOfEnum {
            success: true,
            message: None,
        })
    }

    #[DTO(response)]
    pub struct StructBodyOfEnum {
        pub success: bool,
        pub message: Option<String>,
    }

    #[Response(format(json))]
    pub enum GetStructBodyResult {
        #[Response()]
        Ok(StructBodyOfEnum),
    }

    // --

    #[Route(method = "get", path = "/html-or-json")]
    async fn resp_html_or_json() -> GetHtmlOrJsonBodyResult {
        GetHtmlOrJsonBodyResult::Ok(HtmlOrJsonDataObject{
            status: "open",
            status_timestamp: 1234567890,
        })
    }
    
    #[DTO(response)]
    pub struct HtmlOrJsonDataObject {
        pub status: &'static str,
        pub status_timestamp: u64,
    }

    html_format!(HtmlOrJsonDataObject, self {
        format!("<div><div>status: {}</div><div>status timestamp: {}</div></div>", self.status, self.status_timestamp)
    });

    #[Response(format(html, json), default_format="json")]
    pub enum GetHtmlOrJsonBodyResult {
        /// Current status
        #[Response()]
        Ok(HtmlOrJsonDataObject),
        //Ok(&'static str),
    }

    // todo: Response bodies:
    //     todo: JSON - as a separate feature
    //     todo: XML  - as a separate feature
    //     todo: BSON - as a separate feature
    //     todo: CBOR - as a separate feature

    /// Named struct as a response
    #[Response(format(plain_text, html, json), default_format="plain_text", code=418)]
    pub struct NamedStructResult {
        pub is_alive: bool,
    }

    impl From<NamedStructResult> for String {
        fn from(value: NamedStructResult) -> Self {
            format!("alive: {}", if value.is_alive {
                "true"
            } else {
                "false"
            })
        }
    }

    html_format!(NamedStructResult, self {
        format!("<div>alive: {}</div>", if self.is_alive {
            "true"
        } else {
            "false"
        })
    });

    #[Route(method = "get", path = "/named-struct")]
    async fn resp_named_struct() -> NamedStructResult {
        NamedStructResult { is_alive: true }
    }

    // --

    /// Named struct as a plaintext-only response
    #[Response(format(plain_text), code=418)]
    pub struct NamedStructOnlyPlaintextResult{
        v: String,
    }

    impl From<NamedStructOnlyPlaintextResult> for String {
        fn from(value: NamedStructOnlyPlaintextResult) -> Self {
            format!("v: {}", value.v)
        }
    }

    #[Route(method = "get", path = "/named-struct-only-plaintext")]
    async fn resp_named_struct_only_plaintext() -> NamedStructOnlyPlaintextResult {
        NamedStructOnlyPlaintextResult{ v: "hello, world".into() }
    }

    // --

    /// Unnamed struct as a response
    #[Response(format(plain_text, html, json), default_format="plain_text", code=418)]
    pub struct UnnamedStructResult(String);

    html_format!(UnnamedStructResult, self {
        format!("<div>{}</div>", self.0)
    });

    #[Route(method = "get", path = "/unnamed-struct")]
    async fn resp_unnamed_struct() -> UnnamedStructResult {
        UnnamedStructResult("hello, world".into())
    }

    // --

    /// Unnamed struct as a plaintext-only response
    #[Response(format(plain_text), code=418)]
    pub struct UnnamedStructOnlyPlaintextResult(String);

    #[Route(method = "get", path = "/unnamed-struct-only-plaintext")]
    async fn resp_unnamed_struct_only_plaintext() -> UnnamedStructOnlyPlaintextResult {
        UnnamedStructOnlyPlaintextResult("hello, world".into())
    }

    // --

    /// Unnamed struct DTO
    #[DTO(response)]
    pub struct UnnamedStructDto {
        v: String,
    }

    /// Unnamed struct DTO result
    #[Response(format(json), code=418)]
    pub struct UnnamedStructDtoResult(UnnamedStructDto);

    #[Route(method = "get", path = "/unnamed-struct-dto")]
    async fn resp_unnamed_struct_dto() -> UnnamedStructDtoResult {
        UnnamedStructDtoResult(UnnamedStructDto{ v: "hello, world".into() })
    }

    // --

    /// Unit struct
    #[Response(code=418)]
    pub struct UnitStruct;

    #[Route(method = "get", path = "/unit-struct")]
    async fn resp_unit_struct() -> UnitStruct {
        UnitStruct
    }

    // --
    #[Response(code = 202)]
    pub struct ResultSuccessResponseStruct;

    #[Response(code = 404)]
    pub struct ResultErrorResponseStruct;

    #[Response(format(plain_text))]
    pub enum ResultErrorResponseEnum {
        #[Response(code = 404)]
        #[allow(dead_code)]
        NotFound,

        #[Response(code = 400)]
        NoAccess(String),
    }

    #[Route(method = "get", path = "/result-struct-struct")]
    async fn resp_result_struct_struct() -> Result<ResultSuccessResponseStruct, ResultErrorResponseStruct> {
        Ok(ResultSuccessResponseStruct)
    }

    #[Route(method = "get", path = "/result-struct-enum")]
    async fn resp_result_struct_enum() -> Result<ResultSuccessResponseStruct, ResultErrorResponseEnum> {
        // todo: ResultSuccessResponseStruct and ResultErrorResponseEnum may have overlapping response codes.
        //       these should be somehow deduplicated in compile time.

        Err(ResultErrorResponseEnum::NoAccess("ip blocked".into()))
    }

    //
    // endregion: responses ---------------------------------------------

    #[allow(dead_code)]
    async fn not_a_handler() {}
}

//
// endregion: api implementation to test --------------------------

// region: tests --------------------------------------------------
//

/// In this test we check how OpenAPI spec is generated.
#[test]
fn api_doc() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "t", description = "d", license(name = "n"), version = "0.0.0", contact(name = "name", email = "mail@example.com"))
    )]
    struct ApiDoc;

    let api = OpenApiBuilder::from(ApiDoc::openapi());
    let api = my_api::merge_into_openapi_builder(api);

    let json = api.build().to_json().expect("expected a valid json string");

    eprintln!("generated openapi definition as json:\n---\n{json}\n---");

    // warning: huge JSON spec below. use folds.
    // region: huge json spec ---
    assert_eq!(
        json.parse::<serde_json::Value>().expect("expected a parsed json"),
        json!({
            "openapi": "3.0.3",
            "info": {
                "title": "t",
                "description": "d",
                "license": {"name": "n"},
                "version": "0.0.0",
                "contact": {
                    "name": "name",
                    "email": "mail@example.com"
                }
            },
            "paths": {
                "/": {
                    "get": {
                        "responses": {
                            "202": {
                                "description": ""
                            }
                        }
                    },
                },
                "/hello-world": {
                    "get": {
                        "summary": "This method says \"hello, world!\"",
                        "description": "Ha-ha, classic.",
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "post": {
                        "summary": "POST hello world",
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/html": {
                    "get": {
                        "responses": {
                            "200": {
                                "description": "Home page",
                                "content": {
                                    "text/html; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                }
                            },
                            "404": {
                                "description": "Not Found",
                                "content": {
                                    "text/html; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
                "/html-or-text": {
                    "get": {
                        "responses": {
                            "200": {
                                "description": "Home page",
                                "content": {
                                    "text/html; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    },
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/html-or-json": {
                    "get": {
                        "responses": {
                            "200": {
                                "description": "Current status",
                                "content": {
                                    "text/html; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    },
                                    "application/json": {
                                        "schema": {
                                            "type": "object",
                                            "properties": {
                                                "status": {
                                                    "type": "string"
                                                },
                                                "status_timestamp": {
                                                    "format": "int64",
                                                    "minimum": 0,
                                                    "type": "integer"
                                                }
                                            },
                                            "required": ["status", "status_timestamp"]
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/struct-in-enum": {
                    "get": {
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "type": "object",
                                            "properties": {
                                                "message": {
                                                    "nullable": true,
                                                    "type": "string",
                                                },
                                                "success": {
                                                    "type": "boolean"
                                                }
                                            },
                                            "required": ["success"]
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/greet": {
                    "get": {
                      "parameters": [
                        {
                          "name": "RqConsQueryParams",
                          "in": "query",
                          "required": true,
                          "schema": {
                            "type": "object",
                            "required": [
                              "first_name_renamed"
                            ],
                            "properties": {
                              "first_name_renamed": {
                                "type": "string"
                              },
                              "last_name": {
                                "type": "string",
                                "nullable": true
                              }
                            }
                          }
                        },
                        {
                          "name": "RqConsQueryParams2",
                          "in": "query",
                          "required": true,
                          "schema": {
                            "type": "object",
                            "properties": {
                              "title": {
                                "type": "string",
                                "nullable": true
                              }
                            }
                          }
                        }
                      ],
                      "responses": {
                            "200": {
                                "description": "A quick brown fox jumped over a lazy dog.",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            },
                            "400": {
                                "description": "What did you say?\n\nBad request bro.",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            }
                      }
                    }
                },
                "/team/:team_id/user/:user_id": {
                    "get": {
                        "parameters": [
                            {
                                "in": "path",
                                "name": "RqConsPathStruct",
                                "required": true,
                                "schema": {
                                    "properties": {
                                        "team_id": { "type": "string" },
                                        "user_id": { "type": "integer", "format": "int32" },
                                    },
                                    "required": ["team_id", "user_id"],
                                    "type": "object",
                                }
                            },
                        ],
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
                "/named-struct": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "description": "Named struct as a response",
                                            "properties": {
                                                "is_alive": {
                                                    "type": "boolean"
                                                }
                                            },
                                            "required": ["is_alive"],
                                            "type": "object"
                                        }
                                    },
                                    "text/html; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    },
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": "Named struct as a response"
                            }
                        }
                    }
                },
                "/named-struct-only-plaintext": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": "Named struct as a plaintext-only response"
                            }
                        }
                    }
                },
                "/unnamed-struct": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    },
                                    "text/html; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    },
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    },
                                },
                                "description": "Unnamed struct as a response"
                            }
                        }
                    }
                },
                "/unnamed-struct-dto": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "description": "Unnamed struct DTO",
                                            "properties": {
                                                "v": {
                                                    "type": "string"
                                                }
                                            },
                                            "required": ["v"],
                                            "type": "object"
                                        }
                                    }
                                },
                                "description": "Unnamed struct DTO result"
                            }
                        }
                    }
                },
                "/unnamed-struct-only-plaintext": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    },
                                },
                                "description": "Unnamed struct as a plaintext-only response"
                            }
                        }
                    }
                },
                "/unit-struct": {
                    "get": {
                        "responses": {
                            "418": {
                                "description": "Unit struct"
                            }
                        }
                    }
                },
                "/result-struct-enum": {
                    "get": {
                        "responses": {
                            "202": {
                                "description": ""
                            },
                            "400": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": ""
                            },
                            "404": {
                                "description": ""
                            }
                        }
                    }
                },
                "/result-struct-struct": {
                    "get": {
                        "responses": {
                            "202": {
                                "description": ""
                            },
                            "404": {
                                "description": ""
                            }
                        }
                    }
                },
                "/string_body": {
                    "post": {
                        "requestBody": {
                            "required": true,
                            "content": {
                                "text/plain; charset=utf-8": {
                                    "schema": {
                                        "type": "string"
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            },
                        }
                    },
                },
                "/bytes_body": {
                    "post": {
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/octet-stream": {
                                    "schema": {
                                        "type": "string",
                                        "format": "binary",
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            },
                        }
                    },
                },
                "/image_body": {
                    "post": {
                        "requestBody": {
                            "required": true,
                            "content": {
                                "image/jpeg": {
                                    "schema": {
                                        "type": "string",
                                        "format": "binary",
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            },
                        }
                    },
                },
                "/multi_format": {
                    "post": {
                        "requestBody": {
                            "required": true,
                            "description": "Request body as a named struct.",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "description": "Request body as a named struct.",
                                        "properties": {
                                            "age": {
                                                "format": "int32",
                                                "minimum": 0,
                                                "nullable": true,
                                                "type": "integer"
                                            },
                                            "name": {
                                                "type": "string"
                                            }
                                        },
                                        "required": [
                                            "name"
                                        ],
                                        "type": "object"
                                    }
                                },
                                "application/x-www-form-urlencoded": {
                                    "schema": {
                                        "description": "Request body as a named struct.",
                                        "properties": {
                                            "age": {
                                                "format": "int32",
                                                "minimum": 0,
                                                "nullable": true,
                                                "type": "integer"
                                            },
                                            "name": {
                                                "type": "string"
                                            }
                                        },
                                        "required": [
                                            "name"
                                        ],
                                        "type": "object"
                                    }
                                },
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            },
                        }
                    },
                },
                "/multi_format_dto": {
                    "post": {
                        "requestBody": {
                            "required": true,
                            "description": "Request body as an unnamed struct that wraps around a DTO",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "description": "Some DTO",
                                        "properties": {
                                            "age": {
                                                "format": "int32",
                                                "minimum": 0,
                                                "nullable": true,
                                                "type": "integer"
                                            },
                                            "name": {
                                                "type": "string"
                                            }
                                        },
                                        "required": [
                                            "name"
                                        ],
                                        "type": "object"
                                    }
                                },
                                "application/x-www-form-urlencoded": {
                                    "schema": {
                                        "description": "Some DTO",
                                        "properties": {
                                            "age": {
                                                "format": "int32",
                                                "minimum": 0,
                                                "nullable": true,
                                                "type": "integer"
                                            },
                                            "name": {
                                                "type": "string"
                                            }
                                        },
                                        "required": [
                                            "name"
                                        ],
                                        "type": "object"
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        }
                                    }
                                }
                            },
                        }
                    },
                },
                "/request-extractor": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": ""
                            }
                        }
                    }
                },
                "/header-map": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": ""
                            }
                        }
                    }
                },
                "/extension": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": ""
                            }
                        }
                    }
                },
                "/state": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": ""
                            }
                        }
                    }
                },
            }
        })
    );
    // endregion: huge json spec
}

/// In this test content-negotiation should ignore any Accept header's value
/// because no Response variant has a body anyway.
#[tokio::test]
pub async fn test_root_accept_anything() {
    let (status, headers, body) = get("/", Some("text/plain")).await;

    assert_eq!(body, "");
    assert_no_content_type(&headers);
    assert_eq!(status, StatusCode::ACCEPTED);

    let (status, headers, body) = get("/", Some("something/stupid")).await;

    assert_eq!(body, "");
    assert_no_content_type(&headers);
    assert_eq!(status, StatusCode::ACCEPTED);
}

/// In this test content-negotiation should ignore the absence of Accept header
/// because no Response variant has a body anyway.
#[tokio::test]
pub async fn test_root_no_accept_header() {
    let (status, headers, body) = get("/", None).await;

    assert_eq!(body, "");
    assert_no_content_type(&headers);
    assert_eq!(status, StatusCode::ACCEPTED);
}

/// In this test content-negotiation should see the matching Accept header and return the response.
#[tokio::test]
pub async fn test_hello_world() {
    let (status, headers, body) = get("/hello-world", Some("text/plain")).await;

    assert_eq!(body, "hello, world!");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);
}

/// In this test content-negotiation should notice the absence of Accept header
/// and return the only content-type specified without failing.
#[tokio::test]
pub async fn test_hello_world_no_accept_header() {
    let (status, headers, body) = get("/hello-world", None).await;

    assert_eq!(body, "hello, world!");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);
}

/// In this test we check how Query parameters are read when several structs are mapped to Query.
#[tokio::test]
pub async fn test_query_struct() {
    let (status, headers, body) = get("/greet?first_name_renamed=", Some("text/plain")).await;
    assert_eq!(body, "Empty name");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let (status, headers, body) = get("/greet?first_name_renamed=Max", Some("text/plain")).await;
    assert_eq!(body, "Hello, Max!");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = get("/greet?last_name=Doe&first_name_renamed=John", Some("text/plain")).await;
    assert_eq!(body, "Hello, John Doe!");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = get("/greet?first_name_renamed=John&title=Sir", Some("text/plain")).await;
    assert_eq!(body, "Hello, Sir John!");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = get("/greet?last_name=Backsword&title=Sir&first_name_renamed=John", Some("text/plain")).await;
    assert_eq!(body, "Hello, Sir John Backsword!");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);
}

/*
#[tokio::test]
pub async fn test_query_struct_hashmap() {
    let (status, body) = get("/greet_2?first_name=").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "ass");

    let (status, body) = get("/greet_2?last_name=Doe&first_name=John").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello, John Doe!");

    let (status, body) = get("/greet_2?last_name=Backsword&title=Sir&first_name=John").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello, John Backsword!");
}
*/

/// In this test we check how Path parameters are read.
#[tokio::test]
pub async fn test_path() {
    let (status, headers, body) = get("/team/7/user/1", Some("text/plain")).await;
    assert_eq!(body, "1 -> 7");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = get("/team/Hitchhikers/user/42", Some("text/plain")).await;
    assert_eq!(body, "42 -> Hitchhikers");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);
}

/// In this test we check how JSON body is returned.
#[tokio::test]
pub async fn test_struct_body() {
    let (status, headers, body) = get("/struct-in-enum", Some("application/json")).await;
    assert_eq!(body, "{\"success\":true,\"message\":null}");
    assert_content_type(&headers, "application/json");
    assert_eq!(status, StatusCode::OK);
}

/// In this test we check how HTML body is returned.
#[tokio::test]
pub async fn test_html() {
    let (status, headers, body) = get("/html", Some("text/html")).await;
    assert_eq!(body, "<h1>Hello, world!</h1>");
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(status, StatusCode::OK);
}

/// In this test we check how content-negotiation chooses appropriate serialization of a struct
/// between String and Html.
/// Note how HTML response has <h1> tags around text.
#[tokio::test]
pub async fn test_html_or_text() {
    let (status, headers, body) = get("/html-or-text", Some("text/html")).await;
    assert_eq!(body, "<h1>Hello, world!</h1>");
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = get("/html-or-text", Some("text/plain")).await;
    assert_eq!(body, "Hello, world!");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);
}

/// In this test we check how content-negotiation chooses appropriate serialization of a struct
/// between String and Html based on weights.
#[tokio::test]
pub async fn test_html_or_text_weights() {
    // First content-type has priority
    let (status, headers, body) = get("/html-or-text", Some("text/plain, text/html")).await;
    assert_eq!(body, "Hello, world!");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    // First content-type has priority
    let (status, headers, body) = get("/html-or-text", Some("text/html, text/plain")).await;
    assert_eq!(body, "<h1>Hello, world!</h1>");
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    // Weights have higher priority over position.
    let (status, headers, body) = get("/html-or-text", Some("text/plain;q=0.8, text/html;q=0.9")).await;
    assert_eq!(body, "<h1>Hello, world!</h1>");
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    // Weights are not being prioritized by position in reverse order.
    let (status, headers, body) = get("/html-or-text", Some("text/plain;q=0.9, text/html;q=0.8")).await;
    assert_eq!(body, "Hello, world!");
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    // Inappropriate content-type is ignored.
    let (status, headers, body) = get("/html-or-text", Some("text/html, application/xhtml+xml, application/xml;q=0.9, */*;q=0.8")).await;
    assert_eq!(body, "<h1>Hello, world!</h1>");
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    // Inappropriate content-type is ignored even when placed first.
    let (status, headers, body) = get("/html-or-text", Some("application/xhtml+xml, text/html, application/xml;q=0.9, */*;q=0.8")).await;
    assert_eq!(body, "<h1>Hello, world!</h1>");
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    // HTML has higher priority over plain text when content-type */* is specified.
    let (status, headers, body) = get("/html-or-text", Some("*/*")).await;
    assert_eq!(body, "<h1>Hello, world!</h1>");
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(status, StatusCode::OK);

    // HTML has higher priority over plain text when no Accept header is
    let (status, headers, body) = get("/html-or-text", None).await;
    assert_eq!(body, "<h1>Hello, world!</h1>");
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(status, StatusCode::OK);
}

/// In this test we check how content-negotiation chooses appropriate serialization of a struct
/// between Json and Html.
/// This might be useful if the same backend should be used for both JSON API and HTML page rendering (e.g. for HTMX maybe).
#[tokio::test]
pub async fn test_html_or_json() {
    let (status, headers, body) = get("/html-or-json", Some("text/html")).await;
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(body, "<div><div>status: open</div><div>status timestamp: 1234567890</div></div>");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = get("/html-or-json", Some("application/json")).await;
    assert_content_type(&headers, "application/json");
    assert_eq!(body, "{\"status\":\"open\",\"status_timestamp\":1234567890}");
    assert_eq!(status, StatusCode::OK);
}


/// Testing responses that are defined as a named struct
#[tokio::test]
pub async fn test_get_named_struct() {
    let (status, headers, body) = get("/named-struct", None).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "alive: true");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, headers, body) = get("/named-struct", Some("text/plain")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "alive: true");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, headers, body) = get("/named-struct", Some("text/html")).await;
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(body, "<div>alive: true</div>");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, headers, body) = get("/named-struct", Some("application/json")).await;
    assert_content_type(&headers, "application/json");
    assert_eq!(body, "{\"is_alive\":true}");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);
}

/// Testing responses that are defined as an unnamed struct with single content type
#[tokio::test]
pub async fn test_get_named_struct_single_content_type() {
    let (status, headers, body) = get("/named-struct-only-plaintext", None).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "v: hello, world");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, headers, body) = get("/named-struct-only-plaintext", Some("text/plain")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "v: hello, world");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, _, _) = get("/named-struct-only-plaintext", Some("text/html")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let (status, _, _) = get("/named-struct-only-plaintext", Some("application/json")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

/// Testing responses that are defined as an unnamed struct
#[tokio::test]
pub async fn test_get_unnamed_struct() {
    let (status, headers, body) = get("/unnamed-struct", None).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "hello, world");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, headers, body) = get("/unnamed-struct", Some("text/plain")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "hello, world");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, headers, body) = get("/unnamed-struct", Some("text/html")).await;
    assert_content_type(&headers, "text/html; charset=utf-8");
    assert_eq!(body, "<div>hello, world</div>");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, headers, body) = get("/unnamed-struct", Some("application/json")).await;
    assert_content_type(&headers, "application/json");
    assert_eq!(body, "\"hello, world\"");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);
}

/// Testing responses that are defined as an unnamed struct with single content type
#[tokio::test]
pub async fn test_get_unnamed_struct_single_content_type() {
    let (status, headers, body) = get("/unnamed-struct-only-plaintext", None).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "hello, world");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, headers, body) = get("/unnamed-struct-only-plaintext", Some("text/plain")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "hello, world");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, _, _) = get("/unnamed-struct-only-plaintext", Some("text/html")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let (status, _, _) = get("/unnamed-struct-only-plaintext", Some("application/json")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

/// Testing responses that are defined as an unnamed struct with DTO body
#[tokio::test]
pub async fn test_get_unnamed_struct_dto() {
    let (status, headers, body) = get("/unnamed-struct-dto", None).await;
    assert_content_type(&headers, "application/json");
    assert_eq!(body, "{\"v\":\"hello, world\"}");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, headers, body) = get("/unnamed-struct-dto", Some("application/json")).await;
    assert_content_type(&headers, "application/json");
    assert_eq!(body, "{\"v\":\"hello, world\"}");
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, _, _) = get("/unnamed-struct-dto", Some("text/html")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let (status, _, _) = get("/unnamed-struct-dto", Some("text/plain")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

/// Testing responses that are defined as a unit struct
#[tokio::test]
pub async fn test_get_unit_struct() {
    let (status, _, _) = get("/unit-struct", None).await;
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, _, _) = get("/unit-struct", Some("text/plain")).await;
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, _, _) = get("/unit-struct", Some("text/html")).await;
    assert_eq!(status, StatusCode::IM_A_TEAPOT);

    let (status, _, _) = get("/unit-struct", Some("application/json")).await;
    assert_eq!(status, StatusCode::IM_A_TEAPOT);
}


#[tokio::test]
pub async fn test_post_string() {
    let (status, headers, body) = post("/string_body", Some("text/plain"), None, Body::from("hello, world!")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "body: hello, world!");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = post("/string_body", Some("text/plain"), None, Body::empty()).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "body: ");
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
pub async fn test_post_bytes() {
    let (status, headers, body) = post("/bytes_body", Some("text/plain"), None, Body::from("hello, world!")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "bytes count: 13");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = post("/bytes_body", Some("text/plain"), None, Body::empty()).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "bytes count: 0");
    assert_eq!(status, StatusCode::OK);
}


#[tokio::test]
pub async fn test_post_image() {
    let (status, headers, body) = post("/image_body", Some("text/plain"), None, Body::from("hello, world!")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "bytes count: 13");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = post("/image_body", Some("text/plain"), None, Body::empty()).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "bytes count: 0");
    assert_eq!(status, StatusCode::OK);
}

// Request body JSON - named struct
#[tokio::test]
pub async fn test_post_multi_format_json_named_struct() {
    let (status, headers, body) = post("/multi_format", None, Some("application/json"), Body::from("{\"name\": \"Mark\"}")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "someone named Mark is who knows how many years old");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = post("/multi_format", None, Some("application/json"), Body::from("{\"name\": \"Mark\", \"age\": 20}")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "someone named Mark is 20 years old");
    assert_eq!(status, StatusCode::OK);
}

// Request body JSON - DTO wrapper
#[tokio::test]
pub async fn test_post_multi_format_json_unnamed_struct() {
    let (status, headers, body) = post("/multi_format_dto", None, Some("application/json"), Body::from("{\"name\": \"Mark\"}")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "someone named Mark is who knows how many years old");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = post("/multi_format_dto", None, Some("application/json"), Body::from("{\"name\": \"Mark\", \"age\": 20}")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "someone named Mark is 20 years old");
    assert_eq!(status, StatusCode::OK);
}

// Request body url-encoded - named struct
#[tokio::test]
pub async fn test_post_multi_format_url_encoded_named_struct() {
    //
    // RequestBody as a named struct
    //

    let (status, headers, body) = post("/multi_format", None, Some("application/x-www-form-urlencoded"), Body::from("name=Mark")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "someone named Mark is who knows how many years old");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = post("/multi_format", None, Some("application/x-www-form-urlencoded"), Body::from("name=Mark&age=20")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "someone named Mark is 20 years old");
    assert_eq!(status, StatusCode::OK);

}

// Request body url-encoded - DTO wrapper
#[tokio::test]
pub async fn test_post_multi_format_url_encoded_unnamed_struct() {
    let (status, headers, body) = post("/multi_format_dto", None, Some("application/x-www-form-urlencoded"), Body::from("name=Mark")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "someone named Mark is who knows how many years old");
    assert_eq!(status, StatusCode::OK);

    let (status, headers, body) = post("/multi_format_dto", None, Some("application/x-www-form-urlencoded"), Body::from("name=Mark&age=20")).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "someone named Mark is 20 years old");
    assert_eq!(status, StatusCode::OK);
}

// axum::extract::Request
#[tokio::test]
pub async fn test_get_axum_extract_request() {
    let (status, headers, body) = get("/request-extractor?id=123456", None).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "uri: /request-extractor?id=123456");
    assert_eq!(status, StatusCode::OK);
}

// axum::http::HeaderMap
#[tokio::test]
pub async fn test_get_axum_http_header_map() {
    let (status, headers, body) = get_headers("/header-map", HashMap::from([("x-access-token", "123456789")])).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "token: 123456789");
    assert_eq!(status, StatusCode::OK);
}

// axum::extract::Extension
#[tokio::test]
pub async fn test_get_axum_extract_extension() {
    let (status, headers, body) = get("/extension", None).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "name from extension: Luca");
    assert_eq!(status, StatusCode::OK);
}

// axum::extract::State
#[tokio::test]
pub async fn test_get_axum_extract_state() {
    let (status, headers, body) = get("/state", None).await;
    assert_content_type(&headers, "text/plain; charset=utf-8");
    assert_eq!(body, "name from state: Victoria");
    assert_eq!(status, StatusCode::OK);
}

//
// endregion: tests ------------------------------------------------
