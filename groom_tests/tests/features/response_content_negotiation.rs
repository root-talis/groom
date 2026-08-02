use serde_json::json;

use crate::{
    groom_macros::Controller,
    features::{
        response_content_negotiation::controller::DataObject,
        test_utils::{Req, assert_openapi_doc}
    }
};

/// Malformed Accept header fixture that MUST fail `Accept::parse` (sanity-guarded
/// inside the tests that use it).
const MALFORMED_ACCEPT: &str = "text/plain;q=";

#[Controller()]
mod controller {
    use axum::response::IntoResponse;

    use groom::{
        html_format,
        response::Response,
    };
    use groom_macros::{DTO,Response};

    use utoipa::PartialSchema;

    // ---

    #[DTO(response)]
    pub struct DataObject {
        pub status: &'static str,
        pub status_timestamp: u64,
    }

    impl DataObject {
        pub fn default_json_str() -> &'static str {
            concat!("{\"status\":\"ok\",\"status_timestamp\":1726070400}")
        }

        pub fn default_html_str() -> &'static str {
            concat!("status: <b>ok</b> (since 1726070400)")
        }
    }

    impl Default for DataObject {
        fn default() -> Self {
            Self { 
                status: "ok",
                status_timestamp: 1726070400,
            }
        }
    }
    
    html_format!(DataObject, self {
        // important: in production make sure to escape special chars!
        format!(
            "status: <b>{}</b> (since {})",
            self.status,
            self.status_timestamp
        )
    });

    // ---

    #[Response(format(html, json), default_format="json")]
    pub enum HtmlOrJsonResponse {
        #[Response()]
        Ok(DataObject)
    }

    #[Route(method="get", path="/status")]
    pub async fn status_default_json() -> HtmlOrJsonResponse {
        HtmlOrJsonResponse::Ok(DataObject::default())
    }

    // ---

    #[Response(format(html, json), default_format="html")]
    pub enum JsonOrHtmlResponse {
        #[Response()]
        Ok(DataObject)
    }
    
    #[Route(method="get", path="/status/html")]
    pub async fn status_default_html() -> JsonOrHtmlResponse {
        JsonOrHtmlResponse::Ok(DataObject::default())
    }

    // ---

    #[Response()]
    pub enum NoContentResponse {
        #[allow(dead_code)]
        #[Response(code = 202)]
        Accepted,

        #[Response(code = 418)]
        Teapot,
    }

    #[Route(method="put", path="/no-content")]
    pub async fn root() -> NoContentResponse {
        NoContentResponse::Teapot
    }
}

/// Tests that handler picks default json format by default
#[tokio::test]
pub async fn status_default_json() {
    let r = controller::into_router().validate().unwrap().to_axum_router();

    Req::get("/status").call(&r).await
        .assert_status(200)
        .assert_body(DataObject::default_json_str())
        .assert_content_type("application/json")
    ;
}

/// Tests that handler picks default html format by default
#[tokio::test]
pub async fn status_default_html() {
    let r = controller::into_router().validate().unwrap().to_axum_router();

    Req::get("/status/html").call(&r).await
        .assert_status(200)
        .assert_body(DataObject::default_html_str())
        .assert_content_type("text/html; charset=utf-8")
    ;
}

/// Tests that handler picks json format from headers
#[tokio::test]
pub async fn status_json() {
    let r = controller::into_router().validate().unwrap().to_axum_router();

    Req::get("/status").accept("application/json").call(&r).await
        .assert_status(200)
        .assert_body(DataObject::default_json_str())
        .assert_content_type("application/json")
    ;
}

/// Tests that handler picks html format from headers
#[tokio::test]
pub async fn status_html() {
    let r = controller::into_router().validate().unwrap().to_axum_router();

    Req::get("/status").accept("text/html").call(&r).await
        .assert_status(200)
        .assert_body(DataObject::default_html_str())
        .assert_content_type("text/html; charset=utf-8")
    ;
}

/// Tests that content negotiation ignores any Accept header's value
/// because no Response variant has a body anyway
#[tokio::test]
pub async fn no_body_accept_antrhing() {
    let r = controller::into_router().validate().unwrap().to_axum_router();

    Req::put("/no-content").call(&r).await
        .assert_status(418)
        .assert_no_body()
        .assert_no_content_type()
    ;

    Req::put("/no-content").accept("text/plain").call(&r).await
        .assert_status(418)
        .assert_no_body()
        .assert_no_content_type()
    ;

    Req::put("/no-content").accept("something/stupid").call(&r).await
        .assert_status(418)
        .assert_no_body()
        .assert_no_content_type()
    ;
}

