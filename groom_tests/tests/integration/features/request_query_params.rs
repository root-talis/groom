use axum::Router;
use groom::response::HtmlFormat;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    integration::test_utils::{Req, assert_openapi_doc}
};

#[Controller()]
mod controller {
    use axum::{extract::Query, response::IntoResponse};
    use groom::{html_format, response::Response, schema::GroomSchema, extract::GroomExtractor};
    use groom_macros::{DTO,Response};

    use utoipa::ToSchema;

    // ---

    #[DTO(request)]
    pub struct Params {
        #[serde(rename="first_name_renamed")]
        first_name: String,
        last_name: Option<String>,
    }

    #[DTO(request)]
    pub struct Params2 {
        title: Option<String>,
    }

    #[Response(format(plain_text))]
    pub enum Resp {
        #[Response()]
        Ok(String),

        #[Response(code = 400)]
        BadRequest(String),
    }

    // ---

    #[Route(method = "get", path = "/two_query_inputs")]
    pub async fn two_query_inputs(query: Query<Params>, query2: Query<Params2>) -> Resp {
        if query.first_name.is_empty() {
            Resp::BadRequest("Empty name".into())
        } else {
            Resp::Ok(format!(
                "Hello, {}{}!",
                query2.title.clone().map_or_else(
                    || "".into(),
                    |t| format!("{} ", t)
                ),
                query.last_name.clone().map_or_else(
                    || query.first_name.clone(),
                    |last_name| format!("{} {}", query.first_name, last_name) 
                )
            ))
        }
    }
}

/// Test that Query parameters are correctly read when there are several structs mapped as Query()
#[tokio::test]
pub async fn test_query_multiple_structs() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/two_query_inputs?first_name_renamed=").call(&r).await
        .assert_status(400)
        .assert_body("Empty name")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/two_query_inputs?first_name_renamed=Luca").call(&r).await
        .assert_status(200)
        .assert_body("Hello, Luca!")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/two_query_inputs?last_name=Freeman&first_name_renamed=Gordon").call(&r).await
        .assert_status(200)
        .assert_body("Hello, Gordon Freeman!")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/two_query_inputs?first_name_renamed=Anderson&title=Mr").call(&r).await
        .assert_status(200)
        .assert_body("Hello, Mr Anderson!")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/two_query_inputs?last_name=Backsword&title=Sir&first_name_renamed=John").call(&r).await
        .assert_status(200)
        .assert_body("Hello, Sir John Backsword!")
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
                "contact": {"email": "mail@example.com","name": "name",},
                "description": "d",
                "license": {"name": "n",},
                "title": "t",
                "version": "0.0.0",
            },
            "openapi": "3.0.3",
            "paths": {
                "/two_query_inputs": {
                    "get": {
                        "parameters": [
                            {
                                "in": "query",
                                "name": "Params",
                                "required": true,
                                "schema": {
                                    "properties": {
                                        "first_name_renamed": {
                                            "type": "string",
                                        },
                                        "last_name": {
                                            "nullable": true,
                                            "type": "string",
                                        },
                                    },
                                    "required": [
                                        "first_name_renamed",
                                    ],
                                    "type": "object",
                                },
                            },
                            {
                                "in": "query",
                                "name": "Params2",
                                "required": true,
                                "schema": {
                                    "properties": {
                                        "title": {
                                            "nullable": true,
                                            "type": "string",
                                        },
                                    },
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
                            "400": {
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
