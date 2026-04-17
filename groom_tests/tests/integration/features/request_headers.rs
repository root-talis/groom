use axum::Router;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::test_utils::{Req, assert_openapi_doc}
};

#[Controller()]
mod controller {
    use axum::{response::IntoResponse, http::HeaderMap};

    use groom::{
        response::Response,
        extract::GroomExtractor
    };
    use groom_macros::Response;

    // ---

    #[Response(format(plain_text))]
    pub enum TextResponse {
        #[Response()]
        Ok(String),
    }

    #[Route(method = "get", path = "/header-map")]
    async fn rq_cons_header_map(h: HeaderMap) -> TextResponse {
        let token = h.get("x-access-token");
        TextResponse::Ok(format!(
            "token: {}",
            token.map_or("none", |t| t.to_str().unwrap())
        ))
    }
}

// axum::http::HeaderMap
#[tokio::test]
pub async fn test_get_axum_http_header_map() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/header-map").with_headers([("x-access-token", "123456789")]).call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("token: 123456789")
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
            "openapi": "3.0.3",
            "paths": {
                "/header-map": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
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
            },
        })
    );
}
