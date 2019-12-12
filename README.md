# tokio-childstream

A
[`Stream`](https://docs.rs/futures-core/0.3.1/futures_core/stream/trait.Stream.html)
over
[`Child`](https://docs.rs/tokio/0.2.4/tokio/process/struct.Child.html)
events for [`tokio`](https://docs.rs/tokio/0.2.4/tokio/index.html),
such as stdout/err lines or exit status.

## License

MIT License
Copyright 2019 Nathan Wilcox

## Status

Not-yet-proof-of-concept.

## Todo

- Multiplex over stdout / stderr lines.
- Add exit status.
- Refactor API / improve tests.
- Bump version number / release.
