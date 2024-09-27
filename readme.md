# kix

A result type for testing that supports to eliminate using of `Result::unwrap`.
So a library may enforce flags such as `-D clippy::unwrap_used` without hassle.

## Quickstart

For a quickstart, see [examples](examples). For installation from package
repository, see [crates.io](https://crates.io/crates/kix). For API
references, see [doc.rs](https://docs.rs/kix).

## Test

```bash
# Just install for the first time.
rustup component add clippy

cargo test
cargo clippy
```
