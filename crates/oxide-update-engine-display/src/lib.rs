// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![cfg_attr(doc_cfg, feature(doc_cfg))]
// Setting html_root_url allows cross-crate readme links to resolve. This
// line is updated by cargo-release.
#![doc(html_root_url = "https://docs.rs/oxide-update-engine-display/0.1.0")]

//! Displayers for the update engine.
//!
//! Currently implemented are:
//!
//! * [`LineDisplay`]: a line-oriented display suitable for the command
//!   line.
//! * [`GroupDisplay`]: manages state and shows the results of several
//!   [`LineDisplay`]s at once.
//! * Some utility displayers which can be used to build custom
//!   displayers.

mod group_display;
mod line_display;
mod line_display_shared;
mod utils;

pub use group_display::{GroupDisplay, GroupDisplayStats};
pub use line_display::{LineDisplay, LineDisplayStyles};
// Re-export AbortMessageDisplay from the types crate.
pub use oxide_update_engine_types::buffer::AbortMessageDisplay;
pub use utils::*;
