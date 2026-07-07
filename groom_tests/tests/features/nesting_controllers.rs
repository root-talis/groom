use serde_json::json;

use crate::features::test_utils::{Req, assert_openapi_doc};

use groom::router::GroomRouter;


mod shared_types {
    use groom_macros::DTO;

    #[DTO(response)]
    pub struct SharedItem {
        pub id: u32,
        pub name: String,
    }
}

mod root {
    use groom_macros::Controller;
    use super::GroomRouter;

    pub fn into_router() -> GroomRouter {
        controller::into_router()
    }

    #[Controller()]
    pub mod controller {
        use axum::response::IntoResponse;
        use groom::response::Response;
        use groom_macros::Response;
        use super::super::shared_types::SharedItem;

        #[Response(format(json))]
        pub enum SharedItemResponse {
            #[Response(code = 200)]
            Ok(SharedItem),
        }

        #[Route(method = "get", path = "/")]
        pub async fn get_root() -> SharedItemResponse {
            SharedItemResponse::Ok(SharedItem { id: 0, name: "root".into() })
        }
    }
}

mod api_v1 {
    use groom_macros::Controller;
    use super::GroomRouter;

    pub fn into_router() -> GroomRouter {
        controller::into_router()
    }

    #[Controller()]
    pub mod controller {
        use axum::response::IntoResponse;
        use groom::response::Response;
        use groom_macros::{DTO, Response};
        use super::super::shared_types::SharedItem;

        #[DTO(response)]
        pub struct FooItem {
            pub id: u32,
            pub value: String,
        }

        #[DTO(response)]
        pub struct BarItem {
            pub id: u32,
            pub count: u32,
        }

        #[Response(format(json))]
        pub enum FooResponse {
            #[Response(code = 200)]
            Ok(FooItem),
        }

        #[Response(format(json))]
        pub enum BarResponse {
            #[Response(code = 200)]
            Ok(BarItem),
        }

        #[Response(format(json))]
        pub enum SharedItemWrapperResponse {
            #[Response(code = 200)]
            Ok(SharedItem),
        }

        #[Route(method = "get", path = "/foo")]
        pub async fn get_foo() -> FooResponse {
            FooResponse::Ok(FooItem { id: 1, value: "foo-value".into() })
        }

        #[Route(method = "get", path = "/bar")]
        pub async fn get_bar() -> BarResponse {
            BarResponse::Ok(BarItem { id: 2, count: 42 })
        }

        #[Route(method = "get", path = "/shared")]
        pub async fn get_shared() -> SharedItemWrapperResponse {
            SharedItemWrapperResponse::Ok(SharedItem { id: 3, name: "shared-from-v1".into() })
        }
    }
}

mod api_v2 {
    use groom_macros::Controller;
    use super::hello;
    use super::GroomRouter;

    pub fn into_router_with_nest() -> GroomRouter {
        let r = controller::into_router();
        let hello_r = hello::into_router();
        r.nest("/hello", hello_r).expect("nest hello under api_v2 failed")
    }

    #[Controller()]
    pub mod controller {
        use axum::response::IntoResponse;
        use groom::response::Response;
        use groom_macros::{DTO, Response};

        #[DTO(response)]
        pub struct V2Item {
            pub version: String,
        }

        #[Response(format(json))]
        pub enum V2Response {
            #[Response(code = 200)]
            Ok(V2Item),
        }

        #[Route(method = "get", path = "/")]
        pub async fn get_api_v2() -> V2Response {
            V2Response::Ok(V2Item { version: "2.0".into() })
        }
    }
}

mod hello {
    use groom_macros::Controller;
    use super::GroomRouter;

    pub fn into_router() -> GroomRouter {
        controller::into_router()
    }

    #[Controller()]
    pub mod controller {
        use axum::response::IntoResponse;
        use groom::response::Response;
        use groom_macros::{DTO, Response};

        #[DTO(response)]
        pub struct WorldItem {
            pub greeting: String,
        }

        #[Response(format(json))]
        pub enum WorldResponse {
            #[Response(code = 200)]
            Ok(WorldItem),
        }

        #[Route(method = "get", path = "/world")]
        pub async fn get_world() -> WorldResponse {
            WorldResponse::Ok(WorldItem { greeting: "hello from deep nest".into() })
        }
    }
}

fn bootstrap_router() -> axum::Router {
    GroomRouter::new()
        .merge(root::into_router())
        .expect("merge root failed")
        .nest("/api/v1", api_v1::into_router())
        .expect("nest api_v1 failed")
        .nest("/api/v2", api_v2::into_router_with_nest())
        .expect("nest api_v2 failed")
        .validate()
        .expect("validate failed")
        .to_axum_router()
        .with_state(())
}



