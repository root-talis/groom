# User guide

This is the how-to manual for writing a Groom API. It covers the five annotations — `#[Controller]`, `#[Route]`, `#[DTO]`, `#[RequestBody]`, and `#[Response]` — plus content negotiation, supporting traits and macros, and the example crates. To start fast, read the [quickstart](quickstart.md). For exact signatures and the `GroomRouter` / `OpenApiSpecLayer` reference, see the [API reference](api-reference.md).

## Controllers

A controller is a Rust module that keeps the handlers and types of one API surface in a single place. Mark it with the module-level `#[Controller]` attribute. Inside the module you write `#[Route]` handler functions plus the DTO, request-body, and response types they use. The attribute applies to the whole module:

```rust
#[Controller()]
mod api {
    use axum::{extract::Query, response::IntoResponse};
    use groom::{extract::GroomExtractor, response::Response};
    use groom_macros::{DTO, Response};

    #[Route(method = "get", path = "/hello")]
    pub async fn greet(Query(p): Query<GreetParams>) -> HelloResponse {
        // ...
    }

    // #[DTO] / #[RequestBody] / #[Response] types live here next to their handlers.
}
```

The macro reference table lives in [api-reference.md](api-reference.md). This guide focuses on usage. Request bodies, query parameters, DTOs, and responses are covered below.

The `#[Controller]` attribute is applied to a module containing route handlers and their supporting types.

| Option | Description |
|--------|-------------|
| `state_type = T` | Router state type (`S` in `Router<S>`). Defaults to `()`. When set, import `T` inside the module (required for macro expansion). |

Generated API:

- `into_router() -> GroomRouter<S>` — returns a `GroomRouter` for composition (`.merge()`/`.nest()`), validation (`.validate()`), and terminal conversion (`.to_axum_router()`/`.to_openapi()`)

Handlers annotated with `#[Route]` are wrapped to parse the `Accept` header and dispatch to `Response::__groom_into_response`. The original handler stays a plain `async fn` returning a data structure.

What belongs in a controller module? Everything that describes one API contract: the route handlers, the parameter structs they accept, and the request/response types they exchange. Keep infrastructure out. Middleware, `State`, `Extension`, and non-Groom routes stay ordinary axum, composed at the router level. The `state_type` argument connects the controller to the axum state you serve with. Set it to your application state type, import it inside the module, and every generated route can access it via axum's `State<T>` extractor.

### Composition

A `#[Controller]` module generates one primary entry point: `into_router() -> GroomRouter<S, NotValidated>`. Multiple controllers compose on the resulting router via `.merge()` and `.nest()`. Then `.validate()` produces the validated router, from which you call `.to_axum_router()` or `.to_openapi()`. For method signatures, see [api-reference.md](api-reference.md).

### State

When your application has shared state — a database pool, a config, a service layer — pass it to the controller with `state_type`:

```rust
#[Controller(state_type = AppState)]
mod api {
    use axum::extract::State;
    // import `AppState` inside the module (required for macro expansion)

    #[Route(method = "get", path = "/health")]
    pub async fn health(State(state): State<AppState>) -> HealthResponse {
        // ...
    }
}
```

With no `state_type` argument, the router state defaults to `()`. `state_type` sets the `S` in `Router<S>`. Handlers access it through axum's `State<T>` extractor, like any other axum application. The generated `into_router()` returns `GroomRouter<AppState, NotValidated>`.

## Routes

Route handlers are `async fn`s inside a `#[Controller]` module, each marked with the `#[Route]` helper attribute. The attribute wires the handler into the generated router and into the OpenAPI path collection.

A handler takes axum extractors and groom extractors as parameters — `Query<T>` and `Path<T>` from axum, request bodies from groom (`#[RequestBody]` types) — and returns a `#[Response]` type, or `Result<R, E>` where both sides are `#[Response]` types.

The `#[Route]` attribute marks a handler as an HTTP endpoint.

| Attribute | Description |
|-----------|-------------|
| `method` | HTTP method: `get`, `post`, `put`, `delete`, `patch`. |
| `path` | Route template with `{param}` placeholders (OpenAPI/axum style). |

Doc comments on the handler become OpenAPI `summary` and `description`.

