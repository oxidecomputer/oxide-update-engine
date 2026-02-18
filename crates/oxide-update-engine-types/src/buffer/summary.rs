// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{EventBufferStepData, StepKey, StepStatus};
use crate::{events::ExecutionUuid, spec::StepSpec};
use std::{fmt, time::Duration};

#[derive(Clone, Debug)]
pub struct ExecutionSummary {
    pub total_steps: usize,
    pub execution_status: ExecutionStatus,
}

impl ExecutionSummary {
    // steps should be in order.
    pub(super) fn new<S: StepSpec>(
        execution_id: ExecutionUuid,
        steps: &[&EventBufferStepData<S>],
    ) -> Self {
        let total_steps = steps.len();
        // Iterate through the steps to figure out the current status. Since
        // steps is in order, the last step that isn't NotStarted wins.
        let mut execution_status = ExecutionStatus::NotStarted;
        for data in steps {
            let step_key =
                StepKey { execution_id, index: data.step_info().index };
            match data.step_status() {
                StepStatus::NotStarted => {
                    // This step hasn't been started yet. Skip over it.
                }
                StepStatus::Running { low_priority, progress_event } => {
                    let root_total_elapsed = low_priority
                        .iter()
                        .map(|event| event.total_elapsed)
                        .chain(std::iter::once(progress_event.total_elapsed))
                        .max()
                        .expect("at least one value was provided");
                    execution_status = ExecutionStatus::Running {
                        step_key,
                        root_total_elapsed,
                    };
                }
                StepStatus::Completed { reason } => {
                    let (root_total_elapsed, leaf_total_elapsed) =
                        match reason.step_completed_info() {
                            Some(info) => (
                                Some(info.root_total_elapsed),
                                Some(info.leaf_total_elapsed),
                            ),
                            None => (None, None),
                        };

                    let terminal_status = ExecutionTerminalInfo {
                        kind: TerminalKind::Completed,
                        root_total_elapsed,
                        leaf_total_elapsed,
                        step_key,
                    };
                    execution_status =
                        ExecutionStatus::Terminal(terminal_status);
                }
                StepStatus::Failed { reason } => {
                    let (root_total_elapsed, leaf_total_elapsed) =
                        match reason.step_failed_info() {
                            Some(info) => (
                                Some(info.root_total_elapsed),
                                Some(info.leaf_total_elapsed),
                            ),
                            None => (None, None),
                        };

                    let terminal_status = ExecutionTerminalInfo {
                        kind: TerminalKind::Failed,
                        root_total_elapsed,
                        leaf_total_elapsed,
                        step_key,
                    };
                    execution_status =
                        ExecutionStatus::Terminal(terminal_status);
                }
                StepStatus::Aborted { reason, .. } => {
                    let (root_total_elapsed, leaf_total_elapsed) =
                        match reason.step_aborted_info() {
                            Some(info) => (
                                Some(info.root_total_elapsed),
                                Some(info.leaf_total_elapsed),
                            ),
                            None => (None, None),
                        };

                    let terminal_status = ExecutionTerminalInfo {
                        kind: TerminalKind::Aborted,
                        root_total_elapsed,
                        leaf_total_elapsed,
                        step_key,
                    };
                    execution_status =
                        ExecutionStatus::Terminal(terminal_status);
                }
                StepStatus::WillNotBeRun { .. } => {
                    // Ignore steps that will not be run -- a prior step failed.
                }
            };
        }

        Self { total_steps, execution_status }
    }
}

/// Status about a single execution ID.
///
/// Part of [`ExecutionSummary`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionStatus {
    /// This execution has not been started yet.
    NotStarted,

    /// This execution is currently running.
    Running {
        /// The step key that's currently running.
        ///
        /// Use [`super::EventBuffer::get`] to get more information about this step.
        step_key: StepKey,

        /// The maximum root_total_elapsed seen.
        root_total_elapsed: Duration,
    },

    /// Execution has finished.
    Terminal(ExecutionTerminalInfo),
}

impl ExecutionStatus {
    /// Returns the terminal status and the total amount of time elapsed, or
    /// None if the execution has not reached a terminal state.
    ///
    /// The time elapsed might be None if the execution was interrupted and
    /// completion information wasn't available.
    pub fn terminal_info(&self) -> Option<&ExecutionTerminalInfo> {
        match self {
            Self::NotStarted | Self::Running { .. } => None,
            Self::Terminal(info) => Some(info),
        }
    }
}

/// Terminal status about a single execution ID.
///
/// Part of [`ExecutionStatus`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionTerminalInfo {
    /// The way in which this execution reached a terminal state.
    pub kind: TerminalKind,

    /// Total elapsed time (root) for this execution.
    ///
    /// The total elapsed time may not be available if execution was interrupted
    /// and we inferred that it was terminated.
    pub root_total_elapsed: Option<Duration>,

    /// Total elapsed time (leaf) for this execution.
    ///
    /// The total elapsed time may not be available if execution was interrupted
    /// and we inferred that it was terminated.
    pub leaf_total_elapsed: Option<Duration>,

    /// The step key that was running when this execution was terminated.
    ///
    /// * For completed executions, this is the last step that completed.
    /// * For failed or aborted executions, this is the step that failed.
    /// * For aborted executions, this is the step that was running when the
    ///   abort happened.
    pub step_key: StepKey,
}

/// The way in which an execution was terminated.
///
/// Part of [`ExecutionStatus`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalKind {
    /// This execution completed running.
    Completed,
    /// This execution failed.
    Failed,
    /// This execution was aborted.
    Aborted,
}

impl fmt::Display for TerminalKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Aborted => write!(f, "aborted"),
        }
    }
}
