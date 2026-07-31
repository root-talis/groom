use std::collections::HashMap;

use axum::{Router, extract::Request, http::StatusCode, middleware::{Next, from_fn}, response::Response};
use groom::router::GroomRouterValid;
use serde_json::json;

use crate::features::test_utils::{Req, assert_openapi_doc};

/// Spec layer that adds Bearer auth security scheme to the OpenAPI spec.
#[derive(Clone)]
struct BearerAuthSpecLayer;

impl groom::router::OpenApiSpecLayer for BearerAuthSpecLayer {
    fn modify_openapi(&self, api: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

        let scheme = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );

        let components = api.components.get_or_insert_with(utoipa::openapi::Components::new);
        components
            .security_schemes
            .insert("bearerAuth".to_string(), scheme);
    }

    fn modify_operation(
        &self,
        _path: &str,
        _method: &utoipa::openapi::path::HttpMethod,
        operation: &mut utoipa::openapi::path::Operation,
    ) {
        use utoipa::openapi::security::SecurityRequirement;
        operation.security = Some(vec![
            SecurityRequirement::new("bearerAuth", [] as [&str; 0])
        ]);
    }

    fn mount<S>(&self, r: axum::Router<S>) -> Router<S> where S: Clone + Send + Sync + 'static {
        r.layer(from_fn(Self::check_auth))
    }
}

impl BearerAuthSpecLayer {
    const TOKEN: &'static str = "secret-token";

    async fn check_auth(req: Request, next: Next) -> Response {
        let is_authorized = req
            .headers()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| {
                if let Some(token) = v.strip_prefix("Bearer ") {
                    token == Self::TOKEN
                } else {
                    false
                }
            });

        if is_authorized {
            next.run(req).await
        } else {
            use axum::response::IntoResponse;
            (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
        }
    }
}

mod controllers {
    use groom_macros::Controller;

    #[Controller()]
    pub mod public_controller {
        use axum::response::IntoResponse;
        use groom::response::Response;
        use groom_macros::Response;

        #[Response(format(plain_text))]
        pub enum HelloResponse {
            #[Response()]
            Ok(String),
        }

        #[Route(method = "get", path = "/public/hello")]
        async fn hello() -> HelloResponse {
            HelloResponse::Ok("Hello, public!".into())
        }
    }

    #[Controller()]
    pub mod private_controller {
        use axum::response::IntoResponse;
        use groom::response::Response;
        use groom_macros::Response;

        #[Response(format(plain_text))]
        pub enum HelloResponse {
            #[Response()]
            Ok(String),
        }

        #[Route(method = "get", path = "/private/hello")]
        async fn hello() -> HelloResponse {
            HelloResponse::Ok("Hello, private!".into())
        }
    }
}

fn build_merged_controller() -> GroomRouterValid {
    controllers::public_controller::into_router()
        .merge(
            controllers::private_controller::into_router()
                .layer_with_spec(BearerAuthSpecLayer),
        )
        .expect("merge failed")
        .validate()
        .expect("validation failed")
}

/// Verify routing works when merging two controllers.
#[tokio::test]
async fn test_merge_controllers() {
    let router = build_merged_controller().to_axum_router();

    Req::get("/public/hello").call(&router).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("Hello, public!");

    Req::get("/private/hello")
        .with_headers(HashMap::from([("authorization", "Bearer secret-token")]))
        .call(&router).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("Hello, private!");

    Req::get("/private/hello")
        .call(&router).await
        .assert_status(401)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("Unauthorized");
}

/// Verify OpenAPI spec includes both merged controllers plus BearerAuth security scheme.
#[test]
fn test_merge_controllers_openapi() {
    assert_openapi_doc(
        |api| {
            build_merged_controller().to_openapi(api)
        },
        json!({
            "components": {
                "securitySchemes": {
                    "bearerAuth": {
                        "bearerFormat": "JWT",
                        "scheme": "bearer",
                        "type": "http"
                    }
                }
            },
            "info": {
                "contact": {
                    "email": "mail@example.com",
                    "name": "name"
                },
                "description": "d",
                "license": {
                    "name": "n"
                },
                "title": "t",
                "version": "0.0.0"
            },
            "openapi": "3.1.0",
            "paths": {
                "/private/hello": {
                    "get": {
                        "operationId": "hello",
                        "security": [{ "bearerAuth": [] }],
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": ""
                            },
                            "406": {
                                "description": "The requested content type is not supported",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/public/hello": {
                    "get": {
                        "operationId": "hello",
                        // important: no security here
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": ""
                            },
                            "406": {
                                "description": "The requested content type is not supported",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    );
}


fn build_nested_controller() -> GroomRouterValid {
    let private_router = controllers::private_controller::into_router()
        .layer_with_spec(BearerAuthSpecLayer);

    controllers::public_controller::into_router()
        .nest("/api", private_router)
        .expect("nesting failed")
        .validate()
        .expect("validation failed")
}

/// Verify routing works when nesting private_controller under /api prefix.
#[tokio::test]
async fn test_nest_controllers() {
    let router = build_nested_controller().to_axum_router();

    Req::get("/public/hello").call(&router).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("Hello, public!");

    Req::get("/api/private/hello")
        .with_headers(HashMap::from([("authorization", "Bearer secret-token")]))
        .call(&router).await
        .assert_status(200)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("Hello, private!");

    Req::get("/api/private/hello")
        .call(&router).await
        .assert_status(401)
        .assert_content_type("text/plain; charset=utf-8")
        .assert_body("Unauthorized");
}

/// Verify OpenAPI spec includes nested controller with /api prefix plus BearerAuth security scheme.
#[test]
fn test_nest_controllers_openapi() {
    assert_openapi_doc(
        |api| {
            build_nested_controller().to_openapi(api)
        },
        json!({
            "components": {
                "securitySchemes": {
                    "bearerAuth": {
                        "bearerFormat": "JWT",
                        "scheme": "bearer",
                        "type": "http"
                    }
                }
            },
            "info": {
                "contact": {
                    "email": "mail@example.com",
                    "name": "name"
                },
                "description": "d",
                "license": {
                    "name": "n"
                },
                "title": "t",
                "version": "0.0.0"
            },
            "openapi": "3.1.0",
            "paths": {
                "/api/private/hello": {
                    "get": {
                        "operationId": "hello",
                        "security": [{ "bearerAuth": [] }],
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": ""
                            },
                            "406": {
                                "description": "The requested content type is not supported",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/public/hello": {
                    "get": {
                        "operationId": "hello",
                        // important: no security here
                        "responses": {
                            "200": {
                                "content": {
                                    "text/plain; charset=utf-8": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                },
                                "description": ""
                            },
                            "406": {
                                "description": "The requested content type is not supported",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    );
}
