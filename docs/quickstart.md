# Quickstart

This walkthrough builds a minimal JSON API with Groom. It has a single `GET /hello` route, a validated `GroomRouter`, and both outputs of that router: an `axum::Router` you can serve, and an OpenAPI spec you can hand to frontend tooling. You need a recent stable Rust toolchain. There is no install step: groom is a library you add to your `Cargo.toml` like any other dependency.

## Quick example

```rust
use groom::router::GroomRouterValid;
use groom_macros::Controller;
use utoipa::OpenApi;

#[Controller()]
mod api {
    use axum::{extract::Query, response::IntoResponse};
    use groom::{extract::GroomExtractor, response::Response};
    use groom_macros::{DTO, Response};

    #[Route(method = "get", path = "/hello")]
    pub async fn greet(Query(p): Query<GreetParams>) -> HelloResponse {
        let name = p.name.unwrap_or_else(|| "world".into());
        if name.is_empty() {
            HelloResponse::BadRequest(ErrorMessage {
                error: "`name` must be omitted or non-empty",
            })
        } else {
            HelloResponse::Hello(GreetMessage {
                message: format!("Hello, {name}!"),
            })
        }
    }

    #[DTO(parameters)]
    pub struct GreetParams {
        name: Option<String>,
    }

    #[Response(format(json))]
    pub enum HelloResponse {
        #[Response(code = 200)]
        Hello(GreetMessage),

        #[Response(code = 400)]
        BadRequest(ErrorMessage),
    }

    #[DTO(response)]
    pub struct GreetMessage {
        message: String,
    }

    #[DTO(response)]
    pub struct ErrorMessage {
        error: &'static str,
    }
}

fn make_router() -> GroomRouterValid {
    api::into_router()
        .validate()
        .expect("GroomRouter validation failed for quick-example")
}

pub fn make_axum_router() -> axum::Router {
    make_router().to_axum_router()
}

fn make_openapi(r: &GroomRouterValid) -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(info(title = "My API", version = "0.1.0"))]
    struct ApiDoc;

    r.to_openapi(ApiDoc::openapi())
}
```

## Step by step

### Dependencies

Add `groom` and `groom_macros` (the framework), `axum` (the runtime), and `utoipa` (OpenAPI generation) to your `Cargo.toml`. See the [quick-example Cargo.toml](../examples/quick-example/Cargo.toml) for exact versions. For array query parameters or URL-encoded bodies with repeated keys, enable the optional `axum-extra-query` and `axum-extra-form` features. See the [user guide](user-guide.md).

### The controller module

`#[Controller()]` turns `mod api` into a Groom controller. The module keeps one API surface in one place: the `#[Route]` handlers, the parameter struct, and the response types. Groom generates an `into_router()` function from the module. It is the entry point for everything that follows.

### A route handler

`#[Route(method = "get", path = "/hello")]` marks `greet` as an HTTP endpoint. The handler runs for `GET /hello`. Its parameter is an axum `Query` extractor over `GreetParams`. Groom knows this type, so the query string is deserialized and documented. The handler returns the `HelloResponse` enum: `Hello` carries the 200 payload, `BadRequest` the 400 payload. The handler is plain async Rust: it inspects the parsed name and picks a variant. Groom handles serialization and status codes.

### Parameters

`#[DTO(parameters)]` on `GreetParams` declares the struct as a query-parameter schema. The optional `name` field becomes a `?name=...` query parameter in OpenAPI. Groom derives `Deserialize` and the utoipa `IntoParams` wiring. One type drives both runtime parsing and the documented contract.

### Responses

`#[Response(format(json))]` declares the enum as the response contract. Every variant is one HTTP response. `#[Response(code = 200)]` and `#[Response(code = 400)]` set the status codes. Each variant carries a `#[DTO(response)]` payload type; its fields become the JSON body. The generated `Response` impl serializes each variant and emits the right status.

### Router + validation

`api::into_router()` returns an unvalidated `GroomRouter`. `.validate()` checks the composed routes for shadowing, where two controllers register the same method + path. It returns the typed `GroomRouterValid` alias on success. `.expect(...)` unwraps the validation result; a library would propagate the error instead. Validation must pass before the terminal conversions below are available. Groom enforces this at compile time through its typestate. See [api-reference.md](api-reference.md).

### Outputs

One validated router produces both artifacts. `make_axum_router` converts it with `.to_axum_router()` and serves it with `axum::serve` as an ordinary axum application. `make_openapi` feeds a utoipa `ApiDoc` (the base metadata: title, version, security, tags) into `.to_openapi(...)`. Groom merges its paths and schemas into that document:

```rust
#[derive(OpenApi)]
#[openapi(info(title = "My API", version = "0.1.0"))]
struct ApiDoc;

r.to_openapi(ApiDoc::openapi())
```

## Running it

The [quick-example](../examples/quick-example) crate is a buildable version of exactly this snippet, with a server wrapper and tests:

```sh
cargo run -p groom-example_quick-example --bin quick-example
```

The server listens on `http://127.0.0.1:8889`. Try the endpoint:

```sh
curl 'http://127.0.0.1:8889/hello?name=world'
# {"message":"Hello, world!"}

curl 'http://127.0.0.1:8889/hello'
# {"message":"Hello, world!"}
```

## Next steps

- [user-guide.md](user-guide.md) — all five annotations in depth: `#[Controller]`, `#[Route]`, `#[DTO]`, `#[RequestBody]`, `#[Response]`, plus content negotiation and examples.
- [api-reference.md](api-reference.md) — `GroomRouter` composition and typestate, `OpenApiSpecLayer`, errors.
- [architecture.md](architecture.md) — how groom, axum, and utoipa fit together.
- [examples](../examples/) — bigger applications: hello-world, composition, htmx, auth-middleware, and the full-stack todo app.
