# Quick example

This crate is the exact code from the [Quick example](../../QUICKSTART.md#quick-example) section of the Groom quickstart guide (`QUICKSTART.md`). The snippet lives in `src/quickstart_snippet.rs`; `src/lib.rs` includes it and adds a small runnable server wrapper plus tests.

```sh
cargo test -p groom-example_quick-example
cargo run -p groom-example_quick-example --bin quick-example
```

Try `GET http://127.0.0.1:8889/hello` and `GET http://127.0.0.1:8889/hello?name=Groom`.
