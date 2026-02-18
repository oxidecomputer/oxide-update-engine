<!-- cargo-sync-rdme title [[ -->
# oxide-update-engine
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
![License: MPL-2.0](https://img.shields.io/crates/l/oxide-update-engine.svg?)
[![crates.io](https://img.shields.io/crates/v/oxide-update-engine.svg?logo=rust)](https://crates.io/crates/oxide-update-engine)
[![docs.rs](https://img.shields.io/docsrs/oxide-update-engine.svg?logo=docs.rs)](https://docs.rs/oxide-update-engine)
[![Rust: ^1.85.0](https://img.shields.io/badge/rust-^1.85.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme rustdoc [[ -->
An engine for declaring and executing sequential update steps with
serializable event streams.

The `oxide-update-engine` crate provides the execution engine for
running update steps.

* For types-only consumers (e.g. API clients), see
  [`oxide-update-engine-types`](https://docs.rs/oxide-update-engine-types/0.1.0/oxide_update_engine_types/index.html).
* For code to display update engine events as a human-readable
  stream, see [`oxide-update-engine-display`](https://docs.rs/oxide-update-engine-display).

## Examples

A minimal engine that runs a single update step:

````rust
use oxide_update_engine::{
    StepSuccess, UpdateEngine, channel,
};
use oxide_update_engine_types::spec::StepSpec;

// A StepSpec defines the domain-specific types that flow
// through the engine. Use () for metadata you don't need.
enum MySpec {}
impl StepSpec for MySpec {
    fn spec_name() -> String {
        "example".into()
    }
    type Component = String;
    type StepId = usize;
    type StepMetadata = ();
    type ProgressMetadata = ();
    type CompletionMetadata = ();
    type SkippedMetadata = ();
    type Error = anyhow::Error;
}

let log =
    slog::Logger::root(slog::Discard, slog::o!());
let (sender, _receiver) = channel::<MySpec>();
let engine = UpdateEngine::new(&log, sender);

// Steps run sequentially in registration order.
engine
    .new_step(
        "fw".to_owned(),
        1,
        "Write firmware image",
        |_cx| async {
            // ... perform update work here ...
            StepSuccess::new(()).into()
        },
    )
    .register();

engine.execute().await?;
````

For more complex engines, including engines that have nested local and
remote steps, see [the full
example](https://github.com/oxidecomputer/oxide-update-engine/blob/main/crates/oxide-update-engine-display/examples/basic/main.rs).
<!-- cargo-sync-rdme ]] -->

## License

This project is available under the terms of the [Mozilla Public License 2.0](../../LICENSE).
