# Changelog

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