/// Tests that openapi definition is correctly generated
#[test]
pub fn test_openapi() {
    assert_openapi_doc(
        |api| controller::into_router().validate().unwrap().to_openapi(api),
        json!({
            "info": {
                "contact": {"email": "mail@example.com","name": "name"},
                "description": "d",
                "license": {"name": "n"},
                "title": "t",
                "version": "0.0.0",
            },
            "openapi": "3.1.0",
            "components": {
                "schemas": {
                    "DataObject": {
                        "properties": {
                            "status": {
                                "type":("string"),
                            },
                            "status_timestamp": {
                                "format":("int64"),
                                "minimum": (0),
                                "type": ("integer"),
                            },
                        },
                        "required":  [
                            ("status"),
                            ("status_timestamp"),
                        ],
                        "type": ("object"),
                    },
                },
            },
            "paths": {
                "/no-content": {
                    "put": {
                        "operationId": ("root"),
                        "responses": {
                            "202": {
                                "description": "",
                            },
                            "418": {
                                "description": "",
                            },
                        },
                    },
                },
                "/status": {
                    "get": {
                        "operationId": ("statusDefaultJson"),
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/DataObject"),
                                        },
                                    },
                                    "text/html; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    },
                                },
                                "description": "",
                            },
                            "406": {
                                "description": ("The requested content type is not supported"),
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": ("string"),
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
                "/status/html": {
                    "get": {
                        "operationId": ("statusDefaultHtml"),
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/DataObject"),
                                        },
                                    },
                                    "text/html; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    },
                                },
                                "description": "",
                            },
                            "406": {
                                "description": ("The requested content type is not supported"),
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": ("string"),
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        })
    );
}

#[Controller()]
mod weights_controller {
    use axum::response::IntoResponse;

    use groom::{
        html_format,
        response::Response,
    };
    use groom_macros::{DTO,Response};

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

    #[Route(method = "get", path = "/html-or-text")]
    async fn resp_html_or_text() -> GetHtmlOrTextBodyResult {
        GetHtmlOrTextBodyResult::Ok(PageData("Hello, world!".to_string()))
    }
}

#[Controller()]
mod panic_controller {
    use axum::response::IntoResponse;

    use groom::response::Response;
    use groom_macros::Response;

    use super::controller::{DataObject, NoContentResponse};

    // ---

    #[Response(format(json))]
    pub enum PanicJsonResponse {
        #[allow(dead_code)]
        #[Response()]
        Ok(DataObject),
    }

    #[Response()]
    pub enum PanicNoContent {
        #[allow(dead_code)]
        #[Response()]
        Ok,
    }

    #[Route(method = "get", path = "/panic-negotiating")]
    pub async fn resp_panic_negotiating() -> PanicJsonResponse {
        panic!("handler must not be called")
    }

    #[Route(method = "get", path = "/panic-any-content")]
    pub async fn resp_panic_any_content() -> PanicNoContent {
        panic!("handler must not be called")
    }

    #[Route(method = "put", path = "/any-content-ok")]
    pub async fn resp_any_content_ok() -> NoContentResponse {
        NoContentResponse::Teapot
    }
}

