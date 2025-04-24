# No-op executor

A no-op executor in Rust.

This crate provides a `block_on` function that simply unwraps a future assuming it is ready.

## References

- If you use `async` operations as coroutines, you need to use crates like [Cassette](https://github.com/jamesmunns/cassette) instead.

## License

[The Unlicense](UNLICENSE)
