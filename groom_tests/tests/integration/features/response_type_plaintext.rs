use axum::Router;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::{features::response_type_plaintext::controller::SOME_TEXT, test_utils::{Req, assert_openapi_doc}}
};

#[Controller()]
mod controller {
    use axum::response::IntoResponse;
    use groom::response::Response;
    use groom_macros::Response;


    /// https://twitter.com/stahnma/status/634849376343429120
    pub const SOME_TEXT: &'static str = "Everybody has a testing environment. \
                                    Some people are lucky enough enough to have a totally separate environment \
                                    to run production in";
    // ---

    #[Response(format(plain_text))]
    pub enum StringResponse {
        #[Response()]
        Ok(String)
    }

    #[Route(method="get", path="/string")]
    pub async fn string() -> StringResponse {
        StringResponse::Ok(SOME_TEXT.into())
    }
}

/// Tests that handler for delete request is set correctly
#[tokio::test]
pub async fn test_string() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/string").call(&r).await
        .assert_body(SOME_TEXT)
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Tests that openapi definition is correctly generated
#[tokio::test]
pub async fn test_openapi() {
    assert_openapi_doc(
        |b| controller::merge_into_openapi_builder(b),
        json!({
            "info": {
                "contact": {"email": "mail@example.com","name": "name"},
                "description": "d",
                "license": {"name": "n"},
                "title": "t",
                "version": "0.0.0",
            },
            "openapi": "3.0.3",
            "paths": {
                "/string": {
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
