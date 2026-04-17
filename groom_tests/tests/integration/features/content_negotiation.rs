use axum::Router;
use groom::response::HtmlFormat;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::{features::content_negotiation::controller::DataObject, test_utils::{Req, assert_openapi_doc}}
};

#[Controller()]
mod controller {
    use axum::response::IntoResponse;
    use groom::{html_format, response::Response, schema::GroomSchema};
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
