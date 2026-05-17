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
#[test]
pub fn test_openapi() {
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
            "openapi": "3.1.0",
            "paths": {
                "/a/hello": {
                    "get": {
                        "operationId": ("hello"),
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
                        "operationId": ("hello"), // todo: forbid duplicates
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

/// Disallow having different components with the same schema name when merging controllers.
mod disallow_overlaps {
    use groom_macros::Controller;
    use utoipa::openapi::OpenApiBuilder;

    #[Controller()]
    pub mod controller1 {
        use axum::{response::IntoResponse};

        use groom::{
            response::Response
        };
        use groom_macros::{DTO, Response};
        use utoipa::PartialSchema;

        #[DTO(response)]
        pub struct RespData {
            pub v: i32,
        }

        #[Response(format(json))]
        pub enum HelloResult {
            #[Response()]
            Ok(RespData),
        }

        #[Route(method = "get", path = "/1/hello")]
        async fn hello() -> HelloResult {
            HelloResult::Ok(RespData{v:123})
        }
    }

    #[Controller()]
    pub mod controller2 {
        use axum::{response::IntoResponse};

        use groom::{
            extract::GroomExtractor,
            response::Response
        };
        use groom_macros::{DTO, Response};
        use utoipa::PartialSchema;

        #[DTO(response)]
        pub struct RespData {
            pub v: String,
        }

        #[Response(format(json))]
        pub enum HelloResult {
            #[Response()]
            Ok(RespData),
        }

        #[Route(method = "get", path = "/2/hello")]
        async fn hello() -> HelloResult {
            HelloResult::Ok(RespData{v:"456".into()})
        }
    }

    #[test]
    #[should_panic(expected = "Component `RespData` is defined more then once.")]
    fn test_openapi_doc() {
        let b = OpenApiBuilder::new();
        let b = controller1::merge_into_openapi_builder(b);
        let _ = controller2::merge_into_openapi_builder(b);
    }
}

/// Allow having same shared component when merging controllers.
mod allow_shared_components {
    use groom_macros::{Controller, DTO};
    use serde_json::json;

    use crate::integration::test_utils::assert_openapi_doc;

    #[DTO(response)]
    pub struct RespData {
        pub v: i32,
    }

    #[DTO(response)]
    pub struct RespData2 {
        pub v2: i32,
    }

    #[Controller()]
    pub mod controller1 {
        use axum::{response::IntoResponse};

        use groom::{
            response::Response
        };
        use groom_macros::Response;
        use crate::integration::features::multiple_controllers::allow_shared_components::RespData2;

        use super::RespData;

        use utoipa::PartialSchema;

        #[Response(format(json))]
        pub enum HelloResult {
            #[Response()]
            Ok(RespData),

            #[Response(code=202)]
            Ok2(RespData2),
        }

        #[Route(method = "get", path = "/1/hello")]
        async fn hello() -> HelloResult {
            HelloResult::Ok(RespData{v:123})
        }
    }

    #[Controller()]
    pub mod controller2 {
        use axum::{response::IntoResponse};

        use groom::{
            response::Response
        };
        use groom_macros::Response;
        use super::RespData;

        //use utoipa::PartialSchema;


        #[Response(format(json))]
        pub enum HelloResult {
            #[Response()]
            Ok(RespData),
        }

        #[Route(method = "get", path = "/2/hello")]
        async fn hello() -> HelloResult {
            HelloResult::Ok(RespData{v:123})
        }
    }

    #[test]
    fn test_openapi_doc() {
        assert_openapi_doc(
            |b| {
                let b = controller1::merge_into_openapi_builder(b);
                let b = controller2::merge_into_openapi_builder(b);
                b
            },
            json!( {
                "components": {
                    "schemas": {
                        "RespData": {
                            "properties": {
                                "v": {
                                    "format": ("int32"),
                                    "type": ("integer"),
                                },
                            },
                            "required": [
                                ("v"),
                            ],
                            "type": ("object"),
                        },
                        "RespData2": {
                            "properties": {
                                "v2": {
                                    "format": ("int32"),
                                    "type": ("integer"),
                                },
                            },
                            "required": [
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
                    "/1/hello": {
                        "get": {
                            "operationId": ("hello"),
                            "responses": {
                                "200": {
                                    "content": {
                                        "application/json": {
                                            "schema": {
                                                "$ref": "#/components/schemas/RespData",
                                            },
                                        },
                                    },
                                    "description": (""),
                                },
                                "202": {
                                    "content": {
                                        "application/json": {
                                            "schema": {
                                                "$ref": "#/components/schemas/RespData2",
                                            },
                                        },
                                    },
                                    "description": (""),
                                },
                            },
                        },
                    },
                    "/2/hello": {
                        "get": {
                            "operationId": ("hello"),
                            "responses": {
                                "200": {
                                    "content": {
                                        "application/json": {
                                            "schema": {
                                                "$ref": "#/components/schemas/RespData",
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
}
