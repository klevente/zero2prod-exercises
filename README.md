# Zero to Production in Rust

## Required Dependencies

- `cargo-watch`: `cargo install cargo-watch` (`cargo watch -x check -x test -x run`)
- `cargo-audit`: `cargo install cargo-audit`
- `sqlx-cli`: `cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls postgres`
- `cargo-udeps`: `cargo install cargo-udeps` (`cargo +nightly udeps`)
- `bunyan`: `cargo install bunyan` (`TEST_LOG=true cargo test <test> | bunyan`)