Handler parameters use standard axum extractors (`Query`, `Path`, `Extension`, `State`, `HeaderMap`, `Request`, `String`, `Bytes`, …) plus Groom `RequestBody` types. Import `GroomExtractor` when you use extractors that contribute to the OpenAPI operation.

```rust
#[Route(method = "get", path = "/hello")]
pub async fn greet(Query(p): Query<GreetParams>) -> HelloResponse {
    // `GreetParams` is defined with #[DTO(parameters)], `HelloResponse` with #[Response].
    // Both are covered below.
    todo!()
}
```

Path segments follow axum's `{param}` template syntax and are extracted with axum's `Path<T>` extractor:

```rust
#[Route(method = "get", path = "/tasks/{id}")]
pub async fn get_task(Path(id): Path<u64>) -> TaskResponse {
    // ...
}
```

Handler bodies are ordinary async Rust. Validation, service calls, and error mapping all happen inside the function. The function only returns the response type. Groom handles serialization, status codes, and content negotiation from there.

One handler, one endpoint: the `method` + `path` pair is the route's identity. Two controllers that register the same `method` + `path` form a route shadow. `.validate()` detects this at composition time (see [api-reference.md](api-reference.md)).

## DTOs

A DTO (Data Transfer Object) is the schema half of your API: the shape of a request body, a query parameter struct, or a response payload. Marking a struct or enum with `#[DTO]` derives serialization and OpenAPI schema generation from the type itself, so the contract cannot drift from the code. At least one of the `request` / `response` / `parameters` flags is required; combine them freely (`#[DTO(request, response)]`).

The `#[DTO]` attribute marks a struct or enum as a Data Transfer Object and generates `utoipa::ToSchema` plus serde derives as appropriate.

| Argument | Effect |
|----------|--------|
| `request` | `Deserialize`, `DTO_Request` |
| `response` | `Serialize`, `DTO_Response` |
| `parameters` | `Deserialize` (for query/path parameter structs) |

Combine arguments: `#[DTO(request, response)]`, `#[DTO(parameters)]`, etc. At least one argument is required.

A single type often plays both roles — the same `Task` struct is a valid request body and a valid response payload:

```rust
#[DTO(request, response)]
pub struct Task {
    title: String,
    done: bool,
}
```

Generated derives: serde (`Serialize` / `Deserialize`, depending on the flags), `utoipa::ToSchema` on every DTO, and `utoipa::IntoParams` on `parameters` DTOs, so query/path structs appear in the OpenAPI operations that use them. The marker traits `DTO`, `DTO_Request`, and `DTO_Response` let generic code bound on a type's role in the API.

Use `#[DTO(parameters)]` with `Query<T>` or `Path<T>`. Field doc comments and `serde` attributes (`rename`, `default`, …) appear in the schema. For `Path<T>`, the parameter matches the route's `{param}` placeholder. For `Query<T>`, each field becomes a query parameter.

Enums with unit, tuple, or struct variants are supported as response DTOs. See `groom_tests/tests/features/value_objects.rs`.

#### Array query parameters

Axum's built-in `Query<T>` does not deserialize repeated query keys (for example `?status=New&status=Closed`) into `Vec` fields. To do that, enable the optional `axum-extra-query` feature on `groom`, add `axum-extra` with its `query` feature, and use `axum_extra::extract::Query<T>` in the handler:

```toml
# Cargo.toml
groom = { version = "0.2", features = ["axum-extra-query"] }
axum-extra = { version = "0.12", features = ["query"] }
```

```rust
use axum_extra::extract::Query;

#[DTO(parameters)]
pub struct StatusFilter {
    status: Vec<Status>,
}

#[Route(method = "get", path = "/tasks")]
pub async fn list_tasks(Query(filters): Query<StatusFilter>) -> TaskListResponse {
    // GET /tasks?status=New&status=Closed
    todo!()
}
```

`Option<Vec<T>>` is supported too. Omitting the parameter yields `None`; repeating the key fills the vector. OpenAPI generation produces an `array` schema (or `array` + `null` for optional fields) from the same `#[DTO(parameters)]` struct. See `groom_tests/tests/features/request_query_params.rs` (`test_query_vec_of_enums`, `test_query_opt_vec_of_enums`).

