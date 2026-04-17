use axum::Router;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::{
        features::response_content_negotiation::controller::DataObject,
        test_utils::{Req, assert_openapi_doc}
    }
};

#[Controller()]
mod controller {
    use axum::response::IntoResponse;

    use groom::{
        html_format,
        response::Response,
        schema::GroomSchema,
    };
    use groom_macros::{DTO,Response};

    use utoipa::ToSchema;

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
    let r = controller::merge_into_router(Router::new());

    Req::get("/status").call(&r).await
        .assert_status(200)
        .assert_body(DataObject::default_json_str())
        .assert_content_type("application/json")
    ;
}

/// Tests that handler picks default html format by default
#[tokio::test]
pub async fn status_default_html() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/status/html").call(&r).await
        .assert_status(200)
        .assert_body(DataObject::default_html_str())
        .assert_content_type("text/html; charset=utf-8")
    ;
}

/// Tests that handler picks json format from headers
#[tokio::test]
pub async fn status_json() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/status").accept("application/json").call(&r).await
        .assert_status(200)
        .assert_body(DataObject::default_json_str())
        .assert_content_type("application/json")
    ;
}

/// Tests that handler picks html format from headers
#[tokio::test]
pub async fn status_html() {
    let r = controller::merge_into_router(Router::new());

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
    let r = controller::merge_into_router(Router::new());

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
#[tokio::test]
pub async fn test_openapi() {
    assert_openapi_doc(
        |b| controller::merge_into_openapi_builder(b),
        json!({
            "info": {
                "contact": {"email": "mail@example.com","name": "name"},
                "description": "d",
                "license": {"name": "n"},
                "title": "t",
                "version": "0.0.0",
            },
            "openapi": "3.0.3",
            "paths": {
                "/no-content": {
                    "put": {
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
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "properties": {
                                                "status": {
                                                    "type": "string",
                                                },
                                                "status_timestamp": {
                                                    "format": "int64",
                                                    "minimum": 0,
                                                    "type": "integer"
                                                }
                                            },
                                            "required": [
                                                "status",
                                                "status_timestamp",
                                            ],
                                            "type": "object"
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
                        },
                    },
                },
                "/status/html": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "properties": {
                                                "status": {
                                                    "type": "string",
                                                },
                                                "status_timestamp": {
                                                    "format": "int64",
                                                    "minimum": 0,
                                                    "type": "integer"
                                                }
                                            },
                                            "required": [
                                                "status",
                                                "status_timestamp",
                                            ],
                                            "type": "object"
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

/// In this test we check how content-negotiation chooses appropriate serialization of a struct
/// between String and Html based on weights.
#[tokio::test]
pub async fn test_html_or_text_weights() {
    let r = weights_controller::merge_into_router(Router::new());

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
        |b| weights_controller::merge_into_openapi_builder(b),
        json!( {
            "info": {
                "contact": {"email": "mail@example.com","name": "name",},
                "description": "d",
                "license": {"name": "n",},
                "title": "t",
                "version": "0.0.0",
            },
            "openapi": "3.0.3",
            "paths": {
                "/html-or-text": {
                    "get": {
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
                        },
                    },
                },
            },
        })
    );
}

