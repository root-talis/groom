# Groom

Groom connects user-defined controllers to an [axum](https://github.com/tokio-rs/axum) router and a [utoipa](https://github.com/juhaku/utoipa) OpenAPI spec. Groom is not a framework. You write handler functions and data types in Rust. Groom wires them into axum routes and adds their paths and schemas to a utoipa `OpenApiBuilder`.

Groom is inspired by [poem-openapi](https://github.com/poem-web/poem/blob/3bd9ee79e94b3f8a088a21e16648e7be6eed471c/poem-openapi-derive/src/api.rs).

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

A handler can also return `Result<R, E>`, where both `R` and `E` are `#[Response()]` types.

Each `#[Controller]` module generates a primary public function:

- `into_router() -> GroomRouter<S>` — returns a `GroomRouter` for composition (`.merge()`/`.nest()`), validation (`.validate()`), and terminal conversion (`.to_axum_router()`/`.to_openapi()`).

Import `groom::extract::GroomExtractor` and `groom::response::Response` inside the controller module so extractors and response types appear in the OpenAPI spec.

## Features

Groom adds a statically-typed HTTP layer on top of axum and utoipa:

- **Compile-time-typed handlers** — handler signatures describe request inputs and response outputs at compile time; generated wrappers handle serialization, so handlers return domain types instead of manually built `Response` bodies.
- **Automatic content negotiation** — when a response type declares multiple formats (for example `json` and `html`), Groom selects the matching serializer from the client's `Accept` header.
- **OpenAPI derived from the same types that drive routing** — the spec comes from the types handlers use, so code and documentation stay in sync.
- **Supplements axum rather than replacing it** — middleware, `Extension`, `State`, and non-Groom routes remain ordinary axum.
- **One handler backs a JSON API and HTML views** — useful for status pages, admin dashboards, or embedding reports without maintaining separate endpoints.

## Documentation

- [Quickstart](docs/quickstart.md) — get a Groom API running in minutes
- [User guide](docs/user-guide.md) — annotations and how-tos
- [API reference](docs/api-reference.md) — GroomRouter, OpenApiSpecLayer, content negotiation
- [Architecture](docs/architecture.md) — runtime + codegen internals

## Examples

| Example | Path | Purpose |
|---------|------|---------|
| Quick example | [examples/quick-example](examples/quick-example) | JSON greet endpoint from this guide; snippet kept in sync with `src/quickstart_snippet.rs`. |
| Hello world | [examples/hello-world](examples/hello-world) | Single controller, plain-text responses, inline spec route. |
| HTMX app | [examples/htmx](examples/htmx) | Simple backend with HTMX, rendered with the minijinja templating engine. |
| Todo app | [examples/todo](examples/todo) | Layered backend, multiple endpoints, spec binary, Vue frontend with generated client. |

Run the todo backend (with spec endpoint and CORS for local frontend):

```sh
cd examples/todo && just run-backend
```

## Further work

[List of things to do](TODO.md).

## Licensing

[MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
