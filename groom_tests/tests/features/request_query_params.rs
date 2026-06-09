use axum::Router;
use serde_json::json;

use crate::{
    groom_macros::Controller,
    features::test_utils::{Req, assert_openapi_doc}
};

#[Controller()]
mod controller {
    use axum::{extract::Query, response::IntoResponse};

    use groom::{
        response::Response,
        extract::GroomExtractor
    };
    use groom_macros::{DTO,Response};
    use serde::Deserialize;
    use utoipa::ToSchema;

    // ---

    #[DTO(parameters)]
    pub struct Params {
        #[serde(rename="first_name_renamed")]
        first_name: String,
        last_name: Option<String>,
    }

    #[DTO(parameters)]
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

    #[DTO(parameters)]
    pub struct Filters {
        #[serde(default)]
        pub sort_by: SortBy,
    }

    #[derive(Default, Deserialize, ToSchema)]
    pub enum SortBy {
        #[default]
        Id,
        Title,
        Status
    }


    #[DTO(parameters)]
    pub struct StatusFilter {
        status: Vec<Status>,
    }

    #[DTO(parameters)]
    pub struct OptStatusFilter {
        status: Option<Vec<Status>>,
    }

    #[derive(Default, Deserialize, ToSchema, PartialEq)]
    pub enum Status {
        #[default]
        New,
        Closed
    }

    // ---

