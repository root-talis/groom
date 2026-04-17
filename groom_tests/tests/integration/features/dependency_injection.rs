use axum::{Extension, Router};
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::test_utils::{Req, assert_openapi_doc}
};

// Important: state type should be mentioned in `use` INSIDE the controller.
#[Controller(state_type = SomeState)]
mod controller {
    // this import is used to set state_type in #[Controller()] macro above
    use crate::integration::features::dependency_injection::SomeState;

    use axum::{Extension, extract::State, response::IntoResponse};

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

    #[Route(method = "get", path = "/extension")]
    async fn rq_cons_extension(e: Extension<super::SomeExt>) -> TextResponse {
        TextResponse::Ok(format!("name from extension: {}", e.name))
    }

    #[Route(method = "get", path = "/state")]
    async fn rq_cons_state(e: State<super::SomeState>) -> TextResponse {
        TextResponse::Ok(format!("name from state: {}", e.name))
    }
}

#[derive(Clone)]
pub struct SomeState {
    pub name: &'static str
}

#[derive(Clone)]
pub struct SomeExt {
    pub name: &'static str
}

// axum::extract::Extension
#[tokio::test]
pub async fn test_get_axum_extract_extension() {
    let r = controller::merge_into_router(Router::new())
        .layer(Extension(SomeExt {
            name: "Luca"
        }))
        .with_state(SomeState {
            name: "Victoria"
        })
    ;

    Req::get("/extension").call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("name from extension: Luca")
    ;
}

// axum::extract::State
#[tokio::test]
pub async fn test_get_axum_extract_state() {
    let r = controller::merge_into_router(Router::new())
        .layer(Extension(SomeExt {
            name: "Luca"
        }))
        .with_state(SomeState {
            name: "Victoria"
        })
    ;

    Req::get("/state").call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("name from state: Victoria")
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
                "/extension": {
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
                "/state": {
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
