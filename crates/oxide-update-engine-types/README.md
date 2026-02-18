<!-- cargo-sync-rdme title [[ -->
# oxide-update-engine-types
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
![License: MPL-2.0](https://img.shields.io/crates/l/oxide-update-engine-types.svg?)
[![crates.io](https://img.shields.io/crates/v/oxide-update-engine-types.svg?logo=rust)](https://crates.io/crates/oxide-update-engine-types)
[![docs.rs](https://img.shields.io/docsrs/oxide-update-engine-types.svg?logo=docs.rs)](https://docs.rs/oxide-update-engine-types)
[![Rust: ^1.85.0](https://img.shields.io/badge/rust-^1.85.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme rustdoc [[ -->
Types for `oxide-update-engine`.

This crate contains the serializable types used by the update
engine: events, event buffers, step specifications, and errors.
It has no dependency on the execution engine itself, making it
suitable for consumers that only need to read or display events.
<!-- cargo-sync-rdme ]] -->

## License

This project is available under the terms of the [Mozilla Public License 2.0](../../LICENSE).
