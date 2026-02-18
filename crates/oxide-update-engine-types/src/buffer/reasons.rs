// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{EventBuffer, StepKey};
use crate::{
    events::StepOutcome,
    spec::{NestedSpec, StepSpec},
};
use std::{fmt, sync::Arc, time::Duration};

#[derive(Clone, Debug)]
pub enum CompletionReason {
    /// This step completed.
    StepCompleted(Arc<CompletionInfo>),
    /// A later step within the same execution was started and we don't have
    /// information regarding this step.
    SubsequentStarted {
        /// The later step that was started.
        later_step: StepKey,

        /// The root total elapsed time at the moment the later step was started.
        root_total_elapsed: Duration,
    },
    /// A parent step within the same execution completed and we don't have
    /// information regarding this step.
    ParentCompleted {
        /// The parent step that completed.
        parent_step: StepKey,

        /// Completion info associated with the parent step.
        parent_info: Arc<CompletionInfo>,
    },
}

impl CompletionReason {
    /// Returns the [`CompletionInfo`] for this step, if this is the
    /// [`Self::StepCompleted`] variant.
    pub fn step_completed_info(&self) -> Option<&Arc<CompletionInfo>> {
        match self {
            Self::StepCompleted(info) => Some(info),
            Self::SubsequentStarted { .. } | Self::ParentCompleted { .. } => {
                None
            }
        }
    }
}

/// Completion information associated with a step.
#[derive(Clone, Debug)]
pub struct CompletionInfo {
    /// The attempt number of the step.
    pub attempt: usize,

    /// The outcome of the step: success, warning, or skipped.
    pub outcome: StepOutcome<NestedSpec>,

    /// The total elapsed time as reported by the root event.
    pub root_total_elapsed: Duration,

    /// The total elapsed time as reported by the leaf execution event, for
    /// nested events.
    pub leaf_total_elapsed: Duration,

    /// Duration elapsed for the step.
    pub step_elapsed: Duration,

    /// Duration elapsed for the attempt.
    pub attempt_elapsed: Duration,
}

#[derive(Clone, Debug)]
pub enum FailureReason {
    /// This step failed.
    StepFailed(Arc<FailureInfo>),
    /// A parent step failed.
    ParentFailed {
        /// The parent step that failed.
        parent_step: StepKey,

        /// Failure info associated with the parent step.
        parent_info: Arc<FailureInfo>,
    },
}

impl FailureReason {
    /// Returns the [`FailureInfo`] for this step, if this is the
    /// [`Self::StepFailed`] variant.
    pub fn step_failed_info(&self) -> Option<&Arc<FailureInfo>> {
        match self {
            Self::StepFailed(info) => Some(info),
            Self::ParentFailed { .. } => None,
        }
    }
}

/// Information about a failed step.
#[derive(Clone, Debug)]
pub struct FailureInfo {
    /// The total number of attempts made for this step.
    pub total_attempts: usize,

    /// The failure message.
    pub message: String,

    /// Failure causes.
    pub causes: Vec<String>,

    /// The total elapsed time as reported by the root event.
    pub root_total_elapsed: Duration,

    /// The total elapsed time as reported by the leaf execution event, for
    /// nested events.
    pub leaf_total_elapsed: Duration,

    /// Duration elapsed for the step.
    pub step_elapsed: Duration,

    /// Duration elapsed for the attempt.
    pub attempt_elapsed: Duration,
}

#[derive(Clone, Debug)]
pub enum AbortReason {
    /// This step was aborted.
    StepAborted(Arc<AbortInfo>),
    /// A parent step was aborted.
    ParentAborted {
        /// The parent step key that was aborted.
        parent_step: StepKey,

        /// Abort info associated with the parent step.
        parent_info: Arc<AbortInfo>,
    },
}

impl AbortReason {
    /// Returns the [`AbortInfo`] for this step, if this is the
    /// [`Self::StepAborted`] variant.
    pub fn step_aborted_info(&self) -> Option<&Arc<AbortInfo>> {
        match self {
            Self::StepAborted(info) => Some(info),
            Self::ParentAborted { .. } => None,
        }
    }

    /// Returns a displayer for the message.
    ///
    /// The buffer is used to resolve step keys to step names.
    pub fn message_display<'a, S: StepSpec>(
        &'a self,
        buffer: &'a EventBuffer<S>,
    ) -> AbortMessageDisplay<'a, S> {
        AbortMessageDisplay::new(self, buffer)
    }
}

#[derive(Clone, Debug)]
pub enum WillNotBeRunReason {
    /// A preceding step failed.
    PreviousStepFailed {
        /// The previous step that failed.
        step: StepKey,
    },

    /// A parent step failed.
    ParentStepFailed {
        /// The parent step that failed.
        step: StepKey,
    },

    /// Execution was aborted during a previous step.
    PreviousStepAborted {
        /// The step which was aborted.
        step: StepKey,
    },

    /// A parent step was aborted.
    ParentAborted {
        /// The parent step which was aborted.
        step: StepKey,
    },
}

/// Information associated with a step aborted by the user.
#[derive(Clone, Debug)]
pub struct AbortInfo {
    /// The last attempt number seen.
    pub attempt: usize,

    /// The message associated with the abort.
    pub message: String,

    /// The total elapsed time as reported by the root event.
    pub root_total_elapsed: Duration,

    /// The total elapsed time as reported by the leaf execution event, for
    /// nested events.
    pub leaf_total_elapsed: Duration,

    /// Duration elapsed for the step.
    pub step_elapsed: Duration,

    /// Duration elapsed for the attempt.
    pub attempt_elapsed: Duration,
}

/// Displays the message for an execution abort.
///
/// Returned by [`AbortReason::message_display`].
pub struct AbortMessageDisplay<'a, S: StepSpec> {
    reason: &'a AbortReason,
    buffer: &'a EventBuffer<S>,
    // TODO: color.
}

impl<'a, S: StepSpec> AbortMessageDisplay<'a, S> {
    /// Create a new `AbortMessageDisplay`.
    pub fn new(reason: &'a AbortReason, buffer: &'a EventBuffer<S>) -> Self {
        Self { reason, buffer }
    }
}

impl<S: StepSpec> fmt::Display for AbortMessageDisplay<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reason {
            AbortReason::StepAborted(info) => {
                write!(f, "{}", info.message)
            }
            AbortReason::ParentAborted { parent_step, parent_info } => {
                let parent_description =
                    if let Some(step) = self.buffer.get(parent_step) {
                        &step.step_info().description
                    } else {
                        "unknown step"
                    };

                write!(
                    f,
                    "parent step \"{}\" aborted with: {}",
                    parent_description, parent_info.message
                )
            }
        }
    }
}