/// In this test we check how content-negotiation chooses appropriate serialization of a struct
/// between String and Html based on weights.
#[tokio::test]
pub async fn test_html_or_text_weights() {
    let r = weights_controller::into_router().validate().unwrap().to_axum_router();

    // First content-type has priority
    Req::get("/html-or-text").accept("text/plain, text/html").call(&r).await
        .assert_status(200)
        .assert_body("Hello, world!")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    // First content-type has priority
    Req::get("/html-or-text").accept("text/html, text/plain").call(&r).await
        .assert_status(200)
        .assert_body("<h1>Hello, world!</h1>")
        .assert_content_type("text/html; charset=utf-8")
    ;

    // Weights have higher priority over position.
    Req::get("/html-or-text").accept("text/plain;q=0.8, text/html;q=0.9").call(&r).await
        .assert_status(200)
        .assert_body("<h1>Hello, world!</h1>")
        .assert_content_type("text/html; charset=utf-8")
    ;

    // Weights are not being prioritized by position in reverse order.
    Req::get("/html-or-text").accept("text/plain;q=0.9, text/html;q=0.8").call(&r).await
        .assert_status(200)
        .assert_body("Hello, world!")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    // Inappropriate content-type is ignored.
    Req::get("/html-or-text").accept("text/html, application/xhtml+xml, application/xml;q=0.9, */*;q=0.8").call(&r).await
        .assert_status(200)
        .assert_body("<h1>Hello, world!</h1>")
        .assert_content_type("text/html; charset=utf-8")
    ;

    // Inappropriate content-type is ignored even when placed first.
    Req::get("/html-or-text").accept("application/xhtml+xml, text/html, application/xml;q=0.9, */*;q=0.8").call(&r).await
        .assert_status(200)
        .assert_body("<h1>Hello, world!</h1>")
        .assert_content_type("text/html; charset=utf-8")
    ;

    // HTML has higher priority over plain text when content-type */* is specified.
    Req::get("/html-or-text").accept("*/*").call(&r).await
        .assert_status(200)
        .assert_body("<h1>Hello, world!</h1>")
        .assert_content_type("text/html; charset=utf-8")
    ;

    // HTML has higher priority over plain text when no Accept header is
    Req::get("/html-or-text").call(&r).await
        .assert_status(200)
        .assert_body("<h1>Hello, world!</h1>")
        .assert_content_type("text/html; charset=utf-8")
    ;
}