## Request bodies

A request body extractor turns the HTTP request body into a typed struct. `#[RequestBody]` works on structs only. A named struct defines the body shape directly; a tuple struct wrapping a `#[DTO(request)]` type reuses the DTO's schema. It generates the `GroomExtractor` + `FromRequest` plumbing plus a `{Name}Rejection` enum for malformed input. Raw bodies are available through `String`, `Bytes`, or `groom::binary_request_body!` when the shape is not a struct.

In the handler, the body extractor appears as a plain parameter. Groom wires it into axum's extractor machinery, so the handler signature stays declarative:

```rust
#[Route(method = "post", path = "/people")]
pub async fn create_person(body: CreatePerson) -> PersonResponse {
    // `body` is already a validated `CreatePerson`
    todo!()
}
```

The `#[RequestBody]` attribute marks a struct as a request body extractor. It supports JSON and URL-encoded form data.

| Option | Description |
|--------|-------------|
| `format(json)` | Accept `application/json`. |
| `format(url_encoded)` | Accept `application/x-www-form-urlencoded`. |
| `format(json, url_encoded)` | Content negotiation on input (both formats). |

A named struct defines the body shape directly. A tuple struct wrapping a `#[DTO(request)]` type reuses the DTO schema:

```rust
#[DTO(request)]
pub struct Person { name: String, age: Option<u8> }

#[RequestBody(format(json, url_encoded))]
pub struct CreatePerson(Person);
```

Raw bodies: `String`, `Bytes`, or a type created with `groom::binary_request_body!`:

```rust
groom::binary_request_body!(ImageJpeg with content_type "image/jpeg");
```

#### Array fields in URL-encoded bodies

Axum's built-in `Form<T>` does not deserialize repeated form keys (for example `status=New&status=Closed`) into `Vec` fields. To do that, enable the optional `axum-extra-form` feature on `groom_macros` and add `axum-extra` with its `form` feature:

```toml
# Cargo.toml
groom_macros = { version = "0.2", features = ["axum-extra-form"] }
axum-extra = { version = "0.12", features = ["form"] }
```

The feature forwards to `groom/axum-extra-form` automatically. When you use proc-macros, you do **not** need to enable it separately on `groom`. Handler signatures stay the same; `#[RequestBody(format(url_encoded))]` switches the generated extractor to `axum_extra::extract::Form`:

```rust
#[RequestBody(format(url_encoded))]
pub struct StatusFilter {
    status: Vec<Status>,
}

#[Route(method = "post", path = "/tasks")]
pub async fn filter_tasks(body: StatusFilter) -> TaskListResponse {
    // POST with Content-Type: application/x-www-form-urlencoded
    // body: status=New&status=Closed
    todo!()
}
```

`Option<Vec<T>>` is supported too. An empty body yields `None`; repeating the key fills the vector. See `groom_tests/tests/features/request_body.rs` (`test_url_encoded_vec_of_enums`, `test_url_encoded_opt_vec_of_enums`).

## Responses

The `#[Response]` annotation describes how a handler return type maps to HTTP status codes and content types. On an enum, each variant is a distinct HTTP response. On a struct, the whole type is one response shape. Groom generates the `Response` impl (the `into_response_*` and `__groom_into_response` methods) and the OpenAPI response definitions from the type.

The `#[Response]` attribute describes how a handler return type maps to HTTP status codes and content types. It applies to an enum or struct.

**Enum (discriminated responses)** — each variant is a distinct HTTP response:

```rust
#[Response(format(json))]
pub enum TaskResponse {
    #[Response(code = 200)]
    Ok(TaskViewModel),

    #[Response(code = 404)]
    NotFound,

    #[Response(code = 500)]
    ServerError,
}
```

| Enum-level option | Description |
|-------------------|-------------|
| `format(json)` | JSON responses. |
| `format(plain_text)` | `text/plain; charset=utf-8`. |
| `format(html)` | `text/html; charset=utf-8`. |
| `format(json, html, plain_text)` | Multiple formats; client selects via `Accept`. |
| `default_format = "json"` | Format used when `Accept` is absent. Required when multiple formats are declared. |

