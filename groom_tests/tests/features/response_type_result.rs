use axum::Router;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    features::{test_utils::{Req, assert_openapi_doc}}
};

#[Controller()]
mod controller {
    use axum::{extract::Query, response::{IntoResponse}};

    use groom::{
        response::Response,
        extract::GroomExtractor
    };
    use groom_macros::{DTO,Response};
    use serde::Deserialize;
    use utoipa::ToSchema;
    use utoipa::PartialSchema;

    #[DTO(parameters)]
    pub struct Req {
        pub id: u8
    }

    #[Response(format(json), code = 200)]
    pub struct OkResponse {
        id: u8,
        name: &'static str
    }

    #[DTO(response)]
    pub struct ErrDescription {
        error: &'static str
    }

    impl From<&'static str> for ErrDescription {
        fn from(error: &'static str) -> Self {
            Self{ error }
        }
    }
    
    #[Response(format(json))]
    pub enum ErrorResponse {
        #[Response(code = 400)]
        BadRequest(ErrDescription),
        
        #[Response(code = 404)]
        NotFound(ErrDescription),
    }

    #[Route(method = "get", path = "/")]
    async fn handler(Query(q): Query<Req>) -> Result<OkResponse, ErrorResponse> {
        if q.id == 0 {
            return Err(ErrorResponse::BadRequest("id cannot be zero".into()))
        } 
        
        let name = get_message(q.id).ok_or(ErrorResponse::NotFound("id not found".into()))?;
        Ok(OkResponse { id: q.id, name })
    }

    fn get_message(id: u8) -> Option<&'static str> {
        match id {
            1 => Some("first"),
            2 => Some("second"),
            _ => None
        }
    }
}


#[tokio::test]
pub async fn test_ok() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/?id=1").call(&r).await
        .assert_body(r#"{"id":1,"name":"first"}"#)
        .assert_status(200)
        .assert_content_type("application/json")
    ;

    Req::get("/?id=2").call(&r).await
        .assert_body(r#"{"id":2,"name":"second"}"#)
        .assert_status(200)
        .assert_content_type("application/json")
    ;
}


#[tokio::test]
pub async fn test_not_found() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/?id=3").call(&r).await
        .assert_body(r#"{"error":"id not found"}"#)
        .assert_status(404)
    ;
}

#[tokio::test]
pub async fn test_bad_request() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/?id=0").call(&r).await
        .assert_body(r#"{"error":"id cannot be zero"}"#)
        .assert_status(400)
    ;
}

#[test]
pub fn test_openapi() {
    assert_openapi_doc(
        |b| controller::merge_into_openapi_builder(b),
        json!( {
            "components": {
                "schemas": {
                    "ErrDescription": {
                        "properties": {
                            "error": {
                                "type": ("string"),
                            },
                        },
                        "required": [
                            ("error"),
                        ],
                        "type": ("object"),
                    },
                    "OkResponse": {
                        "properties": {
                            "id": {
                                "format": ("int32"),
                                "minimum": (0),
                                "type": ("integer"),
                            },
                            "name": {
                                "type": ("string"),
                            },
                        },
                        "required": [
                            ("id"),
                            ("name"),
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
                "/": {
                    "get": {
                        "operationId": ("handler"),
                        "parameters": [
                            {
                                "in": ("query"),
                                "name": ("id"),
                                "required": (true),
                                "schema": {
                                    "format": ("int32"),
                                    "minimum": (0),
                                    "type": ("integer"),
                                },
                            },
                        ],
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/OkResponse"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "400": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/ErrDescription"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "404": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/ErrDescription"),
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
    )
}
