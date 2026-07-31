# groom_macros

Proc-macro crate for groom. It provides attribute macros that generate router and OpenAPI wiring at compile time.

## Documentation

- [Quickstart](docs/quickstart.md) — get a Groom API running in minutes
- [User guide](../docs/user-guide.md) — annotations and how-tos
- [API reference](../docs/api-reference.md) — GroomRouter, OpenApiSpecLayer, content negotiation
- [Architecture](../docs/architecture.md) — runtime + codegen internals

## Cargo features

Optional Cargo feature `axum-extra-form` switches `#[RequestBody(format(url_encoded))]` to `axum_extra::extract::Form` for repeated form keys → `Vec` fields. It forwards to `groom/axum-extra-form`; enable it here only — see [Array fields in URL-encoded bodies](../docs/user-guide.md#array-fields-in-url-encoded-bodies).

## Licensing

[MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
