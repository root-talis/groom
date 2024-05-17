# groom

A thin wrapper around [actix](https://actix.rs/) for making code-first strictly-typed HTTP APIs.

Groom is heavily inspired by [poem-openapi](https://github.com/poem-web/poem/blob/3bd9ee79e94b3f8a088a21e16648e7be6eed471c/poem-openapi-derive/src/api.rs).

## Goals:
  - leverage rust's type system to describe and enforce API contracts;
  - abstract out content-type negotiations and serialization/deserialization and allow developer to work with raw data;
  - allow code-first OpenAPI spec generation;
  - be a supplement to actix, not a replacement.

# ❗ groom is WIP - do not use in production!