    #[Route(method = "get", path = "/two_query_inputs")]
    pub async fn two_query_inputs(
        // TODO: this should be put under #/components/parameters
        query: Query<Params>,
        
        // TODO: this should be put under #/components/parameters
        query2: Query<Params2>
    ) -> Resp {
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

    #[Route(method = "get", path = "/enum_parameter")]
    pub async fn enum_parameter(
        query: Query<Filters>,
    ) -> Resp {
        Resp::Ok(format!(
            "Sort by {}",
            match query.sort_by {
                SortBy::Id     => "default",
                SortBy::Title  => "text",
                SortBy::Status => "status",
            }
        ))
    }

    #[Route(method = "get", path = "/vec_enum")]
    pub async fn vec_enum(query: axum_extra::extract::Query<StatusFilter>) -> Resp {
        Resp::Ok(format!(
            "new: {}, closed: {}",
            if query.status.contains(&Status::New) {"y"} else {"n"},
            if query.status.contains(&Status::Closed) {"y"} else {"n"},
        ))
    }

    #[Route(method = "get", path = "/opt_vec_enum")]
    pub async fn opt_vec_enum(query: axum_extra::extract::Query<OptStatusFilter>) -> Resp {
        if let Some(ref status) = query.status {
            Resp::Ok(format!(
                "new: {}, closed: {}",
                if status.contains(&Status::New) {"y"} else {"n"},
                if status.contains(&Status::Closed) {"y"} else {"n"},
            ))
        } else {
            Resp::Ok("null".into())
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


/// Test that enum Query parameters are correctly read
#[tokio::test]
pub async fn test_query_enum() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/enum_parameter").call(&r).await
        .assert_status(200)
        .assert_body("Sort by default")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/enum_parameter?sort_by=Id").call(&r).await
        .assert_status(200)
        .assert_body("Sort by default")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/enum_parameter?sort_by=Title").call(&r).await
        .assert_status(200)
        .assert_body("Sort by text")
        .assert_content_type("text/plain; charset=utf-8")
    ;

    Req::get("/enum_parameter?sort_by=Status").call(&r).await
        .assert_status(200)
        .assert_body("Sort by status")
        .assert_content_type("text/plain; charset=utf-8")
    ;
}

/// Test that Query parameters are correctly read for Vec<Enum>
#[tokio::test]
pub async fn test_query_vec_of_enums() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/vec_enum?status=New").call(&r).await
        .assert_body("new: y, closed: n")
        .assert_status(200)
    ;

    Req::get("/vec_enum?status=Closed").call(&r).await
        .assert_body("new: n, closed: y")
        .assert_status(200)
    ;

    Req::get("/vec_enum?status=New&status=Closed").call(&r).await
        .assert_body("new: y, closed: y")
        .assert_status(200)
    ;

    Req::get("/vec_enum").call(&r).await
        .assert_body("Failed to deserialize query string: missing field `status`")
        .assert_status(400)
    ;
}

/// Test that Query parameters are correctly read for Option<Vec<Enum>>
#[tokio::test]
pub async fn test_query_opt_vec_of_enums() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/opt_vec_enum?status=New").call(&r).await
        .assert_body("new: y, closed: n")
        .assert_status(200)
    ;

    Req::get("/opt_vec_enum?status=Closed").call(&r).await
        .assert_body("new: n, closed: y")
        .assert_status(200)
    ;

    Req::get("/opt_vec_enum?status=New&status=Closed").call(&r).await
        .assert_body("new: y, closed: y")
        .assert_status(200)
    ;

    Req::get("/opt_vec_enum").call(&r).await
        .assert_body("null")
        .assert_status(200)
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
                "contact": {"email": "mail@example.com","name": "name",},
                "description": "d",
                "license": {"name": "n",},
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
                    "Status": {
                        "enum": [
                            ("New"),
                            ("Closed"),
                        ],
                        "type": ("string"),
                    },
                },
            },
            "paths": {
                "/two_query_inputs": {
                    "get": {
                        "operationId": ("twoQueryInputs"),
                        "parameters": [
                            {
                                "in": "query",
                                "name": "first_name_renamed",
                                "required": true,
                                "schema": {
                                    "type": "string",
                                },
                            },
                            {
                                "in": "query",
                                "name": "last_name",
                                "required": false,
                                "schema": {
                                    "type": [
                                        "string",
                                        "null",
                                    ],
                                },
                            },
                            {
                                "in": "query",
                                "name": "title",
                                "required": false,
                                "schema": {
                                    "type": [
                                        "string",
                                        "null",
                                    ],
                                }
                            }
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
                "/enum_parameter": {
                     "get": {
                         "operationId": ("enumParameter"),
                         "parameters": [
                             {
                                 "in": ("query"),
                                 "name": ("sort_by"),
                                 "required": (false),
                                 "schema":  {
                                     "$ref": ("#/components/schemas/SortBy"),
                                 },
                             },
                         ],
                         "responses": {
                             "200": {
                                 "content": {
                                     "text/plain; charset=utf-8": {
                                         "schema": {
                                             "type": ("string"),
                                         },
                                     },
                                 },
                                 "description": (""),
                             },
                             "400": {
                                 "content": {
                                     "text/plain; charset=utf-8": {
                                         "schema": {
                                             "type": ("string"),
                                         },
                                     },
                                 },
                                 "description": (""),
                             },
                         },
                     },
                 },
                "/opt_vec_enum": {
                    "get": {
                        "operationId": ("optVecEnum"),
                        "parameters": [
                            {
                                "in": ("query"),
                                "name": ("status"),
                                "required": (false),
                                "schema": {
                                    "items": {
                                        "$ref": ("#/components/schemas/Status"),
                                    },
                                    "type": [
                                        "array",
                                        "null",
                                    ],
                                },
                            },
                        ],
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": ("string"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "400": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": ("string"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                        },
                    },
                },
                "/vec_enum": {
                    "get": {
                        "operationId": ("vecEnum"),
                        "parameters": [
                            {
                                "in": ("query"),
                                "name": ("status"),
                                "required": (true),
                                "schema": {
                                    "items": {
                                        "$ref": ("#/components/schemas/Status"),
                                    },
                                    "type": ("array"),
                                },
                            },
                        ],
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": ("string"),
                                        },
                                    },
                                },
                                "description": (""),
                            },
                            "400": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": ("string"),
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
    );
}
