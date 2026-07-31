# groom

A thin wrapper around [axum](https://github.com/tokio-rs/axum) for code-first, strictly-typed HTTP APIs.

Groom is inspired by [poem-openapi](https://github.com/poem-web/poem/blob/3bd9ee79e94b3f8a088a21e16648e7be6eed471c/poem-openapi-derive/src/api.rs).

Start with the [Groom README](https://github.com/root-talis/groom/blob/main/README.md).

## Documentation

- [Quickstart](docs/quickstart.md) — get a Groom API running in minutes
- [User guide](../docs/user-guide.md) — annotations and how-tos
- [API reference](../docs/api-reference.md) — GroomRouter, OpenApiSpecLayer, content negotiation
- [Architecture](../docs/architecture.md) — runtime + codegen internals

## Cargo features

- `axum-extra-query` — OpenAPI wiring for `axum_extra::extract::Query` (repeated query keys → `Vec` fields); see [Array query parameters](../docs/user-guide.md#array-query-parameters).
- `axum-extra-form` — optional `axum-extra` (`form`) dependency, enabled via `groom_macros` feature forwarding; see [Array fields in URL-encoded bodies](../docs/user-guide.md#array-fields-in-url-encoded-bodies).

## Licensing

[MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
