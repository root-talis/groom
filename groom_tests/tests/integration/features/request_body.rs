use axum::{Router};
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::test_utils::{Req, ReqBody, assert_openapi_doc}
};

/// A controller
#[Controller()]
mod controller {
    use axum::response::IntoResponse;
    
    use groom::{
        response::Response,
        extract::GroomExtractor
    };
    use groom_macros::{DTO,Response,RequestBody};

    use utoipa::ToSchema;

    // ---

    /// Simple string response
    #[Response(format(plain_text))]
    pub enum StringResponse {
        /// Everything is ok
        #[Response()]
        Ok(String),

        #[allow(dead_code)]
        /// Something in the request is not ok
        #[Response(code = 400)]
        BadRequest(String),
    }

    // ---

    /// Request consumption: String body
    #[Route(method = "post", path = "/string_body")]
    async fn rq_cons_string_body(body: String) -> StringResponse {
        StringResponse::Ok(format!("body: {body}"))
    }

    // ---

    /// Request consumption: Bytes body
    #[Route(method = "post", path = "/bytes_body")]
    async fn rq_cons_bytes_body(body: axum::body::Bytes) -> StringResponse {
        StringResponse::Ok(format!("bytes count: {}", body.iter().count()))
    }

    // ---

    groom::binary_request_body!(ImageJpeg with content_type "image/jpeg");

    /// Request consumption: ImageJpeg body
    #[Route(method = "post", path = "/image_body")]
    async fn rq_cons_image_body(body: ImageJpeg) -> StringResponse {
        StringResponse::Ok(format!("bytes count: {}", body.0.iter().count()))
    }

    // ---

    /// Request body as a named struct.
    #[RequestBody(format(json, url_encoded))]
    pub struct MultiFormatRequestBody {
        /// Person's name
        name: String,

        /// Person's age
        age: Option<u8>,
    }

    /// Accepts data in JSON or URL-encoded
    #[Route(method = "post", path = "/multi_format")]
    async fn rq_cons_multi_format_body(body: MultiFormatRequestBody) -> StringResponse {
        StringResponse::Ok(format!("someone named {} is {} years old", body.name, body.age.map_or(
            "who knows how many".into(),
            |v| format!("{v}")
        )))
    }

    // ---

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
    async fn rq_cons_multi_format_body_dto(MultiFormatRequestBodyDto(body): MultiFormatRequestBodyDto) -> StringResponse {
        StringResponse::Ok(format!("someone named {} is {} years old", body.name, body.age.map_or(
            "who knows how many".into(),
            |v| format!("{v}")
        )))
    }

    // todo: Multipart (for multipart/form-data)
}

#[tokio::test]
pub async fn test_post_string() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/string_body")
        .accept("text/plain")
        .with_body(ReqBody::new("hello, world!"))
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("body: hello, world!")
    ;

    Req::post("/string_body")
        .accept("text/plain")
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("body: ")
    ;
}

#[tokio::test]
pub async fn test_post_bytes() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/bytes_body")
        .accept("text/plain")
        .with_body(ReqBody::new("hello, world!"))
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("bytes count: 13")
    ;

    Req::post("/bytes_body").accept("text/plain").call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("bytes count: 0")
    ;
}


#[tokio::test]
pub async fn test_post_image() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/image_body")
        .accept("text/plain")
        .with_body(ReqBody::new("hello, world!"))
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("bytes count: 13")
    ;

    Req::post("/image_body").accept("text/plain").call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("bytes count: 0")
    ;
}


// Request body JSON - named struct
#[tokio::test]
pub async fn test_post_multi_format_json_named_struct() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/multi_format")
        .with_body(
            ReqBody::new("{\"name\": \"Mark\"}")
                .with_content_type("application/json")
        )
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("someone named Mark is who knows how many years old")
    ;

    Req::post("/multi_format")
        .with_body(
            ReqBody::new("{\"name\": \"Mark\", \"age\": 20}")
                .with_content_type("application/json")
        )
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("someone named Mark is 20 years old")
    ;
}

// Request body JSON - DTO wrapper
#[tokio::test]
pub async fn test_post_multi_format_json_unnamed_struct() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/multi_format_dto")
        .with_body(
            ReqBody::new("{\"name\": \"Mark\"}")
                .with_content_type("application/json")
        )
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("someone named Mark is who knows how many years old")
    ;

    Req::post("/multi_format_dto")
        .with_body(
            ReqBody::new("{\"name\": \"Mark\", \"age\": 20}")
                .with_content_type("application/json")
        )
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("someone named Mark is 20 years old")
    ;
}

