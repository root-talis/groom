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

#[Controller()]
mod union_controller {
    use axum::response::IntoResponse;

    use groom::{
        response::Response,
    };
    use groom_macros::{DTO,Response};

    #[DTO(response)]
    pub struct UnionOk {
        pub id: u64,
    }

    #[DTO(response)]
    pub struct UnionErr {
        pub error: &'static str,
    }

    #[Response(format(json))]
    pub enum UnionA {
        #[Response()]
        Ok(UnionOk),
    }

    #[Response(format(json))]
    pub enum UnionB {
        #[allow(dead_code)]
        #[Response(code = 400)]
        Ok(UnionErr),
    }

    #[Route(method = "get", path = "/union")]
    pub async fn resp_union() -> Result<UnionA, UnionB> {
        Ok(UnionA::Ok(UnionOk { id: 42 }))
    }

    #[Route(method = "get", path = "/union-panic")]
    pub async fn resp_union_panic() -> Result<UnionA, UnionB> {
        panic!("handler must not be called")
    }
}


#[tokio::test]
pub async fn test_ok() {
    let r = controller::into_router().validate().unwrap().to_axum_router();

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
    let r = controller::into_router().validate().unwrap().to_axum_router();

    Req::get("/?id=3").call(&r).await
        .assert_body(r#"{"error":"id not found"}"#)
        .assert_status(404)
    ;
}

#[tokio::test]
pub async fn test_bad_request() {
    let r = controller::into_router().validate().unwrap().to_axum_router();

    Req::get("/?id=0").call(&r).await
        .assert_body(r#"{"error":"id cannot be zero"}"#)
        .assert_status(400)
    ;
}

#[test]
pub fn test_openapi() {
    assert_openapi_doc(
        |api| controller::into_router().validate().unwrap().to_openapi(api),
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
    )
}

/// Result<T,E> union pre-check: Accept satisfying the Ok arm runs the handler (SPEC
/// req 5 acceptance).
#[tokio::test]
pub async fn test_union_accept_json_runs_handler() {
    let r = union_controller::into_router().validate().unwrap().to_axum_router();

    Req::get("/union").accept("application/json").call(&r).await
        .assert_status(200)
        .assert_body(r#"{"id":42}"#)
        .assert_content_type("application/json")
    ;
}

/// Result<T,E> union pre-check: Accept for a format the union no longer
/// supports (html, after Phase 9 migrated both arms to json) is unsatisfiable
/// at negotiation level -> 406 without executing the panic-on-call handler.
/// The response-time 400 guard can no longer be reached for format reasons.
#[tokio::test]
pub async fn test_union_accept_html_no_handler_run() {
    let r = union_controller::into_router().validate().unwrap().to_axum_router();

    let res = Req::get("/union-panic").accept("text/html").call(&r).await;
    res.assert_status(406)
        .assert_content_type("text/plain; charset=utf-8");
    assert_eq!(res.headers.get("vary"), Some(&axum::http::HeaderValue::from_static("Accept")));
}

/// Result<T,E> union pre-check: Accept satisfying neither arm returns 406 without
/// executing the panic-on-call handler (SPEC req 5 acceptance).
#[tokio::test]
pub async fn test_union_accept_unsatisfiable_no_handler_run() {
    let r = union_controller::into_router().validate().unwrap().to_axum_router();

    let res = Req::get("/union-panic").accept("application/xml").call(&r).await;
    res.assert_status(406)
        .assert_content_type("text/plain; charset=utf-8");
    assert_eq!(res.headers.get("vary"), Some(&axum::http::HeaderValue::from_static("Accept")));
}

/// The Result<T,E> union operation documents 406 via arm delegation (D-14).
#[test]
pub fn test_union_openapi_has_406() {
    assert_openapi_doc(
        |api| union_controller::into_router().validate().unwrap().to_openapi(api),
        json!({
            "components": {
                "schemas": {
                    "UnionErr": {
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
                    "UnionOk": {
                        "properties": {
                            "id": {
                                "format": ("int64"),
                                "minimum": (0),
                                "type": ("integer"),
                            },
                        },
                        "required": [
                            ("id"),
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
                "/union": {
                    "get": {
                        "operationId": ("respUnion"),
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/UnionOk"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "400": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/UnionErr"),
                                        },
                                    },
                                },
                                "description": (""),
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
                "/union-panic": {
                    "get": {
                        "operationId": ("respUnionPanic"),
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/UnionOk"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "400": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": ("#/components/schemas/UnionErr"),
                                        },
                                    },
                                },
                                "description": (""),
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

/// Deliberately mismatched-format Result union: Ok arm supports [application/json],
/// Err arm supports [text/html]. Phase 9 requires this to fail at router build
/// (into_router -> __groom_runtime_checks) instead of at request time.
#[Controller()]
mod mismatched_union_controller {
    use axum::response::IntoResponse;

    use groom::{
        html_format,
        response::Response,
    };
    use groom_macros::{DTO,Response};

    #[DTO(response)]
    pub struct MismatchedOk {
        pub id: u64,
    }

    #[DTO(response)]
    pub struct MismatchedErr {
        pub error: &'static str,
    }

    html_format!(MismatchedErr, self {
        format!("<div>error: {}</div>", self.error)
    });

    #[Response(format(json))]
    pub enum MismatchedJson {
        #[Response()]
        Ok(MismatchedOk),
    }

    #[Response(format(html))]
    pub enum MismatchedHtml {
        #[allow(dead_code)]
        #[Response(code = 400)]
        Ok(MismatchedErr),
    }

    #[Route(method = "get", path = "/mismatched-union")]
    pub async fn resp_mismatched_union() -> Result<MismatchedJson, MismatchedHtml> {
        Ok(MismatchedJson::Ok(MismatchedOk { id: 42 }))
    }
}

/// Phase 9 regression: a Result whose arms support different format lists panics
/// at router build with the stable message (not at request time).
#[test]
#[should_panic(expected = "must support the same list of formats")]
pub fn test_mismatched_union_formats_panic_at_router_build() {
    let _ = mismatched_union_controller::into_router();
}
