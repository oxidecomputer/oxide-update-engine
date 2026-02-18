// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![cfg_attr(doc_cfg, feature(doc_cfg))]
// Setting html_root_url allows cross-crate readme links to resolve. This
// line is updated by cargo-release.
#![doc(html_root_url = "https://docs.rs/oxide-update-engine/0.1.0")]

//! An engine for declaring and executing sequential update steps with
//! serializable event streams.
//!
//! The `oxide-update-engine` crate provides the execution engine for
//! running update steps.
//!
//! * For types-only consumers (e.g. API clients), see
//!   [`oxide-update-engine-types`](oxide_update_engine_types).
//! * For code to display update engine events as a human-readable
//!   stream, see [`oxide-update-engine-display`](https://docs.rs/oxide-update-engine-display).
//!
//! # Examples
//!
//! A minimal engine that runs a single update step:
//!
//! ```
//! use oxide_update_engine::{
//!     StepSuccess, UpdateEngine, channel,
//! };
//! use oxide_update_engine_types::spec::StepSpec;
//!
//! // A StepSpec defines the domain-specific types that flow
//! // through the engine. Use () for metadata you don't need.
//! enum MySpec {}
//! impl StepSpec for MySpec {
//!     fn spec_name() -> String {
//!         "example".into()
//!     }
//!     type Component = String;
//!     type StepId = usize;
//!     type StepMetadata = ();
//!     type ProgressMetadata = ();
//!     type CompletionMetadata = ();
//!     type SkippedMetadata = ();
//!     type Error = anyhow::Error;
//! }
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let log =
//!     slog::Logger::root(slog::Discard, slog::o!());
//! let (sender, _receiver) = channel::<MySpec>();
//! let engine = UpdateEngine::new(&log, sender);
//!
//! // Steps run sequentially in registration order.
//! engine
//!     .new_step(
//!         "fw".to_owned(),
//!         1,
//!         "Write firmware image",
//!         |_cx| async {
//!             // ... perform update work here ...
//!             StepSuccess::new(()).into()
//!         },
//!     )
//!     .register();
//!
//! engine.execute().await?;
//! # Ok(())
//! # }
//! ```
//!
//! For more complex engines, including engines that have nested local and
//! remote steps, see [the full
//! example](https://github.com/oxidecomputer/oxide-update-engine/blob/main/crates/oxide-update-engine-display/examples/basic/main.rs).

mod context;
mod engine;
mod errors;

pub(crate) use context::StepContextPayload;
pub use context::{
    CompletionContext, MetadataContext, SharedStepHandle, StepContext,
    StepHandle, StepHandleToken,
};
pub use engine::{
    AbortHandle, AbortWaiter, ComponentRegistrar, ExecutionHandle, NewStep,
    StepResult, StepSkipped, StepSuccess, StepWarning, UpdateEngine, channel,
};
pub use errors::ExecutionError;