| Variant-level option | Description |
|----------------------|-------------|
| `code = N` | HTTP status code. Defaults to `200` when omitted on a variant inside a typed enum. |

Variant doc comments become response descriptions in OpenAPI.

The generated `Response` impl routes each variant through `into_response_*` and `__groom_into_response`. Negotiation runs in the generated wrapper before the handler is invoked; `__groom_into_response` consumes the pre-negotiated format. The runtime checks the variant and the negotiated format, then produces the axum response. Because the status codes and formats are part of the type, every possible outcome of the handler appears in the generated OpenAPI spec.

**Struct (single response shape)** — one status code for the entire type:

```rust
#[Response(format(plain_text, html, json), default_format = "plain_text", code = 418)]
pub struct Health { pub is_alive: bool }
```

| Struct-level option | Description |
|---------------------|-------------|
| `code = N` | HTTP status code (default `200`). |
| `format(...)`, `default_format` | Same as for enums. |

JSON serialization uses serde. Plain-text responses use `From<T> for String` when defined.

**`Result` return type** — handlers can return `Result<Ok, Err>` instead of a response enum. The success type is a `#[Response]` struct (or enum variant). The error type is typically a `#[Response]` enum whose variants map to distinct HTTP status codes. Groom maps `Ok(...)` and `Err(...)` to the right responses and documents all outcomes in OpenAPI.

```rust
#[Response(format(json), code = 200)]
pub struct GreetOk {
    message: String,
}

#[DTO(response)]
pub struct GreetError {
    error: &'static str,
}

#[Response(format(json))]
pub enum GreetFailure {
    #[Response(code = 400)]
    BadRequest(GreetError),
}

#[Route(method = "get", path = "/hello")]
pub async fn greet(Query(p): Query<GreetParams>) -> Result<GreetOk, GreetFailure> {
    let name = p.name.unwrap_or_else(|| "world".into());
    if name.is_empty() {
        return Err(GreetFailure::BadRequest(GreetError {
            error: "`name` must be omitted or non-empty",
        }));
    }
    Ok(GreetOk {
        message: format!("Hello, {name}!"),
    })
}
```

**Formats must match.** The `Ok` and `Err` arms of a `Result` response must declare identical `format(...)` lists — both `format(json)`, or both `format(json, html)`. Mixing `format(json)` and `format(html)`, or pairing a formatted arm with an any-content arm, is a build-time error. The router panics at `into_router()` with `"Result<...>: both variants must support the same list of formats"` instead of failing per-request at runtime. Any-content arms (no `format(...)`) are legal only when both arms are any-content.

See `groom_tests/tests/features/response_type_result.rs` for a full example with multiple error status codes and OpenAPI assertions.

### HTML responses

HTML is a first-class response format alongside JSON and plain text. Typical use cases:

- **HTMX applications** — well-typed controllers that are easy to set up.
- **Human-readable status or health pages** — operators hit `/status` in a browser while monitoring tools call the same route with `Accept: application/json`.
- **Lightweight admin or debug UIs** — expose a read-only view of internal state without a separate frontend build.
- **Mixed clients on one contract** — declare `format(json, html)` on a response type so API consumers and browsers share handlers and OpenAPI paths.

**HTML-only endpoint** — return a `String` or a `#[DTO(response)]` struct with `format(html)`:

```rust
#[Response(format(html))]
pub enum StatusPageResponse {
    #[Response(code = 200)]
    Ok(StatusView),
}
```

**Rendering a struct as HTML** — implement `groom::html_format!` for types used in HTML (or multi-format) responses:

```rust
groom::html_format!(StatusView, self {
    format!("<p>status: <b>{}</b></p>", self.status)
});
```

