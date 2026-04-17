use axum::Router;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::{features::response_type_json::controller::DataObject, test_utils::{Req, assert_openapi_doc}}
};

#[Controller()]
mod controller {
    use axum::response::IntoResponse;

    use groom::response::Response;
    use groom_macros::{
        DTO,
        Response
    };

    use utoipa::PartialSchema; // required to send unnamed struct or enum variant with unnamed contents as JSON

    // ---

    #[DTO(response)]
    pub struct DataObject {
        pub status: &'static str,
        pub status_timestamp: Option<u64>,
    }

    impl DataObject {
        pub fn default_str() -> &'static str {
            concat!("{\"status\":\"ok\",\"status_timestamp\":1726070400}")
        }

        pub fn unknown_time_str() -> &'static str {
            concat!("{\"status\":\"ok\",\"status_timestamp\":null}")
        }

        pub fn unknown_time() -> Self {
            Self { status: "ok", status_timestamp: None }
        }
    }

    impl Default for DataObject {
        fn default() -> Self {
            Self { 
                status: "ok",
                status_timestamp: Some(1726070400),
            }
        }
    }

    // ---

    #[Response(format(json))]
    pub enum JsonStructResponse {
        #[Response()]
        Ok(DataObject)
    }

    #[Route(method="get", path="/json_struct")]
    pub async fn json_struct() -> JsonStructResponse {
        JsonStructResponse::Ok(DataObject::default())
    }

    #[Route(method="get", path="/json_struct/no_time")]
    pub async fn json_struct_no_time() -> JsonStructResponse {
        JsonStructResponse::Ok(DataObject::unknown_time())
    }

    // ---

    #[Response(format(json))]
    pub enum JsonStringResponse {
        #[Response()]
        Ok(String) // requires utoipa::PartialSchema
    }

    #[Route(method="get", path="/json_string")]
    pub async fn json_string() -> JsonStringResponse {
        JsonStringResponse::Ok("Hello, world!".into())
    }

    // ---
}

/// Tests that json struct is returned correctly
#[tokio::test]
pub async fn json_struct() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/json_struct").call(&r).await
        .assert_status(200)
        .assert_body(DataObject::default_str())
        .assert_content_type("application/json")
    ;
}

/// Tests that json struct with None value is returned correctly
#[tokio::test]
pub async fn json_struct_no_time() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/json_struct/no_time").call(&r).await
        .assert_status(200)
        .assert_body(DataObject::unknown_time_str())
        .assert_content_type("application/json")
    ;
}

/// Tests that json string is returned correctly
#[tokio::test]
pub async fn json_string() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/json_string").call(&r).await
        .assert_status(200)
        .assert_body("\"Hello, world!\"")
        .assert_content_type("application/json")
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
            "openapi": "3.1.0",
            "paths": {
                "/json_string": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "",
                            },
                        },
                    },
                },
                "/json_struct": {
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
                                                    "type": [
                                                        "integer",
                                                        "null",
                                                    ]
                                                }
                                            },
                                            "required": [
                                                "status",
                                            ],
                                            "type": "object"
                                        },
                                    },
                                },
                                "description": "",
                            },
                        },
                    },
                },
                "/json_struct/no_time": {
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
                                                    "type": [
                                                        "integer",
                                                        "null",
                                                    ]
                                                }
                                            },
                                            "required": [
                                                "status",
                                            ],
                                            "type": "object"
                                        },
                                    },
                                },
                                "description": "",
                            },
                        },
                    },
                },
            },
            "components": {},
        })
    );
}
