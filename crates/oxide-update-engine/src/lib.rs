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
//!   stream, see [`oxide-update-engine-display`](oxide_update_engine_display).

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
