# groom

A thin wrapper around [axum](https://github.com/tokio-rs/axum) for making code-first strictly-typed HTTP APIs.

Groom is heavily inspired by [poem-openapi](https://github.com/poem-web/poem/blob/3bd9ee79e94b3f8a088a21e16648e7be6eed471c/poem-openapi-derive/src/api.rs).

Groom [README.md](https://github.com/root-talis/groom/blob/main/README.md) is a good starting point.

For how this crate's modules, traits, and runtime behavior fit together, see [ARCHITECTURE.md](ARCHITECTURE.md). Proc-macro code generation is documented in [groom_macros/ARCHITECTURE.md](../groom_macros/ARCHITECTURE.md).

Optional Cargo feature `axum-extra-query` wires OpenAPI for `axum_extra::extract::Query` so handlers can accept repeated query keys as `Vec` fields — see the root [README.md](../README.md#array-query-parameters).

## Goals:
  - leverage rust's type system to describe and enforce API contracts;
  - abstract out content-type negotiations and serialization/deserialization and allow developer to work with raw data;
  - allow code-first OpenAPI spec generation;
  - be a supplement to axum, not a replacement;
  - check everything at compile-time with developer-friendly error messages - wherever possible.

# ❗ groom is WIP - do not use in production!

[List of things to do](TODO.md).

## Licensing:
[MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
