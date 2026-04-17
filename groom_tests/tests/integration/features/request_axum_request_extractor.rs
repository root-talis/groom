use axum::Router;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::test_utils::{Req, assert_openapi_doc}
};

#[Controller()]
mod controller {
    use axum::{extract::Request, response::IntoResponse};
    
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

    #[Route(method = "get", path = "/request-extractor")]
    async fn rq_cons_request(req: Request) -> TextResponse {
        let uri = req.uri().to_string();
        TextResponse::Ok(format!("uri: {uri}"))
    }
}

/// Test that Path parameters are correctly read
#[tokio::test]
pub async fn test_path_params() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/request-extractor?id=123456").call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("uri: /request-extractor?id=123456");
}

// Todo: HashMap in query

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
            "openapi": "3.1.0",
            "paths": {
                "/request-extractor": {
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
            "components": {},
        })
    );
}