The macro body is the right place to call a templating engine. For anything beyond trivial markup, prefer a crate such as [Askama](https://github.com/djc/askama), [Tera](https://github.com/Keats/tera), or [Minijinja](https://github.com/mitsuhiko/minijinja) over ad-hoc `format!` strings. Templates separate layout from data, support inheritance and partials, and handle escaping consistently. A handler still returns your domain type; `html_format!` renders the template:

```rust
groom::html_format!(StatusView, self {
    // Pseudocode: render a template with `self` as context
    status_template.render(self).unwrap()
});
```

When building HTML manually, escape any user-controlled values to avoid XSS (see `groom_tests/tests/features/response_type_html.rs`).

**Content negotiation with HTML** — combine formats and set a default for when `Accept` is missing:

```rust
#[Response(format(json, html), default_format = "json")]
pub enum StatusResponse {
    #[Response(code = 200)]
    Ok(StatusView),
}
```

See `groom_tests/tests/features/response_content_negotiation.rs` for full `Accept` header behavior.

## Content negotiation

When a response type declares multiple formats, groom negotiates the client's `Accept` header **once in the generated wrapper, before the handler runs**, and passes the negotiated mime to response conversion. `default_format` is used **only when `Accept` is absent** (required when multiple formats are declared). JSON detection accepts both `application/json` and `application/*+json` vendor suffixes (for example `application/vnd.api+json`). When the client's `Accept` header satisfies none of the declared formats, the server responds `406 Not Acceptable` with a `Vary: Accept` header and a body listing the supported content types; a malformed `Accept` yields `400` with `Invalid Accept header.`. Request bodies negotiate on input the same way. A `#[RequestBody(format(json))]` type accepts `application/json`; `format(url_encoded)` accepts `application/x-www-form-urlencoded`; `format(json, url_encoded)` accepts both via `Content-Type`. Unsupported **request** content types get a `400` plain-text response. The parsing functions (`parse_accept_header`, `parse_content_type_header`, `get_body_content_type`) are documented in [api-reference.md](api-reference.md).

## Supporting traits and macros

These items complete the picture when you combine groom with axum and utoipa:

| Item | Role |
|------|------|
| `groom::extract::GroomExtractor` | Extends axum extractors with OpenAPI metadata. |
| `groom::response::Response` | Converts return types to HTTP responses and OpenAPI response definitions. |
| `groom::binary_request_body!` | Newtype over `Bytes` with a custom request content type. |
| `groom::html_format!` | Defines HTML rendering for a type used in multi-format responses. |
| `utoipa::ToSchema` / `utoipa::PartialSchema` | Required on nested types referenced inside DTOs and responses. |

## Example crates

| Example | Path | Purpose |
|---------|------|---------|
| Quick example | [quick-example](../examples/quick-example) | JSON greet endpoint from the [quickstart](quickstart.md); snippet kept in sync with `quickstart_snippet.rs`. |
| Hello world | [hello-world](../examples/hello-world) | Single controller, plain-text responses, inline spec route. |
| HTMX app | [htmx](../examples/htmx) | Simple backend with HTMX, rendered with minijinja templating engine. |
| Auth middleware | [auth-middleware](../examples/auth-middleware) | Complete working example of an `OpenApiSpecLayer`-based middleware. |
| Todo app | [todo](../examples/todo) | Layered backend, multiple endpoints, spec binary, Vue frontend with generated client. |

## Feature tests as reference

The [groom_tests](../groom_tests/tests/features/) crate exercises individual features in isolation. Useful entry points:

| Test module | Topic |
|-------------|-------|
| `request_body` | `RequestBody`, raw bodies, `binary_request_body!`; `Vec` / `Option<Vec>` in url-encoded bodies via `axum-extra-form` |
| `request_query_params` | `#[DTO(parameters)]` with `Query`; `Vec` / `Option<Vec>` via `axum_extra::extract::Query` |
| `request_path_params` | Path parameters and enums in paths |
| `request_headers` | `HeaderMap` extractor |
| `request_methods` | All HTTP methods on one path |
| `request_axum_request_extractor` | Full `Request` extractor |
| `response_type_json` / `response_type_plaintext` / `response_type_html` | Single-format responses |
| `response_type_result` | `Result<Ok, Err>` handler return types |
| `response_struct` | Struct (non-enum) responses, `html_format!` |
| `response_content_negotiation` | Multi-format responses and `Accept` |
| `value_objects` | Algebraic types in response schemas |
| `dependency_injection` | `Extension` and `State` |
| `multiple_controllers` | Composing routers and OpenAPI builders |
