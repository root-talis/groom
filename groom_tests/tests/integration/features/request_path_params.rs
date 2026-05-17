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
use serde::Deserialize;
use utoipa::ToSchema;

    // ---

    /// TODO: this should be put under #/components/parameters
    #[DTO(parameters)]
    pub struct PathParams {
        pub team_id: String,
        pub user_id: i32,
        pub sort_by: SortBy,
    }

    #[derive(Deserialize, ToSchema)]
    pub enum SortBy {
        Id,
        Title,
        Status
    }

    #[Response(format(plain_text))]
    pub enum TextResponse {
        #[Response()]
        Ok(String),
    }

    #[Route(method = "get", path = "/team/{team_id}/user/{user_id}/{sort_by}")]
    async fn path_params(Path(team): Path<PathParams>) -> TextResponse {
        TextResponse::Ok(format!("{} -> {} / {}", team.user_id, team.team_id, match team.sort_by {
            SortBy::Id     => "id sort",
            SortBy::Title  => "title sort",
            SortBy::Status => "status sort",
        }))
    }
}

/// Test that Path parameters are correctly read
#[tokio::test]
pub async fn test_path_params() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/team/7/user/1/Id").call(&r).await
        .assert_status(200)
        .assert_body("1 -> 7 / id sort")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/team/Hitchhikers/user/42/Status").call(&r).await
        .assert_status(200)
        .assert_body("42 -> Hitchhikers / status sort")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

// Todo: HashMap in query

/// Tests that openapi definition is correctly generated
#[test]
pub fn test_openapi() {
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
            "components": {
                "schemas": {
                    "SortBy": {
                        "enum": [
                            ("Id"),
                            ("Title"),
                            ("Status"),
                        ],
                        "type": ("string"),
                    },
                },
            },
            "paths": {
                "/team/{team_id}/user/{user_id}/{sort_by}": {
                    "get": {
                        "operationId": ("pathParams"),
                        "parameters": [
                            {
                                "in": "path",
                                "name": "team_id",
                                "required": true,
                                "schema": {
                                    "type": ("string"),
                                }
                            },
                            {
                                "in": "path",
                                "name": "user_id",
                                "required": true,
                                "schema": {
                                    "type": ("integer"),
                                    "format": "int32"
                                },
                            },
                            {
                                "in": ("path"),
                                "name": ("sort_by"),
                                "required": (true),
                                "schema": {
                                    "$ref": ("#/components/schemas/SortBy"),
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
