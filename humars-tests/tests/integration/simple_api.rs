use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Request, StatusCode};

use utoipa::openapi::path::{OperationBuilder, ParameterBuilder, PathItemBuilder};
use utoipa::openapi::request_body::RequestBodyBuilder;
use utoipa::openapi::{self, ComponentsBuilder, ContentBuilder, PathsBuilder};
use utoipa::ToSchema;
use utoipa::{OpenApi, openapi::OpenApiBuilder};


use tower::ServiceExt; // for `call`, `oneshot` and `ready`
use http_body_util::BodyExt;

use crate::humars_macros::Controller;

// for scratchpad: ------
use crate::humars::extract::HumarsExtractor;
use crate::integration::simple_api::my_api::RqConsPathStruct;
// end for scratchpad ---

use serde_json::json;

#[cfg(test)]
use pretty_assertions::{assert_eq, /*assert_ne*/};


// region: test bootstrap utils -----------------------------------
//

fn router() -> Router {
    my_api::merge_into_router(Router::new())
}

async fn get(url: &str, accept: Option<&'static str>) -> (StatusCode, HeaderMap, String) {
    let app = router();

    let mut request = Request::builder().uri(url);

    if accept.is_some() {
        request = request.header("accept", accept.unwrap());
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

fn assert_content_type(headers: &HeaderMap, expected: &str) {
    assert_eq!(headers.get("content-type").expect("should respond with content-type header"), expected);
}

fn assert_no_content_type(headers: &HeaderMap) {
    assert_eq!(headers.get("content-type"), None, "should respond without content-type header");
}

//
// endregion: test bootstrap utils --------------------------------------

// region: api implementation to test ------------------------------------
//

#[Controller]
mod my_api {
    use axum::extract::{Path, Query};
    use axum::response::IntoResponse;

    use crate::humars_macros::{Response, DTO};
    use humars::extract::HumarsExtractor;
    use humars::response::Response;

    use utoipa::ToSchema;

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
    pub enum RqConsPathResponse {
        #[Response()]
        Ok(String),

        /// Access has been denied.
        #[Response(code = 401)]
        #[allow(dead_code)]
        GoToHell,

        /// Something bad just happened, but it's not your fault (probably).
        #[Response(code = 500)]
        #[allow(dead_code)]
        InternalServerError,
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
    async fn rq_cons_path_struct(Path(team): Path<RqConsPathStruct>) -> RqConsPathResponse {
        RqConsPathResponse::Ok(format!("{} -> {}", team.user_id, team.team_id))
    }
    // todo: validate that in Path<T> that if T is a struct, it implements DTO, make good error message

    // todo: Request bodies:
        // todo: String
        // todo: Bytes
        // todo: axum::Json<Value> - as a separate feature
        // todo: axum::form::Form  - as a separate feature
        // todo: XML               - as a separate feature
        // todo: BSON              - as a separate feature

    // todo: Request

    // todo: HeaderMap

    // todo: Extension<State>

    // todo: Method

    // todo: multiple HTTP methods in one handler

    // todo: extractors wrapped in Option<> and Result<>

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
    impl Into<axum::body::Body> for PageData {
        fn into(self) -> axum::body::Body {
            format!("<h1>{}</h1>", self.0).into()
        }
    }

    #[Response(format(plain_text, html))]
    pub enum GetHtmlOrTextBodyResult {
        /// Home page
        #[Response()]
        Ok(PageData),
        //Ok(&'static str),
    }
    
    // --

    #[Route(method = "get", path = "/struct")]
    async fn resp_struct_body() -> GetStructBodyResult {
        GetStructBodyResult::Ok(StructBody {
            success: true,
            message: None,
        })
    }

    #[DTO(response)]
    pub struct StructBody {
        pub success: bool,
        pub message: Option<String>,
    }

    #[Response(format(json))]
    pub enum GetStructBodyResult {
        #[Response()]
        Ok(StructBody),
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

    impl Into<axum::body::Body> for HtmlOrJsonDataObject {
        fn into(self) -> axum::body::Body {
            format!("<div><div>status: {}</div><div>status timestamp: {}</div></div>", self.status, self.status_timestamp).into()
        }
    }

    #[Response(format(html, json))]
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

    //
    // endregion: responses ---------------------------------------------

    #[allow(dead_code)]
    async fn not_a_handler() {}
}

//
// endregion: api implementation to test --------------------------

// region: tests --------------------------------------------------
//

#[test]
fn api_doc_scratchpad() {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    let api = OpenApiBuilder::from(ApiDoc::openapi());

    // region: should generate this for /hello-world
    let parameter = ParameterBuilder::new()
        .parameter_in(openapi::path::ParameterIn::Query)
        .name("sad")
        .schema(Some(my_api::RqConsQueryParams::schema().1))
        .build()
    ;

    let components = ComponentsBuilder::new()
        .schema("my_component", my_api::RqConsPathStruct::schema().1)
        .build()
    ;

    let content = ContentBuilder::new()
        .schema(components.schemas["my_component"].clone())
        .build()
    ;

    let request_body = RequestBodyBuilder::new()
        .content("application/json", content)
        .build()
    ;

    let op_builder= OperationBuilder::new()
        .parameter(parameter)
        .request_body(Some(request_body))
        .description(Some("description"))
        .summary(Some("summary"));

    let op_builder = axum::extract::Path::<RqConsPathStruct>::__openapi_modify_operation(op_builder);

    let resp = utoipa::openapi::ResponseBuilder::new().build();

    let op_builder = op_builder.response("202", resp);

    let operation = op_builder.build();

    let operation = OperationBuilder::from(operation).build(); 

    let path_item= PathItemBuilder::new()
        .operation(utoipa::openapi::PathItemType::Get, operation)
        .build()
    ;

    let paths = PathsBuilder::new()
        .path("/hello-world", path_item)
        .build()
    ;
    let paths = PathsBuilder::from(paths).build();

    let api = api.paths(paths);
    //let api = api.components(Some(components));

    // endregion: should generate this for /hello-world

    let _json = api.build().to_json().expect("expected a valid json");

    let _breakpoint = false;

    let hm = ::axum::http::header::HeaderMap::new();
    let _has_header = hm.get(::axum::http::header::ACCEPT);
}

/// In this test we check how OpenAPI spec is generated.
#[test]
fn api_doc() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "t", description = "d", license(name = "n"), version = "0.0.0")
    )]
    struct ApiDoc;

    let api = OpenApiBuilder::from(ApiDoc::openapi());
    let api = my_api::merge_into_openapi_builder(api);

    let json = api.build().to_json().expect("expected a valid json string");

    eprintln!("generated openapi definition as json:\n---\n{json}\n---");

    assert_eq!(
        json.parse::<serde_json::Value>().expect("expected a parsed json"),
        json!({
            "openapi": "3.0.3",
            "info": {"title": "t", "description": "d", "license": {"name": "n"}, "version": "0.0.0"},
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
                            }
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
                "/struct": {
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
                            "401": {
                                "description": "Access has been denied."
                            },
                            "500": {
                                "description": "Something bad just happened, but it's not your fault (probably)."
                            },
                        }
                    },
                },
            }
        })
    );
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
    let (status, headers, body) = get("/struct", Some("application/json")).await;
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

    // HTML has higher priority over plain text when no Accept header is specified.
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


//
// endregion: tests ------------------------------------------------
