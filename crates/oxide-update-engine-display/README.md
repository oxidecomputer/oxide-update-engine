<!-- cargo-sync-rdme title [[ -->
# oxide-update-engine-display
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
![License: MPL-2.0](https://img.shields.io/crates/l/oxide-update-engine-display.svg?)
[![crates.io](https://img.shields.io/crates/v/oxide-update-engine-display.svg?logo=rust)](https://crates.io/crates/oxide-update-engine-display)
[![docs.rs](https://img.shields.io/docsrs/oxide-update-engine-display.svg?logo=docs.rs)](https://docs.rs/oxide-update-engine-display)
[![Rust: ^1.85.0](https://img.shields.io/badge/rust-^1.85.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme rustdoc [[ -->
Displayers for the update engine.

Currently implemented are:

* [`LineDisplay`](https://docs.rs/oxide-update-engine-display/0.1.0/oxide_update_engine_display/line_display/struct.LineDisplay.html): a line-oriented display suitable for the command
  line.
* [`GroupDisplay`](https://docs.rs/oxide-update-engine-display/0.1.0/oxide_update_engine_display/group_display/struct.GroupDisplay.html): manages state and shows the results of several
  [`LineDisplay`](https://docs.rs/oxide-update-engine-display/0.1.0/oxide_update_engine_display/line_display/struct.LineDisplay.html)s at once.
* Some utility displayers which can be used to build custom
  displayers.
<!-- cargo-sync-rdme ]] -->

## License

This project is available under the terms of the [Mozilla Public License 2.0](../../LICENSE).
