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

    #[DTO(response)]
    pub enum EnumValueObject {
        UnitVariant,
        UnnamedStructVariant(String),
        NamedStructVariant {
            value: String
        }
    }

    pub mod submod {
        use groom_macros::DTO;

        #[DTO(response)]
        pub enum EnumValueObject {
            UnitVariant,
            UnnamedStructVariant(String),
            NamedStructVariant {
                value: String
            }
        }
    }

    #[DTO(response)]
    pub struct WrapperStruct {
        pub v: EnumValueObject,
    }


    #[DTO(response)]
    pub struct WrapperStructWithConflict {
        pub v: EnumValueObject,
        pub v2: submod::EnumValueObject,
    }

    // todo: other formats
    #[Response(format(json))]
    pub enum Resp {
        #[Response(code=200)]
        Enum(EnumValueObject),

        #[Response(code=202)]
        StructWithEnum(WrapperStruct),

        #[Response(code=203)]
        StructWithEnumWithConflict(WrapperStructWithConflict),
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

    #[Route(method = "get", path = "/wrapped/conflict/named-struct")]
    async fn resp_wrapped_named_struct_with_conflict() -> Resp {
        Resp::StructWithEnumWithConflict(WrapperStructWithConflict{
            v: EnumValueObject::NamedStructVariant{ value: "foo".into() },
            v2: submod::EnumValueObject::NamedStructVariant { value: "bar".into() }
        })
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
            "components": {
                "schemas": {
                    "EnumValueObject": {
                        "oneOf": [
                            {
                                "enum": [
                                    "UnitVariant",
                                ],
                                "type": "string",
                            },
                            {
                                "properties": {
                                    "UnnamedStructVariant": {
                                        "type": "string",
                                    },
                                },
                                "required": [
                                    "UnnamedStructVariant",
                                ],
                                "type": "object",
                            },
                            {
                                "properties": {
                                    "NamedStructVariant": {
                                        "properties": {
                                            "value": {
                                                "type": ("string"),
                                            },
                                        },
                                        "required": [
                                            ("value"),
                                        ],
                                        "type": ("object"),
                                    },
                                },
                                "required": [
                                    ("NamedStructVariant"),
                                ],
                                "type": ("object"),
                            },
                        ],
                    },
                    "WrapperStruct": {
                        "properties": {
                            "v": {
                                "$ref": ("#/components/schemas/EnumValueObject"),
                            },
                        },
                        "required": [
                            ("v"),
                        ],
                        "type": ("object"),
                    },
                    "WrapperStructWithConflict": {
                        "properties": {
                            "v": {
                                "$ref": ("#/components/schemas/EnumValueObject"),
                            },
                            "v2": {
                                "$ref": ("#/components/schemas/EnumValueObject"),
                            },
                        },
                        "required": [
                            ("v"),
                            ("v2"),
                        ],
                        "type": ("object"),
                    },
                },
            },
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
                "/enum/named-struct": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/EnumValueObject")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "202": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStruct")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "203": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStructWithConflict")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                        },
                    },
                },
                "/enum/unit": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/EnumValueObject")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "202": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStruct")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "203": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStructWithConflict")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                        },
                    },
                },
                "/enum/unnamed-struct": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/EnumValueObject")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "202": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStruct")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "203": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStructWithConflict")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                        },
                    },
                },
                "/wrapped/conflict/named-struct": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/EnumValueObject"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "202": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStruct"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "203": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStructWithConflict")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                        },
                    },
                },
                "/wrapped/named-struct": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/EnumValueObject")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "202": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStruct")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "203": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStructWithConflict")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                        },
                    },
                },
                "/wrapped/unit": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/EnumValueObject")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "202": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStruct")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "203": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStructWithConflict")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                        },
                    },
                },
                "/wrapped/unnamed-struct": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/EnumValueObject"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "202": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStruct")
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "203": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/WrapperStructWithConflict"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                        },
                    },
                },
            },
        })
    );
}
