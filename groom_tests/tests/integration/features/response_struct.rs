use axum::Router;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::test_utils::{Req, assert_openapi_doc}
};

#[Controller()]
mod controller {
    use axum::response::IntoResponse;

    use groom::{
        html_format,
        response::Response,
    };
    use groom_macros::{
        DTO,
        Response
    };

    use utoipa::PartialSchema; // required for String type in response

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
}

/// Default content-type for named struct
#[tokio::test]
pub async fn named_struct_default() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/named-struct").call(&r).await
        .assert_status(418)
        .assert_body("alive: true")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Plaintext content-type for named struct
#[tokio::test]
pub async fn named_struct_plaintext() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/named-struct").accept("text/plain").call(&r).await
        .assert_status(418)
        .assert_body("alive: true")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// HTML content-type for named struct
#[tokio::test]
pub async fn named_struct_html() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/named-struct").accept("text/html").call(&r).await
        .assert_status(418)
        .assert_body("<div>alive: true</div>")
        .assert_content_type("text/html; charset=utf-8")
    ;
}

/// JSON content-type for named struct
#[tokio::test]
pub async fn named_struct_json() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/named-struct").accept("application/json").call(&r).await
        .assert_status(418)
        .assert_body("{\"is_alive\":true}")
        .assert_content_type("application/json")
    ;
}

/// Default content-type for named struct with single content-type allowed
#[tokio::test]
pub async fn named_struct_single_format_default() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/named-struct-only-plaintext").call(&r).await
        .assert_status(418)
        .assert_body("v: hello, world")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Plaintext content-type for named struct with single content-type allowed
#[tokio::test]
pub async fn named_struct_single_format_plaintext() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/named-struct-only-plaintext").accept("text/plain").call(&r).await
        .assert_status(418)
        .assert_body("v: hello, world")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Unsupported content-type for named struct with single content-type allowed
#[tokio::test]
pub async fn named_struct_single_format_unsupported() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/named-struct-only-plaintext").accept("text/html").call(&r).await
        .assert_status(400)
        .assert_body("Requested Content-Type is not supported.")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/named-struct-only-plaintext").accept("application/json").call(&r).await
        .assert_status(400)
        .assert_body("Requested Content-Type is not supported.")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Default content-type for unnamed struct
#[tokio::test]
pub async fn unnamed_struct_default() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct").call(&r).await
        .assert_status(418)
        .assert_body("hello, world")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Plaintext content-type for unnamed struct
#[tokio::test]
pub async fn unnamed_struct_plaintext() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct").accept("text/plain").call(&r).await
        .assert_status(418)
        .assert_body("hello, world")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// HTML content-type for unnamed struct
#[tokio::test]
pub async fn unnamed_struct_html() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct").accept("text/html").call(&r).await
        .assert_status(418)
        .assert_body("<div>hello, world</div>")
        .assert_content_type("text/html; charset=utf-8")
    ;
}

/// JSON content-type for unnamed struct
#[tokio::test]
pub async fn unnamed_struct_json() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct").accept("application/json").call(&r).await
        .assert_status(418)
        .assert_body("\"hello, world\"")
        .assert_content_type("application/json")
    ;
}


/// Default content-type for unnamed struct with single content-type allowed
#[tokio::test]
pub async fn unnamed_struct_single_format_default() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct-only-plaintext").call(&r).await
        .assert_status(418)
        .assert_body("hello, world")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Plaintext content-type for unnamed struct with single content-type allowed
#[tokio::test]
pub async fn unnamed_struct_single_format_plaintext() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct-only-plaintext").accept("text/plain").call(&r).await
        .assert_status(418)
        .assert_body("hello, world")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Unsupported content-type for unnamed struct with single content-type allowed
#[tokio::test]
pub async fn unnamed_struct_single_format_unsupported() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct-only-plaintext").accept("text/html").call(&r).await
        .assert_status(400)
        .assert_body("Requested Content-Type is not supported.")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/unnamed-struct-only-plaintext").accept("application/json").call(&r).await
        .assert_status(400)
        .assert_body("Requested Content-Type is not supported.")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}


/// Default response for unnamed struct with DTO body
#[tokio::test]
pub async fn unnamed_struct_dto_default() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct-dto").call(&r).await
        .assert_status(418)
        .assert_body("{\"v\":\"hello, world\"}")
        .assert_content_type("application/json")
    ;
}

/// Json response for unnamed struct with DTO body
#[tokio::test]
pub async fn unnamed_struct_dto_json() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct-dto").accept("application/json").call(&r).await
        .assert_status(418)
        .assert_body("{\"v\":\"hello, world\"}")
        .assert_content_type("application/json")
    ;
}

/// Unsupported response for unnamed struct with DTO body
#[tokio::test]
pub async fn unnamed_struct_dto_unsupported() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unnamed-struct-dto").accept("text/html").call(&r).await
        .assert_status(400)
        .assert_body("Requested Content-Type is not supported.")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/unnamed-struct-dto").accept("text/plain").call(&r).await
        .assert_status(400)
        .assert_body("Requested Content-Type is not supported.")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Testing responses that are defined as a unit struct
#[tokio::test]
pub async fn test_get_unit_struct() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/unit-struct").call(&r).await
        .assert_status(418)
        .assert_no_body()
        .assert_no_content_type()
    ;

    Req::get("/unit-struct").accept("text/plain").call(&r).await
        .assert_status(418)
        .assert_no_body()
        .assert_no_content_type()
    ;

    Req::get("/unit-struct").accept("text/html").call(&r).await
        .assert_status(418)
        .assert_no_body()
        .assert_no_content_type()
    ;

    Req::get("/unit-struct").accept("application/json").call(&r).await
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
                "contact": {
                    "email": ("mail@example.com"),
                    "name": ("name"),
                },
                "description": ("d"),
                "license": {
                    "name": ("n"),
                },
                "title": ("t"),
                "version": ("0.0.0"),
            },
            "openapi": ("3.1.0"),
            "paths": {
                "/named-struct": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "description": ("Named struct as a response"),
                                            "properties": {
                                                "is_alive": {
                                                    "type": ("boolean"),
                                                },
                                            },
                                            "required": [
                                                ("is_alive"),
                                            ],
                                            "type": ("object"),
                                        },
                                    },
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
                                "description": ("Named struct as a response"),
                            },
                        },
                    },
                },
                "/named-struct-only-plaintext": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": ("string"),
                                        },
                                    },
                                },
                                "description": ("Named struct as a plaintext-only response"),
                            },
                        },
                    },
                },
                "/unit-struct": {
                    "get": {
                        "responses": {
                            "418": {
                                "description": ("Unit struct"),
                            },
                        },
                    },
                },
                "/unnamed-struct": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
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
                                "description": ("Unnamed struct as a response"),
                            },
                        },
                    },
                },
                "/unnamed-struct-dto": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "description": ("Unnamed struct DTO"),
                                            "properties": {
                                                "v": {
                                                    "type": "string",
                                                },
                                            },
                                            "required": [
                                                "v",
                                            ],
                                            "type": ("object"),
                                        },
                                    },
                                },
                                "description": ("Unnamed struct DTO result"),
                            },
                        },
                    },
                },
                "/unnamed-struct-only-plaintext": {
                    "get": {
                        "responses": {
                            "418": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": ("string"),
                                        },
                                    },
                                },
                                "description": ("Unnamed struct as a plaintext-only response"),
                            },
                        },
                    },
                },
            },
            "components": {},
        })
    );
}
