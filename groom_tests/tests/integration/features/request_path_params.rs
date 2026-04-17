use axum::Router;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::test_utils::{Req, assert_openapi_doc}
};

#[Controller()]
mod controller {
    use axum::{extract::Path, response::IntoResponse};

    use groom::{
        response::Response,
        extract::GroomExtractor
    };
    use groom_macros::{DTO,Response};

    // ---

    #[DTO(request)]
    #[serde(rename="RqConsPathStructRenamed")]
    pub struct PathParams {
        pub team_id: String,
        pub user_id: i32,
    }

    #[Response(format(plain_text))]
    pub enum TextResponse {
        #[Response()]
        Ok(String),
    }

    #[Route(method = "get", path = "/team/:team_id/user/:user_id")]
    async fn rq_cons_path_struct(Path(team): Path<PathParams>) -> TextResponse {
        TextResponse::Ok(format!("{} -> {}", team.user_id, team.team_id))
    }
}

/// Test that Path parameters are correctly read
#[tokio::test]
pub async fn test_path_params() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/team/7/user/1").call(&r).await
        .assert_status(200)
        .assert_body("1 -> 7")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/team/Hitchhikers/user/42").call(&r).await
        .assert_status(200)
        .assert_body("42 -> Hitchhikers")
        .assert_content_type("text/plain; charset=utf-8")
    ;
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
            "openapi": "3.0.3",
            "paths": {
                "/team/:team_id/user/:user_id": {
                    "get": {
                        "parameters": [
                            {
                                "in": "path",
                                "name": "PathParams",
                                "required": true,
                                "schema": {
                                    "properties": {
                                        "team_id": {
                                            "type": "string",
                                        },
                                        "user_id": {
                                            "format": "int32",
                                            "type": "integer",
                                        },
                                    },
                                    "required": [
                                        "team_id",
                                        "user_id",
                                    ],
                                    "type": "object",
                                },
                            },
                        ],
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
