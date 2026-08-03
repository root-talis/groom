# Changelog

## Unreleased

### groom

- **Breaking:** `Response::__groom_negotiate_content_type` now returns `Option<&'static Mime>` instead of owned `Mime`. Success paths borrow the type's supported-mime const; there is no success-path Mime clone. Update hand-written `Response` impls to match.
- `Result<T, E>` content negotiation now uses only the `Ok` type (`T`). When Accept cannot be satisfied, groom no longer builds a 406 via `T` and then retries `E`. Hand-written `Response` impls used in a `Result` must still declare identical format lists on both arms (enforced at router build); there is no dual-negotiate fallback for mismatched lists.
- **Breaking:** `ComponentsRegistry::merge` now returns `Result<Self, SchemaMergeError>` instead of `(String, Schema, Schema)`. The error boxes both schemas so the success `Result` stays small. `GroomRouter::merge` / `nest` still map conflicts to name-only `MergeError::SchemaConflict`.
- Form body `Content-Type` values with a charset (or other Mime parameters) are now accepted; detection matches type and subtype only, like JSON.
- Major architecture change: `GroomRouter` introduced as the central composition type.
- Added typestate pattern: `GroomRouter<S, NotValidated>` → `.validate()` → `GroomRouter<S, Validated>`.
- `GroomRouter::new()` — creates an empty router with empty registry and no OpenAPI paths.
- `GroomRouter::merge()` / `GroomRouter::nest()` — compose controllers; return `MergeResult<Self>` with `MergeError::SchemaConflict` on schema name collisions.
- `GroomRouter::validate()` — detects route shadowing (`RouterValidationError::RouteShadow`) across merged controllers.
- `GroomRouter::to_axum_router(self)` — terminal: extracts the inner `axum::Router<S>`.
- `GroomRouter::to_openapi(&self, api: OpenApi)` — terminal: merges accumulated paths and components into an `OpenApi` document.
- `GroomRouter::layer()` / `.fallback()` / `.route_layer()` — delegate transparently to the inner axum router.
- Added `prepend_path()` helper for OpenAPI path prefixing under `.nest()`.
- `RouterValidationError` extracted from `MergeError` as a separate error type with `RouteShadow { path, method }` variant.
- `MergeError` now carries only `SchemaConflict` and `SchemaNotFound` variants.
- `GroomRouter::from_router()` removed — controller composition goes through `into_router()` / `.merge()`.
- **Breaking:** `Result` response types now require both variants to declare the same list of formats. The router rejects a mismatch at build time with `"both variants must support the same list of formats"` instead of failing per-request.
- Added `Response::__groom_check_response_formats` and `HTTPFormatsSet`: `into_router()` now validates content-type format lists alongside HTTP status codes.

### groom_macros

- `#[Controller]` modules now generate `into_router() -> GroomRouter<S, NotValidated>` as the primary generated function, replacing the separate `merge_into_router()` / `merge_into_openapi_builder()` pattern.
- `merge_into_router()` retained as a soft-deprecated backward-compat function.
- **Breaking:** Generated `merge_into_router` now returns `Result<GroomRouter<S>, MergeError>` instead of panicking on schema conflict; callers must handle the result. Prefer `into_router()` for the non-fallible single-controller path.

## v0.2.2

### groom

- Added optional `axum-extra` dependency with feature flags `axum-extra-query` and `axum-extra-form`.
- Implemented `GroomExtractor` for `axum_extra::extract::Query<T>` behind the `axum-extra-query` feature. Handlers can use Axum Extra's `Query` extractor for repeated query parameters — fields such as `Vec<T>` and `Option<Vec<T>>` deserialize from `?status=New&status=Closed`.

### groom_macros

- Added `axum-extra-form` feature (enables `groom/axum-extra-form`). Url-encoded `#[RequestBody]` types now use `axum_extra::extract::Form` instead of `axum::extract::Form` when the feature is on, enabling repeated form fields for `Vec<T>` and `Option<Vec<T>>` (e.g. `status=New&status=Closed`).

## v0.2.1

### groom

- Changed `Response::__groom_check_response_codes` to take `&str` instead of `&String`; macro-generated `#[Response]` implementations were updated accordingly.
- Added `Default` for `HTTPCodeSet`.
- Replaced `Into<RefOr<Schema>>` on `ComponentEntry` with `From<ComponentEntry> for RefOr<Schema>`.
- Clippy-driven simplifications in content-type parsing; behavior unchanged.

### groom_macros

- Renamed internal macro-argument parser from `parse_nested_meta!` to `extract_macro_arguments!`.
- Removed unused OpenAPI components-setup path in `#[Controller]` generation and dead schema fragments in `#[RequestBody]` struct impls.
- Renamed internal `#[Controller]` helpers; generated code is unchanged.
- Aligned path dependency on `groom` to 0.2.1.

## v0.2.0

### groom

- Added `ComponentsRegistry` for deduplicating and merging OpenAPI component schemas. Request/response bodies and parameter types are registered under `#/components/schemas` and referenced with `$ref`.
- Added JSON pointer helpers for building schema references.
- Consolidated path and query parameter handling into `extract/parameters.rs` with shared `GroomExtractor` implementations for `Path<T>` and `Query<T>`.
- Added `runtime_checks::HTTPCodeSet` with runtime validation that `Result<T, E>` response variants on a handler use distinct HTTP status codes.
- Implemented `Response` for `Result<T, E>` when both `T` and `E` implement `Response`.
- `String` types are inlined in OpenAPI schemas instead of being added to `#/components/schemas`.
- Updated dependencies: axum 0.7 → 0.8.9, utoipa 4.2 → 5.4, derive_more 0.99 → 2.1, darling 0.20 → 0.23, thiserror 1 → 2, strum 0.25 → 0.28.

### groom_macros

- `#[Response]` now supports structs in addition to enums.
- `#[DTO]` now supports enums and a `parameters` role for path/query DTOs, including enum-typed parameter fields.
- Handler return type `Result<T, E>` is supported when both arms are `#[Response]` types.
- OpenAPI `operationId` is generated from the handler function name (camelCase).
- Request and response body schemas are registered through the components registry.
- Fixed merging of OpenAPI paths when multiple controllers contribute to the same spec.
- `#[Response]` generates formatter code only for declared content-types, not every supported format.
- Fixed non-deterministic MIME type ordering in multi-format responses (HashMap → BTreeMap).
- `#[Controller]` modules may contain synchronous helper functions that are not route handlers.
- Internal refactoring of `#[Controller]`, `#[RequestBody]`, `#[Response]`, and `#[DTO]` macro implementations; generated API surface is unchanged.