#[tokio::test]
pub async fn test_html_or_text_weights_openapi() {
    assert_openapi_doc(
        |api| weights_controller::into_router().validate().unwrap().to_openapi(api),
        json!( {
            "info": {
                "contact": {"email": "mail@example.com","name": "name",},
                "description": "d",
                "license": {"name": "n",},
                "title": "t",
                "version": "0.0.0",
            },
            "openapi": "3.1.0",
            "paths": {
                "/html-or-text": {
                    "get": {
                        "operationId": ("respHtmlOrText"),
                        "responses": {
                            "200": {
                                "content": {
                                    "text/html; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Home page",
                            },
                            "406": {
                                "description": ("The requested content type is not supported"),
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": ("string"),
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
            "components": {},
        })
    );
}

/// A panic-on-call handler returning a negotiating type must receive 406 (not run)
/// for an Accept that matches none of its formats (SPEC req 1 acceptance).
#[tokio::test]
pub async fn test_panic_handler_not_called_on_unsatisfiable_accept() {
    let r = panic_controller::into_router().validate().unwrap().to_axum_router();

    let res = Req::get("/panic-negotiating").accept("application/xml").call(&r).await;
    res.assert_status(406)
        .assert_body("Supported content types: application/json")
        .assert_content_type("text/plain; charset=utf-8");
    assert_eq!(res.headers.get("vary"), Some(&axum::http::HeaderValue::from_static("Accept")));
}

/// A malformed Accept must yield 400 without running a panic-on-call negotiating
/// handler (SPEC req 3 acceptance).
#[tokio::test]
pub async fn test_panic_handler_not_called_on_malformed_accept_negotiating() {
    // sanity guard: the malformed fixture must fail Accept::parse
    assert!(MALFORMED_ACCEPT.parse::<accept_header::Accept>().is_err());

    let r = panic_controller::into_router().validate().unwrap().to_axum_router();

    let res = Req::get("/panic-negotiating").accept(MALFORMED_ACCEPT).call(&r).await;
    res.assert_status(400)
        .assert_body("Invalid Accept header.");
}

/// A malformed Accept must yield 400 without running a panic-on-call any-content
/// handler too — any-content types skip negotiation but still validate the header
/// (SPEC req 3 acceptance).
#[tokio::test]
pub async fn test_panic_handler_not_called_on_malformed_accept_any_content() {
    // sanity guard: the malformed fixture must fail Accept::parse
    assert!(MALFORMED_ACCEPT.parse::<accept_header::Accept>().is_err());

    let r = panic_controller::into_router().validate().unwrap().to_axum_router();

    let res = Req::get("/panic-any-content").accept(MALFORMED_ACCEPT).call(&r).await;
    res.assert_status(400)
        .assert_body("Invalid Accept header.");
}

/// An any-content handler still runs on a valid Accept (preserved behavior, req 7).
#[tokio::test]
pub async fn test_any_content_handler_still_runs_on_valid_accept() {
    let r = panic_controller::into_router().validate().unwrap().to_axum_router();

    Req::put("/any-content-ok").accept("text/plain").call(&r).await
        .assert_status(418)
        .assert_no_body()
        .assert_no_content_type()
    ;
}

/// The 406 body lists every supported mime of the type (deterministic const-array
/// order: plain_text, then html) and carries the Vary: Accept header (SPEC req 2
/// acceptance).
#[tokio::test]
pub async fn test_406_vary_and_body_lists_all_supported_mimes() {
    let r = weights_controller::into_router().validate().unwrap().to_axum_router();

    let res = Req::get("/html-or-text").accept("application/xml").call(&r).await;
    res.assert_status(406)
        .assert_body("Supported content types: text/plain; charset=utf-8, text/html; charset=utf-8")
        .assert_content_type("text/plain; charset=utf-8");
    assert_eq!(res.headers.get("vary"), Some(&axum::http::HeaderValue::from_static("Accept")));
    assert!(res.body.contains("text/plain") && res.body.contains("text/html"));
}

/// Charset-bearing and bare Accept values both negotiate successfully against
/// groom's UTF-8 wire formats (P008 / D-21).
#[tokio::test]
pub async fn test_charset_bearing_and_bare_accept_values() {
    let r = weights_controller::into_router().validate().unwrap().to_axum_router();

    Req::get("/html-or-text").accept("text/plain; charset=utf-8").call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8");
    Req::get("/html-or-text").accept("text/plain").call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8");

    Req::get("/html-or-text").accept("text/html; charset=utf-8").call(&r).await
        .assert_status(200)
        .assert_content_type("text/html; charset=utf-8");
    Req::get("/html-or-text").accept("text/html").call(&r).await
        .assert_status(200)
        .assert_content_type("text/html; charset=utf-8");

    let r = controller::into_router().validate().unwrap().to_axum_router();
    Req::get("/status").accept("application/json").call(&r).await
        .assert_status(200)
        .assert_content_type("application/json");

    let r = weights_controller::into_router().validate().unwrap().to_axum_router();
    Req::get("/html-or-text").accept("application/xml").call(&r).await
        .assert_status(406)
        .assert_content_type("text/plain; charset=utf-8");
}

/// D-22 identity gate: the negotiation supported-mime list (surfaced in the 406
/// body) must be identical to the OpenAPI 200 content keys per format.
#[tokio::test]
pub async fn test_d22_supported_mimes_match_openapi_content_keys() {
    use std::collections::BTreeSet;
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "t",
            description = "d",
            license(name = "n"),
            version = "0.0.0",
            contact(name = "name", email = "mail@example.com")
        )
    )]
    struct ApiDoc;

    fn content_keys(api: &utoipa::openapi::OpenApi, path: &str) -> BTreeSet<String> {
        let json: serde_json::Value =
            serde_json::from_str(&api.to_json().expect("valid openapi json")).unwrap();
        json["paths"][path]["get"]["responses"]["200"]["content"]
            .as_object()
            .expect("200 content map")
            .keys()
            .cloned()
            .collect()
    }

    fn supported_from_406_body(body: &str) -> BTreeSet<String> {
        let prefix = "Supported content types: ";
        assert!(body.starts_with(prefix), "unexpected 406 body: {body}");
        body[prefix.len()..]
            .split(", ")
            .map(str::to_string)
            .collect()
    }

    // Multi-format (plain + html): OpenAPI 200 keys == 406 supported list.
    let api = weights_controller::into_router()
        .validate()
        .unwrap()
        .to_openapi(ApiDoc::openapi());
    let openapi_keys = content_keys(&api, "/html-or-text");
    let expected_html_or_text = BTreeSet::from([
        "text/plain; charset=utf-8".to_string(),
        "text/html; charset=utf-8".to_string(),
    ]);
    assert_eq!(openapi_keys, expected_html_or_text);

    let r = weights_controller::into_router().validate().unwrap().to_axum_router();
    let res = Req::get("/html-or-text").accept("application/xml").call(&r).await;
    res.assert_status(406);
    assert_eq!(supported_from_406_body(&res.body), openapi_keys);

    // JSON-only: OpenAPI 200 keys == 406 supported list.
    let api = panic_controller::into_router()
        .validate()
        .unwrap()
        .to_openapi(ApiDoc::openapi());
    let openapi_keys = content_keys(&api, "/panic-negotiating");
    let expected_json = BTreeSet::from(["application/json".to_string()]);
    assert_eq!(openapi_keys, expected_json);

    let r = panic_controller::into_router().validate().unwrap().to_axum_router();
    let res = Req::get("/panic-negotiating").accept("application/xml").call(&r).await;
    res.assert_status(406);
    assert_eq!(supported_from_406_body(&res.body), openapi_keys);
}

