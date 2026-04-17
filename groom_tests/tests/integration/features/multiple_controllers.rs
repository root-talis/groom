use axum::Router;
use serde_json::json;

use crate::{
    integration::test_utils::{Req, assert_openapi_doc}
};

mod namespace_a {
    use axum::{Extension, Router};
    use groom_macros::Controller;
    use utoipa::openapi::OpenApiBuilder;

    #[derive(Debug, Clone)]
    pub struct ExtensionA {
        pub answer: i128
    }

    #[Controller(state_type = SomeState)]
    pub mod controller {
        use crate::integration::features::multiple_controllers::SomeState;

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

        #[Route(method = "get", path = "/a/hello")]
        async fn hello(s: State<SomeState>, e: Extension<super::ExtensionA>) -> TextResponse {
            TextResponse::Ok(format!("namespace_a / name from state: {}; value from extension: {}", s.name, e.answer))
        }
    }

    pub fn merge_router(r: Router<super::SomeState>) -> Router<super::SomeState> {
        controller::merge_into_router(r)
            .layer(Extension(ExtensionA{ answer: 42 }))
    }

    pub fn merge_api_spec(b: OpenApiBuilder) -> OpenApiBuilder {
        controller::merge_into_openapi_builder(b)
    }
}

mod namespace_b {
    use axum::{Extension, Router};
    use groom_macros::Controller;
    use utoipa::openapi::OpenApiBuilder;

    #[derive(Debug, Clone)]
    pub struct ExtensionB {
        pub name: String
    }

    #[Controller(state_type = SomeState)]
    pub mod controller {
        use crate::integration::features::multiple_controllers::SomeState;

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

        #[Route(method = "get", path = "/b/hello")]
        async fn hello(s: State<SomeState>, e: Extension<super::ExtensionB>) -> TextResponse {
            TextResponse::Ok(format!("namespace_b / name from state: {}; value from extension: {}", s.name, e.name))
        }
    }

    pub fn merge_router(r: Router<super::SomeState>) -> Router<super::SomeState> {
        controller::merge_into_router(r)
            .layer(Extension(ExtensionB{ name: "Luca".into() }))
    }

    pub fn merge_api_spec(b: OpenApiBuilder) -> OpenApiBuilder {
        controller::merge_into_openapi_builder(b)
    }
}

#[derive(Clone)]
pub struct SomeState {
    pub name: &'static str
}

fn bootstrap_router() -> Router {
    // init router
    let r = Router::new();

    // add controllers from different modules
    let r = namespace_a::merge_router(r);
    let r = namespace_b::merge_router(r);

    // finally add the shared state
    r.with_state(SomeState {name: "Victoria"})
}

#[tokio::test]
pub async fn test_merge_routers() {
    let r = bootstrap_router();

    Req::get("/a/hello").call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("namespace_a / name from state: Victoria; value from extension: 42")
    ;

    Req::get("/b/hello").call(&r).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("namespace_b / name from state: Victoria; value from extension: Luca")
    ;
}

/// Tests that openapi definition is correctly generated
#[tokio::test]
pub async fn test_openapi() {
    assert_openapi_doc(
        |b| {
            let b = namespace_a::merge_api_spec(b);
            let b = namespace_b::merge_api_spec(b);
            b
        },
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
                "/a/hello": {
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
                "/b/hello": {
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
