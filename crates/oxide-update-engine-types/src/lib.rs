// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![cfg_attr(doc_cfg, feature(doc_cfg))]
// Setting html_root_url allows cross-crate readme links to resolve. This
// line is updated by cargo-release.
#![doc(html_root_url = "https://docs.rs/oxide-update-engine-types/0.1.0")]

//! Types for `oxide-update-engine`.
//!
//! This crate contains the serializable types used by the update
//! engine: events, event buffers, step specifications, and errors.
//! It has no dependency on the execution engine itself, making it
//! suitable for consumers that only need to read or display events.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

pub mod buffer;
pub mod errors;
pub mod events;
mod macros;
pub mod spec;

/// A unique identifier for an execution of an update engine.
///
/// Each time an `UpdateEngine` is executed, it is assigned a unique
/// `ExecutionId`.
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Deserialize,
    Serialize,
)]
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct ExecutionId(pub Uuid);

impl fmt::Display for ExecutionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
