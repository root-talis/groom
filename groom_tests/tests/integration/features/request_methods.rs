use axum::Router;

use crate::{
    groom_macros::Controller,
    integration::test_utils::{Req, assert_openapi_doc}
};

use serde_json::json;


#[Controller()]
mod controller {
    use crate::groom_macros::Response;

    use axum::response::IntoResponse;
    use groom::response::Response;

    #[Response()]
    pub enum RootResponse {
        #[Response(code = 200)]
        OkGet,

        #[Response(code = 201)]
        OkPost,

        #[Response(code = 202)]
        OkPut,

        #[Response(code = 203)]
        OkDelete,

        #[Response(code = 204)]
        OkPatch,
    }

    #[Route(method="get", path="/")]
    pub async fn root_get() -> RootResponse {
        RootResponse::OkGet
    }

    #[Route(method="post", path="/")]
    pub async fn root_post() -> RootResponse {
        RootResponse::OkPost
    }

    #[Route(method="put", path="/")]
    pub async fn root_put() -> RootResponse {
        RootResponse::OkPut
    }

    #[Route(method="delete", path="/")]
    pub async fn root_delete() -> RootResponse {
        RootResponse::OkDelete
    }

    #[Route(method="patch", path="/")]
    pub async fn root_patch() -> RootResponse {
        RootResponse::OkPatch
    }
}


/// Tests that handler for delete request is set correctly
#[tokio::test]
pub async fn test_delete() {
    let r = controller::merge_into_router(Router::new());

    Req::delete("/").call(&r).await
        .assert_body("")
        .assert_status(203)
        .assert_no_content_type()
    ;
}

/// Tests that handler for get request is set correctly
#[tokio::test]
pub async fn test_get() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/").call(&r).await
        .assert_body("")
        .assert_status(200)
        .assert_no_content_type()
    ;
}

/// Tests that handler for patch request is set correctly
#[tokio::test]
pub async fn test_patch() {
    let r = controller::merge_into_router(Router::new());

    Req::patch("/").call(&r).await
        .assert_body("")
        .assert_status(204)
        .assert_no_content_type()
    ;
}

/// Tests that handler for post request is set correctly
#[tokio::test]
pub async fn test_post() {
    let r = controller::merge_into_router(Router::new());

    Req::post("/").call(&r).await
        .assert_body("")
        .assert_status(201)
        .assert_no_content_type()
    ;
}

/// Tests that handler for pu request is set correctly
#[tokio::test]
pub async fn test_put() {
    let r = controller::merge_into_router(Router::new());

    Req::put("/").call(&r).await
        .assert_body("")
        .assert_status(202)
        .assert_no_content_type()
    ;
}

/// Tests that openapi definition is correctly generated
#[tokio::test]
pub async fn test_openapi() {
    assert_openapi_doc(
        |b| controller::merge_into_openapi_builder(b),
        json!({
            "openapi":"3.0.3",
            "info":{
                "title":"t",
                "description":"d",
                "contact":{"name":"name","email":"mail@example.com"},
                "license":{"name":"n"},
                "version":"0.0.0"
            },
            "paths":{
                "/":{
                    "get":{
                        "responses":{
                            "200":{"description":""},
                            "201":{"description":""},
                            "202":{"description":""},
                            "203":{"description":""},
                            "204":{"description":""}
                        }
                    },
                    "post":{
                        "responses":{
                            "200":{"description":""},
                            "201":{"description":""},
                            "202":{"description":""},
                            "203":{"description":""},
                            "204":{"description":""}
                        }
                    },
                    "put":{
                        "responses":{
                            "200":{"description":""},
                            "201":{"description":""},
                            "202":{"description":""},
                            "203":{"description":""},
                            "204":{"description":""}
                        }
                    },
                    "delete":{
                        "responses":{
                            "200":{"description":""},
                            "201":{"description":""},
                            "202":{"description":""},
                            "203":{"description":""},
                            "204":{"description":""}
                        }
                    },
                    "patch":{
                        "responses":{
                            "200":{"description":""},
                            "201":{"description":""},
                            "202":{"description":""},
                            "203":{"description":""},
                            "204":{"description":""}
                        }
                    }
                }
            }
        })
    );
}
