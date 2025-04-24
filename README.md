# No-op executor

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/noop-executor/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/noop-executor/actions)
[![Crate](https://img.shields.io/crates/v/noop-executor.svg?style=flat-square)](https://crates.io/crates/noop-executor)
[![License](https://img.shields.io/github/license/raviqqe/noop-executor.svg?style=flat-square)](UNLICENSE)

A no-op executor in Rust.

This crate provides a `block_on` function that simply unwraps a future assuming it is ready.

## References

- If you use `async` operations as coroutines, you need to use crates like [Cassette](https://github.com/jamesmunns/cassette) instead.

## License

[The Unlicense](UNLICENSE)
