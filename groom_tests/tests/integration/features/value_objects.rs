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

    #[DTO(response)]
    pub enum EnumValueObject {
        UnitVariant,
        UnnamedStructVariant(String),
        NamedStructVariant {
            value: String
        }
    }

    #[DTO(response)]
    pub struct WrapperStruct {
        pub v: EnumValueObject
    }

    // todo: other formats
    #[Response(format(json))]
    pub enum Resp {
        #[Response(code=200)]
        Enum(EnumValueObject),

        #[Response(code=202)]
        StructWithEnum(WrapperStruct),
    }

    // todo: enums as request values
    
    // todo: newtypes as response values
    // todo: newtypes as request values


    #[Route(method = "get", path = "/enum/unit")]
    async fn resp_enum_unit() -> Resp {
        Resp::Enum(EnumValueObject::UnitVariant)
    }

    #[Route(method = "get", path = "/enum/unnamed-struct")]
    async fn resp_enum_unnamed_struct() -> Resp {
        Resp::Enum(EnumValueObject::UnnamedStructVariant("foo".into()))
    }

    #[Route(method = "get", path = "/enum/named-struct")]
    async fn resp_enum_named_struct() -> Resp {
        Resp::Enum(EnumValueObject::NamedStructVariant{value: "foo".into()})
    }

    #[Route(method = "get", path = "/wrapped/unit")]
    async fn resp_wrapped_unit() -> Resp {
        Resp::StructWithEnum(WrapperStruct{ v: EnumValueObject::UnitVariant })
    }

    #[Route(method = "get", path = "/wrapped/unnamed-struct")]
    async fn resp_wrapped_unnamed_struct() -> Resp {
        Resp::StructWithEnum(WrapperStruct{ v: EnumValueObject::UnnamedStructVariant("foo".into()) })
    }

    #[Route(method = "get", path = "/wrapped/named-struct")]
    async fn resp_wrapped_named_struct() -> Resp {
        Resp::StructWithEnum(WrapperStruct{ v: EnumValueObject::NamedStructVariant{value: "foo".into()} })
    }
}

#[tokio::test]
pub async fn test_enum_responses() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/enum/unit").call(&r).await
        .assert_status(200)
        .assert_body(r#""UnitVariant""#)
        .assert_content_type("application/json")
    ;

    Req::get("/enum/unnamed-struct").call(&r).await
        .assert_status(200)
        .assert_body(r#"{"UnnamedStructVariant":"foo"}"#)
        .assert_content_type("application/json")
    ;

    Req::get("/enum/named-struct").call(&r).await
        .assert_status(200)
        .assert_body(r#"{"NamedStructVariant":{"value":"foo"}}"#)
        .assert_content_type("application/json")
    ;
}

#[tokio::test]
pub async fn test_wrapped_responses() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/wrapped/unit").call(&r).await
        .assert_status(202)
        .assert_body(r#"{"v":"UnitVariant"}"#)
        .assert_content_type("application/json")
    ;

    Req::get("/wrapped/unnamed-struct").call(&r).await
        .assert_status(202)
        .assert_body(r#"{"v":{"UnnamedStructVariant":"foo"}}"#)
        .assert_content_type("application/json")
    ;

    Req::get("/wrapped/named-struct").call(&r).await
        .assert_status(202)
        .assert_body(r#"{"v":{"NamedStructVariant":{"value":"foo"}}}"#)
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
