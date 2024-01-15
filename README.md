# Zero to Production in Rust

## Required Dependencies

- `cargo-watch`: `cargo install cargo-watch` (`cargo watch -x check -x test -x run`)
- `cargo-audit`: `cargo install cargo-audit`
- `sqlx-cli`: `cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls postgres`
- `cargo-udeps`: `cargo install cargo-udeps` (`cargo +nightly udeps`)
- `bunyan`: `cargo install bunyan` (`TEST_LOG=true cargo test <test> | bunyan`)

## Increasing Open File Descriptors

On Linux, the integration tests can exhaust the maximum available file descriptors, since all test cases run in a single binary. To increase this, run `ulimit -n <N>` (ex. `N = 10000` - `ulimit -n 10000`)

## TODO

Improve the subscription flow by thinking about the topics below:
- What happens if a user tries to subscribe twice? Make sure that they receive two confirmation emails;
- What happens if a user clicks on a confirmation link twice?
- What happens if the subscription token is well-formatted but non-existent?
- Add validation on the incoming token, we are currently passing the raw user input straight into a query (thanks sqlx for protecting us from SQL injections <3);
- Use a proper templating solution for our emails (e.g. tera);
- Anything that comes to your mind!

## Error Handling

Errors should be logged when they are handled, so if the error is being propagated upstream via `?` or `return Err(...)`, the error should not be logged - the only reasonable operation would be to add additional context to it via these propagating layers.