// Request body url-encoded - named struct
#[tokio::test]
pub async fn test_post_multi_format_url_encoded_named_struct() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/multi_format")
        .with_body(
            ReqBody::new("name=Mark")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("someone named Mark is who knows how many years old")
    ;

    Req::post("/multi_format")
        .with_body(
            ReqBody::new("name=Mark&age=20")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("someone named Mark is 20 years old")
    ;
}

// Request body url-encoded - DTO wrapper
#[tokio::test]
pub async fn test_post_multi_format_url_encoded_unnamed_struct() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/multi_format_dto")
        .with_body(
            ReqBody::new("name=Mark")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("someone named Mark is who knows how many years old")
    ;

    Req::post("/multi_format_dto")
        .with_body(
            ReqBody::new("name=Mark&age=20")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("someone named Mark is 20 years old")
    ;
}


/// Tests that openapi definition is correctly generated
#[tokio::test]
pub async fn test_openapi() {
    assert_openapi_doc(
        |b| controller::merge_into_openapi_builder(b),
        json!({
            "info": {
                "contact": {"email": "mail@example.com","name": "name",
                },
                "description": "d",
                "license": {"name": "n"},
                "title": "t",
                "version": "0.0.0",
            },
            "openapi": ("3.0.3"),
            "paths": {
                "/bytes_body": {
                    "post": {
                        "requestBody": {
                            "content": {
                                "application/octet-stream": {
                                    "schema": {
                                        "format": "binary",
                                        "type": "string",
                                    },
                                },
                            },
                            "required": true,
                        },
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Everything is ok",
                            },
                            "400": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Something in the request is not ok",
                            },
                        },
                        "summary": "Request consumption: Bytes body",
                    },
                },
                "/image_body": {
                    "post": {
                        "requestBody": {
                            "content": {
                                "image/jpeg": {
                                    "schema": {
                                        "format": "binary",
                                        "type": "string",
                                    },
                                },
                            },
                            "required": true,
                        },
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Everything is ok",
                            },
                            "400": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Something in the request is not ok",
                            },
                        },
                        "summary": "Request consumption: ImageJpeg body",
                    },
                },
                "/multi_format": {
                    "post": {
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "description": ("Request body as a named struct."),
                                        "properties": {
                                            "age": {
                                                "description": "Person's age",
                                                "format": "int32",
                                                "minimum": 0,
                                                "nullable": true,
                                                "type": "integer",
                                            },
                                            "name": {
                                                "description": "Person's name",
                                                "type": "string",
                                            },
                                        },
                                        "required": [
                                            "name",
                                        ],
                                        "type": "object",
                                    },
                                },
                                "application/x-www-form-urlencoded": {
                                    "schema": {
                                        "description": ("Request body as a named struct."),
                                        "properties": {
                                            "age": {
                                                "description": "Person's age",
                                                "format": "int32",
                                                "minimum": 0,
                                                "nullable": true,
                                                "type": "integer",
                                            },
                                            "name": {
                                                "description": "Person's name",
                                                "type": "string",
                                            },
                                        },
                                        "required": [
                                            "name",
                                        ],
                                        "type": "object",
                                    },
                                },
                            },
                            "description": "Request body as a named struct.",
                            "required": true,
                        },
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Everything is ok",
                            },
                            "400": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Something in the request is not ok",
                            },
                        },
                        "summary": "Accepts data in JSON or URL-encoded",
                    },
                },
                "/multi_format_dto": {
                    "post": {
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "description": "Some DTO",
                                        "properties": {
                                            "age": {
                                                "format": "int32",
                                                "minimum": 0,
                                                "nullable": true,
                                                "type": "integer",
                                            },
                                            "name": {
                                                "type": "string",
                                            },
                                        },
                                        "required": [
                                            "name",
                                        ],
                                        "type": "object",
                                    },
                                },
                                "application/x-www-form-urlencoded": {
                                    "schema": {
                                        "description": "Some DTO",
                                        "properties": {
                                            "age": {
                                                "format": "int32",
                                                "minimum": 0,
                                                "nullable": true,
                                                "type": "integer",
                                            },
                                            "name": {
                                                "type": "string",
                                            },
                                        },
                                        "required": [
                                            "name",
                                        ],
                                        "type": "object",
                                    },
                                },
                            },
                            "description": "Request body as an unnamed struct that wraps around a DTO",
                            "required": true,
                        },
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Everything is ok",
                            },
                            "400": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Something in the request is not ok",
                            },
                        },
                    },
                },
                "/string_body": {
                    "post": {
                        "requestBody": {
                            "content": {
                                "text/plain; charset=utf-8": {
                                    "schema": {
                                        "type": "string",
                                    },
                                },
                            },
                            "required": true,
                        },
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Everything is ok",
                            },
                            "400": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string",
                                        },
                                    },
                                },
                                "description": "Something in the request is not ok",
                            },
                        },
                        "summary": "Request consumption: String body",
                    },
                },
            },
        })
    );
}

