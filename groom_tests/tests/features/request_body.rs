use axum::{Router};
use serde_json::json;

use crate::{
    groom_macros::Controller,
    features::test_utils::{Req, ReqBody, assert_openapi_doc}
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

    use serde::Deserialize;
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

    // ---

    #[RequestBody(format(url_encoded))]
    pub struct StatusFilter {
        status: Vec<Status>,
    }

    #[RequestBody(format(url_encoded))]
    pub struct OptStatusFilter {
        status: Option<Vec<Status>>,
    }

    #[derive(Default, Deserialize, ToSchema, PartialEq)]
    pub enum Status {
        #[default]
        New,
        Closed
    }

    #[Route(method = "post", path = "/url_encoded_vec_enum")]
    async fn url_encoded_vec_enum(body: StatusFilter) -> StringResponse {
        StringResponse::Ok(format!(
            "new: {}, closed: {}",
            if body.status.contains(&Status::New) {"y"} else {"n"},
            if body.status.contains(&Status::Closed) {"y"} else {"n"},
        ))
    }

    #[Route(method = "post", path = "/url_encoded_opt_vec_enum")]
    async fn url_encoded_opt_vec_enum(body: OptStatusFilter) -> StringResponse {
        if let Some(ref status) = body.status {
            StringResponse::Ok(format!(
                "new: {}, closed: {}",
                if status.contains(&Status::New) {"y"} else {"n"},
                if status.contains(&Status::Closed) {"y"} else {"n"},
            ))
        } else {
            StringResponse::Ok("null".into())
        }
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

/// Test that url_encoded RequestBody is correctly read for Vec<Enum>
#[tokio::test]
pub async fn test_url_encoded_vec_of_enums() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/url_encoded_vec_enum")
        .with_body(
            ReqBody::new("status=New")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_body("new: y, closed: n")
        .assert_status(200)
    ;

    Req::post("/url_encoded_vec_enum")
        .with_body(
            ReqBody::new("status=Closed")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_body("new: n, closed: y")
        .assert_status(200)
    ;

    Req::post("/url_encoded_vec_enum")
        .with_body(
            ReqBody::new("status=New&status=Closed")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_body("new: y, closed: y")
        .assert_status(200)
    ;

    Req::post("/url_encoded_vec_enum")
        .with_body(
            ReqBody::new("")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_body("Failed to deserialize form body: missing field `status`")
        .assert_status(422)
    ;
}

/// Test that url_encoded RequestBody is correctly read for Option<Vec<Enum>>
#[tokio::test]
pub async fn test_url_encoded_opt_vec_of_enums() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/url_encoded_opt_vec_enum")
        .with_body(
            ReqBody::new("status=New")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_body("new: y, closed: n")
        .assert_status(200)
    ;

    Req::post("/url_encoded_opt_vec_enum")
        .with_body(
            ReqBody::new("status=Closed")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_body("new: n, closed: y")
        .assert_status(200)
    ;

    Req::post("/url_encoded_opt_vec_enum")
        .with_body(
            ReqBody::new("status=New&status=Closed")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_body("new: y, closed: y")
        .assert_status(200)
    ;

    Req::post("/url_encoded_opt_vec_enum")
        .with_body(
            ReqBody::new("")
                .with_content_type("application/x-www-form-urlencoded")
        )
        .call(&r)
        .await
        .assert_body("null")
        .assert_status(200)
    ;
}


/// Tests that openapi definition is correctly generated
#[test]
pub fn test_openapi() {
    assert_openapi_doc(
        |b| controller::merge_into_openapi_builder(b),
        json!({
            "components": {
                "schemas": {
                    "MultiFormatDto": {
                        "description": "Some DTO",
                        "properties": {
                            "age": {
                                "format": "int32",
                                "minimum": 0,
                                "type": [
                                    "integer",
                                    "null",
                                ],
                            },
                            "name": {
                                "type": "string",
                            },
                        },
                        "required":  [
                            "name",
                        ],
                        "type": "object",
                    },
                    "MultiFormatRequestBody": {
                        "description": "Request body as a named struct.",
                        "properties": {
                            "age": {
                                "description": "Person's age",
                                "format": "int32",
                                "minimum": 0,
                                "type": [
                                    "integer",
                                    "null",
                                ],
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
                    "OptStatusFilter": {
                        "properties": {
                            "status": {
                                "items": {
                                    "$ref": "#/components/schemas/Status",
                                },
                                "type": [
                                    "array",
                                    "null",
                                ],
                            },
                        },
                        "type": "object",
                    },
                    "Status": {
                        "enum": [
                            "New",
                            "Closed",
                        ],
                        "type": "string",
                    },
                    "StatusFilter": {
                        "properties": {
                            "status": {
                                "items": {
                                    "$ref": "#/components/schemas/Status",
                                },
                                "type": "array",
                            },
                        },
                        "required": [
                            "status",
                        ],
                        "type": "object",
                    },
                },
            },
            "info": {
                "contact": {
                    "email": "mail@example.com",
                    "name": "name",
                },
                "description": "d",
                "license": {
                    "name": "n",
                },
                "title": "t",
                "version": "0.0.0",
            },
            "openapi": "3.1.0",
            "paths": {
                "/bytes_body": {
                    "post": {
                        "operationId": ("rqConsBytesBody"),
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
                        "operationId": ("rqConsImageBody"),
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
                        "operationId": ("rqConsMultiFormatBody"),
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/MultiFormatRequestBody"
                                    },
                                },
                                "application/x-www-form-urlencoded": {
                                    "schema": {
                                        "$ref": "#/components/schemas/MultiFormatRequestBody",
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
                        "operationId": ("rqConsMultiFormatBodyDto"),
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/MultiFormatDto",
                                    },
                                },
                                "application/x-www-form-urlencoded": {
                                    "schema": {
                                        "$ref": "#/components/schemas/MultiFormatDto",
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
                        "operationId": ("rqConsStringBody"),
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
                "/url_encoded_opt_vec_enum": {
                    "post": {
                        "operationId": ("urlEncodedOptVecEnum"),
                        "requestBody": {
                            "content": {
                                "application/x-www-form-urlencoded": {
                                    "schema": {
                                        "$ref": "#/components/schemas/OptStatusFilter",
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
                    },
                },
                "/url_encoded_vec_enum": {
                    "post": {
                        "operationId": ("urlEncodedVecEnum"),
                        "requestBody": {
                            "content": {
                                "application/x-www-form-urlencoded": {
                                    "schema": {
                                        "$ref": "#/components/schemas/StatusFilter",
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
                    },
                },
            },
        })
    );
}