#[tokio::test]
async fn test_routing() {
    let router = bootstrap_router();

    Req::get("/").call(&router).await
        .assert_status(200);

    Req::get("/api/v1/foo").call(&router).await
        .assert_status(200);

    Req::get("/api/v1/bar").call(&router).await
        .assert_status(200);

    Req::get("/api/v2/hello/world").call(&router).await
        .assert_status(200);

    Req::get("/api/v1/nonexistent").call(&router).await
        .assert_status(404);
}

#[tokio::test]
async fn test_root_controller_standalone() {
    let r = GroomRouter::new()
        .merge(root::into_router())
        .expect("merge root failed")
        .validate()
        .expect("validate failed")
        .to_axum_router();
    let router = r.with_state(());

    Req::get("/").call(&router).await
        .assert_status(200);
}

#[tokio::test]
async fn test_two_level_deep_nesting() {
    let r = GroomRouter::new()
        .merge(root::into_router())
        .expect("merge root failed")
        .nest("/api/v1", api_v1::into_router())
        .expect("nest api_v1 failed")
        .validate()
        .expect("validate failed")
        .to_axum_router();
    let router = r.with_state(());

    Req::get("/api/v1/foo").call(&router).await
        .assert_status(200);
}


#[test]
pub fn test_openapi_schema() {
    assert_openapi_doc(
        |api| {
            GroomRouter::new()
                .merge(root::into_router())
                .expect("merge root failed")
                .nest("/api/v1", api_v1::into_router())
                .expect("nest api_v1 failed")
                .nest("/api/v2", api_v2::into_router_with_nest())
                .expect("nest api_v2 failed")
                .validate()
                .expect("validate failed")
                .to_openapi(api)
        },
        json!({
            "openapi": "3.1.0",
            "info": {
                "title": "t",
                "description": "d",
                "license": {"name": "n"},
                "version": "0.0.0",
                "contact": {"name": "name", "email": "mail@example.com"}
            },
            "paths": {
                "/": {
                    "get": {
                        "operationId": "getRoot",
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "application/json": {
                                        "schema": {"$ref": "#/components/schemas/SharedItem"}
                                    }
                                }
                            }
                        }
                    }
                },
                "/api/v1/bar": {
                    "get": {
                        "operationId": "getBar",
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "application/json": {
                                        "schema": {"$ref": "#/components/schemas/BarItem"}
                                    }
                                }
                            }
                        }
                    }
                },
                "/api/v1/foo": {
                    "get": {
                        "operationId": "getFoo",
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "application/json": {
                                        "schema": {"$ref": "#/components/schemas/FooItem"}
                                    }
                                }
                            }
                        }
                    }
                },
                "/api/v1/shared": {
                    "get": {
                        "operationId": "getShared",
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "application/json": {
                                        "schema": {"$ref": "#/components/schemas/SharedItem"}
                                    }
                                }
                            }
                        }
                    }
                },
                "/api/v2": {
                    "get": {
                        "operationId": "getApiV2",
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "application/json": {
                                        "schema": {"$ref": "#/components/schemas/V2Item"}
                                    }
                                }
                            }
                        }
                    }
                },
                "/api/v2/hello/world": {
                    "get": {
                        "operationId": "getWorld",
                        "responses": {
                            "200": {
                                "description": "",
                                "content": {
                                    "application/json": {
                                        "schema": {"$ref": "#/components/schemas/WorldItem"}
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "BarItem": {
                        "type": "object",
                        "required": ["id", "count"],
                        "properties": {
                            "count": {"type": "integer", "format": "int32", "minimum": 0},
                            "id": {"type": "integer", "format": "int32", "minimum": 0}
                        }
                    },
                    "FooItem": {
                        "type": "object",
                        "required": ["id", "value"],
                        "properties": {
                            "id": {"type": "integer", "format": "int32", "minimum": 0},
                            "value": {"type": "string"}
                        }
                    },
                    "SharedItem": {
                        "type": "object",
                        "required": ["id", "name"],
                        "properties": {
                            "id": {"type": "integer", "format": "int32", "minimum": 0},
                            "name": {"type": "string"}
                        }
                    },
                    "V2Item": {
                        "type": "object",
                        "required": ["version"],
                        "properties": {
                            "version": {"type": "string"}
                        }
                    },
                    "WorldItem": {
                        "type": "object",
                        "required": ["greeting"],
                        "properties": {
                            "greeting": {"type": "string"}
                        }
                    }
                }
            }
        })
    );
}
