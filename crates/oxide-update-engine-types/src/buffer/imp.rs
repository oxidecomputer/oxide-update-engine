// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{
    reasons::{
        AbortInfo, AbortReason, CompletionInfo, CompletionReason, FailureInfo,
        FailureReason, WillNotBeRunReason,
    },
    summary::ExecutionSummary,
};
use crate::{
    events::{
        Event, EventReport, ExecutionUuid, ProgressEvent, ProgressEventKind,
        StepEvent, StepEventKind, StepEventPriority, StepInfo,
    },
    spec::{NestedSpec, StepSpec},
};
use derive_where::derive_where;
use either::Either;
use indexmap::IndexMap;
use petgraph::{prelude::*, visit::Walker};
use std::{
    collections::{HashMap, VecDeque},
    fmt,
    sync::Arc,
    time::Duration,
};

/// A receiver for events that provides a pull-based model with periodic
/// reports.
///
/// By default, the update engine provides a *push-based model* where you are
/// notified of new events as soon as they come in. The event buffer converts
/// events into a *pull-based model*, where periodic, serializable
/// [`EventReport`]s can be generated.
///
/// # Features
///
/// The buffer is responsible for tracking step and progress events as they
/// come in. The buffer can:
///
/// * Receive events and update its internal state based on them.
/// * Discard progress events that are no longer useful.
/// * Cap the number of [low-priority events](StepEventPriority::Low) such that
///   older events are dropped.
///
/// The buffer is currently resilient against:
///
/// * duplicated and dropped progress events
/// * duplicated step events
/// * dropped *low-priority* step events
///
/// The buffer is currently *not* resilient against:
/// * dropped *high-priority* step events
/// * reordered progress or step events
///
/// These cases can be handled on a best-effort basis in the future at some cost
/// to complexity, if required.
#[derive_where(Clone, Debug)]
pub struct EventBuffer<S: StepSpec> {
    event_store: EventStore<S>,
    max_low_priority: usize,
}

impl<S: StepSpec> EventBuffer<S> {
    /// Creates a new event buffer.
    ///
    /// `max_low_priority` determines the maximum number of low-priority events
    /// retained for a particular step at any given time.
    pub fn new(max_low_priority: usize) -> Self {
        Self { event_store: EventStore::default(), max_low_priority }
    }

    /// The default value for `max_low_priority`, as created by EventBuffer::default().
    pub const DEFAULT_MAX_LOW_PRIORITY: usize = 8;

    /// Adds an [`EventReport`] to the buffer.
    pub fn add_event_report(&mut self, report: EventReport<S>) {
        for event in report.step_events {
            self.add_step_event(event);
        }
        for event in report.progress_events {
            self.add_progress_event(event);
        }
    }

    /// Adds an individual [`Event`] to the buffer.
    pub fn add_event(&mut self, event: Event<S>) {
        match event {
            Event::Step(event) => {
                self.add_step_event(event);
            }
            Event::Progress(event) => {
                self.add_progress_event(event);
            }
        }
    }

    /// Adds a [`StepEvent`] to the buffer.
    ///
    /// This might cause older low-priority events to fall off the list.
    pub fn add_step_event(&mut self, event: StepEvent<S>) {
        self.event_store.handle_root_step_event(event, self.max_low_priority);
    }

    /// Returns the root execution ID, if this event buffer is aware of any
    /// events.
    pub fn root_execution_id(&self) -> Option<ExecutionUuid> {
        self.event_store.root_execution_id
    }

    /// Returns an execution summary for the root execution ID, if this event buffer is aware of any
    /// events.
    pub fn root_execution_summary(&self) -> Option<ExecutionSummary> {
        // XXX: more efficient algorithm
        let root_execution_id = self.root_execution_id()?;
        let mut summary = self.steps().summarize();
        summary.swap_remove(&root_execution_id)
    }

    /// Returns information about each step, as currently tracked by the buffer,
    /// in order of when the events were first defined.
    pub fn steps(&self) -> EventBufferSteps<'_, S> {
        EventBufferSteps::new(&self.event_store)
    }

    /// Iterates over all known steps in the buffer in a recursive fashion.
    ///
    /// The iterator is depth-first and pre-order (i.e. for nested steps, the
    /// parent step is visited before the child steps).
    pub fn iter_steps_recursive(
        &self,
    ) -> impl Iterator<Item = (StepKey, &EventBufferStepData<S>)> {
        self.event_store.event_map_value_dfs()
    }

    /// Returns information about the given step, as currently tracked by the
    /// buffer.
    pub fn get(&self, step_key: &StepKey) -> Option<&EventBufferStepData<S>> {
        self.event_store.map.get(step_key)
    }

    /// Returns per-execution data for the given execution ID.
    pub fn get_execution_data(
        &self,
        execution_id: &ExecutionUuid,
    ) -> Option<&EventBufferExecutionData> {
        self.event_store.execution_map.get(execution_id)
    }

    /// Generates an [`EventReport`] for this buffer.
    ///
    /// This report can be serialized and sent over the wire.
    pub fn generate_report(&self) -> EventReport<S> {
        self.generate_report_since(&mut None)
    }

    /// Generates an [`EventReport`] for this buffer, updating `last_seen` to a
    /// new value for incremental report generation.
    ///
    /// This report can be serialized and sent over the wire.
    pub fn generate_report_since(
        &self,
        last_seen: &mut Option<usize>,
    ) -> EventReport<S> {
        // Gather step events across all keys.
        let mut step_events = Vec::new();
        let mut progress_events = Vec::new();
        for (_, step_data) in self.steps().as_slice() {
            step_events
                .extend(step_data.step_events_since_impl(*last_seen).cloned());
            progress_events
                .extend(step_data.step_status.progress_event().cloned());
        }

        // Sort events.
        step_events.sort_unstable_by_key(|event| event.event_index);
        progress_events.sort_unstable_by_key(|event| event.total_elapsed);
        if let Some(last) = step_events.last() {
            // Only update last_seen if there are new step events (otherwise it
            // stays the same).
            *last_seen = Some(last.event_index);
        }

        EventReport {
            step_events,
            progress_events,
            root_execution_id: self.root_execution_id(),
            last_seen: *last_seen,
        }
    }

    /// Returns true if any further step events are pending since `last_seen`.
    ///
    /// This does not currently care about pending progress events, just pending
    /// step events. A typical use for this is to check that all step events
    /// have been reported before a sender shuts down.
    pub fn has_pending_events_since(&self, last_seen: Option<usize>) -> bool {
        for (_, step_data) in self.steps().as_slice() {
            if step_data.step_events_since_impl(last_seen).next().is_some() {
                return true;
            }
        }
        false
    }

    pub fn add_progress_event(&mut self, event: ProgressEvent<S>) {
        self.event_store.handle_progress_event(event);
    }

    // -- Test support methods --------------------------------------------------
    //
    // These expose internal structure for integration tests. They are not part
    // of the public API and may change without notice.

    /// Returns true if `key` is present in the internal event tree.
    #[doc(hidden)]
    pub fn __test_step_key_in_event_tree(&self, key: &StepKey) -> bool {
        self.event_store.event_tree.contains_node(EventTreeNode::Step(*key))
    }

    /// Returns true if `key` is present in the internal event map.
    #[doc(hidden)]
    pub fn __test_step_key_in_map(&self, key: &StepKey) -> bool {
        self.event_store.map.contains_key(key)
    }

    /// Verifies that `root_execution_id` is the sole root in the event tree
    /// (i.e. the only node with zero incoming edges). Returns `Ok(())` on
    /// success or an `Err` describing the violation.
    #[doc(hidden)]
    pub fn __test_verify_single_root(
        &self,
        root_execution_id: ExecutionUuid,
    ) -> Result<(), String> {
        use petgraph::Direction;

        for node in self.event_store.event_tree.nodes() {
            let count = self
                .event_store
                .event_tree
                .neighbors_directed(node, Direction::Incoming)
                .count();
            if node == EventTreeNode::Root(root_execution_id) {
                if count != 0 {
                    return Err(format!(
                        "for root execution ID, \
                         incoming neighbors should be 0 but got {count}"
                    ));
                }
            } else if count == 0 {
                return Err(format!(
                    "for non-root node {node:?}, \
                     incoming neighbors should be > 0"
                ));
            }
        }

        Ok(())
    }
}

impl<S: StepSpec> Default for EventBuffer<S> {
    fn default() -> Self {
        Self {
            event_store: Default::default(),
            max_low_priority: Self::DEFAULT_MAX_LOW_PRIORITY,
        }
    }
}

#[derive_where(Clone, Debug, Default)]
struct EventStore<S: StepSpec> {
    // A tree which has the general structure:
    //
    // root execution id ───> root step 0
    //     │      │
    //     │      └─────────> root step 1 ───> nested execution id
    //     │                                       │        │
    //     │                                       v        v
    //     │                             nested step 0    nested step 1
    //     │
    //     └────────────────> root step 2
    //
    // and so on.
    //
    // While petgraph seems like overkill at first, it results in really
    // straightforward algorithms below compared to alternatives like storing
    // trees using Box pointers.
    event_tree: DiGraphMap<EventTreeNode, ()>,
    root_execution_id: Option<ExecutionUuid>,
    map: HashMap<StepKey, EventBufferStepData<S>>,
    execution_map: HashMap<ExecutionUuid, EventBufferExecutionData>,
}

impl<S: StepSpec> EventStore<S> {
    /// Returns a DFS of event map values.
    fn event_map_value_dfs(
        &self,
    ) -> impl Iterator<Item = (StepKey, &EventBufferStepData<S>)> + '_ {
        self.root_execution_id.into_iter().flat_map(|execution_id| {
            let dfs =
                Dfs::new(&self.event_tree, EventTreeNode::Root(execution_id));
            dfs.iter(&self.event_tree).filter_map(|node| {
                if let EventTreeNode::Step(key) = node {
                    Some((key, &self.map[&key]))
                } else {
                    None
                }
            })
        })
    }

    /// Handles a non-nested step event.
    fn handle_root_step_event(
        &mut self,
        event: StepEvent<S>,
        max_low_priority: usize,
    ) {
        if matches!(event.kind, StepEventKind::Unknown) {
            // Ignore unknown events.
            return;
        }

        // This is a non-nested step event so the event index is a root event
        // index.
        let root_event_index = RootEventIndex(event.event_index);

        let actions = self.recurse_for_step_event(
            &event,
            0,
            None,
            None,
            root_event_index,
            event.total_elapsed,
        );

        if let Some(new_execution) = actions.new_execution {
            if new_execution.nest_level == 0 {
                self.root_execution_id = Some(new_execution.execution_id);
            }

            if !new_execution.steps_to_add.is_empty() {
                let total_steps = new_execution.steps_to_add.len();

                // Populate execution-level data, computing
                // parent_key_and_child_index if this is a new execution.
                // Use or_insert_with to preserve idempotent replay behavior.
                self.execution_map
                    .entry(new_execution.execution_id)
                    .or_insert_with(|| {
                        let parent_key_and_child_index = if let Some(
                            parent_key,
                        ) =
                            new_execution.parent_key
                        {
                            match self.map.get_mut(&parent_key) {
                                Some(parent_data) => {
                                    let child_index =
                                        parent_data.child_executions_seen;
                                    parent_data.child_executions_seen += 1;
                                    Some((parent_key, child_index))
                                }
                                None => {
                                    // This should never happen -- it
                                    // indicates that the parent key was
                                    // unknown. This can happen if we didn't
                                    // receive an event regarding a parent
                                    // execution being started.
                                    None
                                }
                            }
                        } else {
                            None
                        };

                        EventBufferExecutionData {
                            parent_key_and_child_index,
                            nest_level: new_execution.nest_level,
                            total_steps,
                        }
                    });

                for (new_step_key, new_step, sort_key) in
                    new_execution.steps_to_add
                {
                    // These are brand new steps so their keys shouldn't exist
                    // in the map. But if they do, don't overwrite them.
                    self.map.entry(new_step_key).or_insert_with(|| {
                        EventBufferStepData::new(
                            new_step,
                            sort_key,
                            root_event_index,
                        )
                    });
                }
            }
        }

        if let Some(key) = actions.progress_key {
            if let Some(value) = self.map.get_mut(&key) {
                // Set progress *before* adding the step event so that it can
                // transition to the running state if it isn't there already.
                if let Some(current_progress) = event.progress_event() {
                    value.set_progress(current_progress);
                }
            }
        }

        if let Some(key) = actions.step_key {
            if let Some(value) = self.map.get_mut(&key) {
                match event.kind.priority() {
                    StepEventPriority::High => {
                        value.add_high_priority_step_event(event);
                    }
                    StepEventPriority::Low => {
                        value.add_low_priority_step_event(
                            event,
                            max_low_priority,
                        );
                    }
                }
            }
        }
    }

    fn handle_progress_event(&mut self, event: ProgressEvent<S>) {
        if matches!(event.kind, ProgressEventKind::Unknown) {
            // Ignore unknown events.
            return;
        }

        if let Some(key) = Self::step_key_for_progress_event(&event) {
            if let Some(value) = self.map.get_mut(&key) {
                value.set_progress(event);
            }
        }
    }

    /// Recurses down the structure of a step event, adding nodes to the event
    /// tree as required. Returns the event key for the next event, if one is
    /// available.
    fn recurse_for_step_event<S2: StepSpec>(
        &mut self,
        event: &StepEvent<S2>,
        nest_level: usize,
        parent_key: Option<StepKey>,
        parent_sort_key: Option<&StepSortKey>,
        root_event_index: RootEventIndex,
        root_total_elapsed: Duration,
    ) -> RecurseActions {
        let mut new_execution = None;
        let (step_key, progress_key) = match &event.kind {
            StepEventKind::ExecutionStarted { steps, first_step, .. } => {
                let root_node = EventTreeNode::Root(event.execution_id);
                self.add_root_node(event.execution_id);
                // All nodes are added during the ExecutionStarted phase.
                let mut steps_to_add = Vec::new();
                for step in steps {
                    let step_key = StepKey {
                        execution_id: event.execution_id,
                        index: step.index,
                    };
                    let sort_key = StepSortKey::new(
                        parent_sort_key,
                        root_event_index.0,
                        step.index,
                    );
                    let step_node = self.add_step_node(step_key);
                    self.event_tree.add_edge(root_node, step_node, ());
                    let step_info = step.clone().into_generic();
                    steps_to_add.push((step_key, step_info, sort_key));
                }
                new_execution = Some(NewExecutionAction {
                    execution_id: event.execution_id,
                    parent_key,
                    nest_level,
                    steps_to_add,
                });

                // Register the start of progress.
                let key = StepKey {
                    execution_id: event.execution_id,
                    index: first_step.info.index,
                };
                (Some(key), Some(key))
            }
            StepEventKind::StepCompleted {
                step,
                attempt,
                outcome,
                next_step,
                step_elapsed,
                attempt_elapsed,
                ..
            } => {
                let key = StepKey {
                    execution_id: event.execution_id,
                    index: step.info.index,
                };
                let outcome = outcome.clone().into_generic();
                let info = CompletionInfo {
                    attempt: *attempt,
                    outcome,
                    root_total_elapsed,
                    leaf_total_elapsed: event.total_elapsed,
                    step_elapsed: *step_elapsed,
                    attempt_elapsed: *attempt_elapsed,
                };
                // Mark this key and all child keys completed.
                self.mark_step_key_completed(key, info, root_event_index);

                // Register the next step in the event map.
                let next_key = StepKey {
                    execution_id: event.execution_id,
                    index: next_step.info.index,
                };
                (Some(key), Some(next_key))
            }
            StepEventKind::ProgressReset { step, .. }
            | StepEventKind::AttemptRetry { step, .. } => {
                // Reset progress for the step in the event map.
                let key = StepKey {
                    execution_id: event.execution_id,
                    index: step.info.index,
                };
                (Some(key), Some(key))
            }
            StepEventKind::ExecutionCompleted {
                last_step: step,
                last_attempt,
                last_outcome,
                step_elapsed,
                attempt_elapsed,
            } => {
                // This is a terminal event: clear all progress for this
                // execution ID and any nested events.

                let key = StepKey {
                    execution_id: event.execution_id,
                    index: step.info.index,
                };
                let outcome = last_outcome.clone().into_generic();
                let info = CompletionInfo {
                    attempt: *last_attempt,
                    outcome,
                    root_total_elapsed,
                    leaf_total_elapsed: event.total_elapsed,
                    step_elapsed: *step_elapsed,
                    attempt_elapsed: *attempt_elapsed,
                };
                // Mark this key and all child keys completed.
                self.mark_execution_id_completed(key, info, root_event_index);

                (Some(key), Some(key))
            }
            StepEventKind::ExecutionFailed {
                failed_step: step,
                total_attempts,
                step_elapsed,
                attempt_elapsed,
                message,
                causes,
            } => {
                // This is a terminal event: clear all progress for this
                // execution ID and any nested events.

                let key = StepKey {
                    execution_id: event.execution_id,
                    index: step.info.index,
                };
                let info = FailureInfo {
                    total_attempts: *total_attempts,
                    message: message.clone(),
                    causes: causes.clone(),
                    root_total_elapsed,
                    leaf_total_elapsed: event.total_elapsed,
                    step_elapsed: *step_elapsed,
                    attempt_elapsed: *attempt_elapsed,
                };
                self.mark_step_failed(key, info, root_event_index);

                (Some(key), Some(key))
            }
            StepEventKind::ExecutionAborted {
                aborted_step: step,
                attempt,
                step_elapsed,
                attempt_elapsed,
                message,
            } => {
                // This is a terminal event: clear all progress for this
                // execution ID and any nested events.

                let key = StepKey {
                    execution_id: event.execution_id,
                    index: step.info.index,
                };
                let info = AbortInfo {
                    attempt: *attempt,
                    message: message.clone(),
                    root_total_elapsed,
                    leaf_total_elapsed: event.total_elapsed,
                    step_elapsed: *step_elapsed,
                    attempt_elapsed: *attempt_elapsed,
                };
                self.mark_step_aborted(key, info, root_event_index);

                (Some(key), Some(key))
            }
            StepEventKind::Nested { step, event: nested_event, .. } => {
                // Recurse and find any nested events.
                let parent_key = StepKey {
                    execution_id: event.execution_id,
                    index: step.info.index,
                };

                // The parent should always exist, but if it doesn't, don't fail on that.
                let parent_sort_key = self
                    .map
                    .get(&parent_key)
                    .map(|data| data.sort_key().clone());

                let actions = self.recurse_for_step_event(
                    nested_event,
                    nest_level + 1,
                    Some(parent_key),
                    parent_sort_key.as_ref(),
                    root_event_index,
                    root_total_elapsed,
                );
                if let Some(nested_new_execution) = &actions.new_execution {
                    // Add an edge from the parent node to the new execution's root node.
                    self.event_tree.add_edge(
                        EventTreeNode::Step(parent_key),
                        EventTreeNode::Root(nested_new_execution.execution_id),
                        (),
                    );
                }

                new_execution = actions.new_execution;
                (actions.step_key, actions.progress_key)
            }
            StepEventKind::NoStepsDefined | StepEventKind::Unknown => {
                (None, None)
            }
        };

        RecurseActions { new_execution, step_key, progress_key }
    }

    fn step_key_for_progress_event<S2: StepSpec>(
        event: &ProgressEvent<S2>,
    ) -> Option<StepKey> {
        match &event.kind {
            ProgressEventKind::WaitingForProgress { step, .. }
            | ProgressEventKind::Progress { step, .. } => {
                let key = StepKey {
                    execution_id: event.execution_id,
                    index: step.info.index,
                };
                Some(key)
            }
            ProgressEventKind::Nested { event: nested_event, .. } => {
                Self::step_key_for_progress_event(nested_event)
            }
            ProgressEventKind::Unknown => None,
        }
    }

    fn add_root_node(&mut self, execution_id: ExecutionUuid) -> EventTreeNode {
        self.event_tree.add_node(EventTreeNode::Root(execution_id))
    }

    fn add_step_node(&mut self, key: StepKey) -> EventTreeNode {
        self.event_tree.add_node(EventTreeNode::Step(key))
    }

    fn mark_step_key_completed(
        &mut self,
        root_key: StepKey,
        info: CompletionInfo,
        root_event_index: RootEventIndex,
    ) {
        let info = Arc::new(info);
        if let Some(value) = self.map.get_mut(&root_key) {
            // Completion status only applies to the root key. Nodes reachable
            // from this node are still marked as complete, but without status.
            value.mark_completed(
                CompletionReason::StepCompleted(info.clone()),
                root_event_index,
            );
        }

        // Mark anything reachable from this node as completed.
        let mut dfs =
            DfsPostOrder::new(&self.event_tree, EventTreeNode::Step(root_key));
        while let Some(key) = dfs.next(&self.event_tree) {
            if let EventTreeNode::Step(key) = key {
                if key != root_key {
                    if let Some(value) = self.map.get_mut(&key) {
                        value.mark_completed(
                            CompletionReason::ParentCompleted {
                                parent_step: root_key,
                                parent_info: info.clone(),
                            },
                            root_event_index,
                        );
                    }
                }
            }
        }
    }

    fn mark_execution_id_completed(
        &mut self,
        root_key: StepKey,
        info: CompletionInfo,
        root_event_index: RootEventIndex,
    ) {
        let info = Arc::new(info);
        if let Some(value) = self.map.get_mut(&root_key) {
            // Completion status only applies to the root key.
            value.mark_completed(
                CompletionReason::StepCompleted(info.clone()),
                root_event_index,
            );
        }

        let mut dfs = DfsPostOrder::new(
            &self.event_tree,
            EventTreeNode::Root(root_key.execution_id),
        );
        while let Some(key) = dfs.next(&self.event_tree) {
            if let EventTreeNode::Step(key) = key {
                if key != root_key {
                    if let Some(value) = self.map.get_mut(&key) {
                        // There's two kinds of nodes reachable from
                        // EventTreeNode::Root that could be marked as
                        // completed: subsequent steps within the same
                        // execution, and steps in child executions.
                        if key.execution_id == root_key.execution_id {
                            value.mark_completed(
                                CompletionReason::SubsequentStarted {
                                    later_step: root_key,
                                    root_total_elapsed: info.root_total_elapsed,
                                },
                                root_event_index,
                            );
                        } else {
                            value.mark_completed(
                                CompletionReason::ParentCompleted {
                                    parent_step: root_key,
                                    parent_info: info.clone(),
                                },
                                root_event_index,
                            );
                        }
                    }
                }
            }
        }
    }

    fn mark_step_failed(
        &mut self,
        root_key: StepKey,
        info: FailureInfo,
        root_event_index: RootEventIndex,
    ) {
        let info = Arc::new(info);
        self.mark_step_failed_impl(root_key, |value, kind| {
            match kind {
                MarkStepFailedImplKind::Root => {
                    value.mark_failed(
                        FailureReason::StepFailed(info.clone()),
                        root_event_index,
                    );
                }
                MarkStepFailedImplKind::Descendant => {
                    value.mark_failed(
                        FailureReason::ParentFailed {
                            parent_step: root_key,
                            parent_info: info.clone(),
                        },
                        root_event_index,
                    );
                }
                MarkStepFailedImplKind::Subsequent => {
                    value.mark_will_not_be_run(
                        WillNotBeRunReason::PreviousStepFailed {
                            step: root_key,
                        },
                        root_event_index,
                    );
                }
                MarkStepFailedImplKind::PreviousCompleted => {
                    value.mark_completed(
                        CompletionReason::SubsequentStarted {
                            later_step: root_key,
                            root_total_elapsed: info.root_total_elapsed,
                        },
                        root_event_index,
                    );
                }
            };
        })
    }

    fn mark_step_aborted(
        &mut self,
        root_key: StepKey,
        info: AbortInfo,
        root_event_index: RootEventIndex,
    ) {
        let info = Arc::new(info);
        self.mark_step_failed_impl(root_key, |value, kind| {
            match kind {
                MarkStepFailedImplKind::Root => {
                    value.mark_aborted(
                        AbortReason::StepAborted(info.clone()),
                        root_event_index,
                    );
                }
                MarkStepFailedImplKind::Descendant => {
                    value.mark_aborted(
                        AbortReason::ParentAborted {
                            parent_step: root_key,
                            parent_info: info.clone(),
                        },
                        root_event_index,
                    );
                }
                MarkStepFailedImplKind::Subsequent => {
                    value.mark_will_not_be_run(
                        WillNotBeRunReason::PreviousStepAborted {
                            step: root_key,
                        },
                        root_event_index,
                    );
                }
                MarkStepFailedImplKind::PreviousCompleted => {
                    value.mark_completed(
                        CompletionReason::SubsequentStarted {
                            later_step: root_key,
                            root_total_elapsed: info.root_total_elapsed,
                        },
                        root_event_index,
                    );
                }
            };
        });
    }

    fn mark_step_failed_impl(
        &mut self,
        root_key: StepKey,
        mut cb: impl FnMut(&mut EventBufferStepData<S>, MarkStepFailedImplKind),
    ) {
        if let Some(value) = self.map.get_mut(&root_key) {
            (cb)(value, MarkStepFailedImplKind::Root);
        }

        // Exceptional situation (in normal use, past steps should always show
        // up): Mark all past steps for this key as completed. The assumption
        // here is that this is the first step that failed.
        for index in 0..root_key.index {
            let key = StepKey { execution_id: root_key.execution_id, index };
            if let Some(value) = self.map.get_mut(&key) {
                (cb)(value, MarkStepFailedImplKind::PreviousCompleted);
            }
        }

        // Exceptional situation (in normal use, descendant steps should always
        // show up if they aren't being run): Mark all descendant steps as
        // failed -- there isn't enough else to go by.
        let mut dfs =
            DfsPostOrder::new(&self.event_tree, EventTreeNode::Step(root_key));
        while let Some(key) = dfs.next(&self.event_tree) {
            if let EventTreeNode::Step(key) = key {
                if let Some(value) = self.map.get_mut(&key) {
                    (cb)(value, MarkStepFailedImplKind::Descendant);
                }
            }
        }

        // Mark all future steps for this execution ID as "will not be run", We
        // do this last because all non-future steps for this execution ID will
        // have been covered by the above loops.
        let mut dfs = DfsPostOrder::new(
            &self.event_tree,
            EventTreeNode::Root(root_key.execution_id),
        );
        while let Some(key) = dfs.next(&self.event_tree) {
            if let EventTreeNode::Step(key) = key {
                if let Some(value) = self.map.get_mut(&key) {
                    (cb)(value, MarkStepFailedImplKind::Subsequent);
                }
            }
        }
    }
}

enum MarkStepFailedImplKind {
    Root,
    Descendant,
    Subsequent,
    PreviousCompleted,
}

/// Actions taken by a recursion step.
#[derive(Clone, Debug)]
struct RecurseActions {
    new_execution: Option<NewExecutionAction>,
    // The key to record this step against.
    step_key: Option<StepKey>,
    // The key to record the progress action against.
    progress_key: Option<StepKey>,
}

#[derive(Clone, Debug)]
struct NewExecutionAction {
    // An execution ID corresponding to a new run, if seen.
    execution_id: ExecutionUuid,

    // The parent key for this execution, if this is a nested step.
    parent_key: Option<StepKey>,

    // The nest level for this execution.
    nest_level: usize,

    // New steps to add, generated by ExecutionStarted events.
    // The tuple is:
    // * step key
    // * step info
    // * step sort key
    steps_to_add: Vec<(StepKey, StepInfo<NestedSpec>, StepSortKey)>,
}

/// An ordered list of steps contained in an event buffer.
///
/// Returned by [`EventBuffer::steps`].
#[derive_where(Clone, Debug)]
pub struct EventBufferSteps<'buf, S: StepSpec> {
    steps: Vec<(StepKey, &'buf EventBufferStepData<S>)>,
}

impl<'buf, S: StepSpec> EventBufferSteps<'buf, S> {
    fn new(event_store: &'buf EventStore<S>) -> Self {
        let mut steps: Vec<_> = event_store.event_map_value_dfs().collect();
        steps.sort_unstable_by_key(|(_, value)| value.sort_key());
        Self { steps }
    }

    /// Returns the list of steps in the event buffer.
    pub fn as_slice(&self) -> &[(StepKey, &'buf EventBufferStepData<S>)] {
        &self.steps
    }

    /// Summarizes the current state of all known executions, keyed by execution
    /// ID.
    ///
    /// Values are returned as an `IndexMap`, in order of when execution IDs
    /// were first defined.
    pub fn summarize(&self) -> IndexMap<ExecutionUuid, ExecutionSummary> {
        let mut by_execution_id: IndexMap<ExecutionUuid, Vec<_>> =
            IndexMap::new();
        // Index steps by execution key.
        for &(step_key, data) in &self.steps {
            by_execution_id
                .entry(step_key.execution_id)
                .or_default()
                .push(data);
        }

        by_execution_id
            .into_iter()
            .map(|(execution_id, steps)| {
                let summary = ExecutionSummary::new(execution_id, &steps);
                (execution_id, summary)
            })
            .collect()
    }
}

/// Per-execution data tracked by the event buffer.
///
/// Unlike [`EventBufferStepData`], which is keyed by individual step,
/// this data is shared across all steps within a single execution.
#[derive(Clone, Debug)]
pub struct EventBufferExecutionData {
    parent_key_and_child_index: Option<(StepKey, usize)>,
    nest_level: usize,
    total_steps: usize,
}

impl EventBufferExecutionData {
    #[inline]
    pub fn parent_key_and_child_index(&self) -> Option<(StepKey, usize)> {
        self.parent_key_and_child_index
    }

    #[inline]
    pub fn nest_level(&self) -> usize {
        self.nest_level
    }

    #[inline]
    pub fn total_steps(&self) -> usize {
        self.total_steps
    }
}

/// Step-related data for a particular key.
#[derive_where(Clone, Debug)]
pub struct EventBufferStepData<S: StepSpec> {
    step_info: StepInfo<NestedSpec>,

    sort_key: StepSortKey,

    child_executions_seen: usize,

    // Invariant: stored in order sorted by leaf event index.
    high_priority: Vec<StepEvent<S>>,
    step_status: StepStatus<S>,
    // The last root event index that caused the data within this step to be
    // updated.
    last_root_event_index: RootEventIndex,
}

impl<S: StepSpec> EventBufferStepData<S> {
    fn new(
        step_info: StepInfo<NestedSpec>,
        sort_key: StepSortKey,
        root_event_index: RootEventIndex,
    ) -> Self {
        Self {
            step_info,
            sort_key,
            child_executions_seen: 0,
            high_priority: Vec::new(),
            step_status: StepStatus::NotStarted,
            last_root_event_index: root_event_index,
        }
    }

    #[inline]
    pub fn step_info(&self) -> &StepInfo<NestedSpec> {
        &self.step_info
    }

    #[inline]
    pub fn child_executions_seen(&self) -> usize {
        self.child_executions_seen
    }

    #[inline]
    pub fn step_status(&self) -> &StepStatus<S> {
        &self.step_status
    }

    #[inline]
    pub fn last_root_event_index(&self) -> RootEventIndex {
        self.last_root_event_index
    }

    #[inline]
    fn sort_key(&self) -> &StepSortKey {
        &self.sort_key
    }

    /// Returns a reference to the sort key for test comparison.
    #[doc(hidden)]
    #[inline]
    pub fn __test_sort_key(&self) -> &StepSortKey {
        &self.sort_key
    }

    /// Returns step events since the provided event index.
    pub fn step_events_since(
        &self,
        last_seen: Option<usize>,
    ) -> Vec<&StepEvent<S>> {
        let mut events: Vec<_> =
            self.step_events_since_impl(last_seen).collect();
        events.sort_unstable_by_key(|event| event.event_index);
        events
    }

    // Returns step events since the provided event index.
    //
    // Does not necessarily return results in sorted order.
    fn step_events_since_impl(
        &self,
        last_seen: Option<usize>,
    ) -> impl Iterator<Item = &StepEvent<S>> {
        let iter = self
            .high_priority
            .iter()
            .filter(move |event| Some(event.event_index) > last_seen);
        let iter2 = self
            .step_status
            .low_priority()
            .filter(move |event| Some(event.event_index) > last_seen);
        iter.chain(iter2)
    }

    fn add_high_priority_step_event(&mut self, root_event: StepEvent<S>) {
        let root_event_index = RootEventIndex(root_event.event_index);
        // Dedup by the *leaf index* in case nested reports aren't deduped
        // coming in.
        match self.high_priority.binary_search_by(|probe| {
            probe.leaf_event_index().cmp(&root_event.leaf_event_index())
        }) {
            Ok(_) => {
                // This is a duplicate.
            }
            Err(index) => {
                // index is typically the last element, so this should be quite
                // efficient.
                self.update_root_event_index(root_event_index);
                self.high_priority.insert(index, root_event);
            }
        }
    }

    fn add_low_priority_step_event(
        &mut self,
        root_event: StepEvent<S>,
        max_low_priority: usize,
    ) {
        let root_event_index = RootEventIndex(root_event.event_index);
        let mut updated = false;
        match &mut self.step_status {
            StepStatus::NotStarted => {
                unreachable!(
                    "we always set progress before adding low-pri step events"
                );
            }
            StepStatus::Running { low_priority, .. } => {
                // Dedup by the *leaf index* in case nested reports aren't
                // deduped coming in.
                match low_priority.binary_search_by(|probe| {
                    probe.leaf_event_index().cmp(&root_event.leaf_event_index())
                }) {
                    Ok(_) => {
                        // This is a duplicate.
                    }
                    Err(index) => {
                        // The index is almost always at the end, so this is
                        // efficient enough.
                        low_priority.insert(index, root_event);
                        updated = true;
                    }
                }

                // Limit the number of events to the maximum low priority, ejecting
                // the oldest event(s) if necessary.
                while low_priority.len() > max_low_priority {
                    low_priority.pop_front();
                }
            }
            StepStatus::Completed { .. }
            | StepStatus::Failed { .. }
            | StepStatus::Aborted { .. }
            | StepStatus::WillNotBeRun { .. } => {
                // Ignore low-priority events for terminated steps since they're
                // likely duplicate events.
            }
        }

        if updated {
            self.update_root_event_index(root_event_index);
        }
    }

    fn mark_completed(
        &mut self,
        reason: CompletionReason,
        root_event_index: RootEventIndex,
    ) {
        match self.step_status {
            StepStatus::NotStarted | StepStatus::Running { .. } => {
                self.step_status = StepStatus::Completed { reason };
                self.update_root_event_index(root_event_index);
            }
            StepStatus::Completed { .. }
            | StepStatus::Failed { .. }
            | StepStatus::Aborted { .. }
            | StepStatus::WillNotBeRun { .. } => {
                // Ignore the status if the step has already been marked
                // terminated.
            }
        }
    }

    fn mark_failed(
        &mut self,
        reason: FailureReason,
        root_event_index: RootEventIndex,
    ) {
        match self.step_status {
            StepStatus::NotStarted | StepStatus::Running { .. } => {
                self.step_status = StepStatus::Failed { reason };
                self.update_root_event_index(root_event_index);
            }
            StepStatus::Completed { .. }
            | StepStatus::Failed { .. }
            | StepStatus::Aborted { .. }
            | StepStatus::WillNotBeRun { .. } => {
                // Ignore the status if the step has already been marked
                // terminated.
            }
        }
    }

    fn mark_aborted(
        &mut self,
        reason: AbortReason,
        root_event_index: RootEventIndex,
    ) {
        match &mut self.step_status {
            StepStatus::NotStarted => {
                match reason {
                    AbortReason::ParentAborted { parent_step, .. } => {
                        // A parent was aborted and this step hasn't been
                        // started.
                        self.step_status = StepStatus::WillNotBeRun {
                            reason: WillNotBeRunReason::ParentAborted {
                                step: parent_step,
                            },
                        };
                    }
                    AbortReason::StepAborted(info) => {
                        self.step_status = StepStatus::Aborted {
                            reason: AbortReason::StepAborted(info),
                            last_progress: None,
                        };
                    }
                }
                self.update_root_event_index(root_event_index);
            }
            StepStatus::Running { progress_event, .. } => {
                self.step_status = StepStatus::Aborted {
                    reason,
                    last_progress: Some(progress_event.clone()),
                };
                self.update_root_event_index(root_event_index);
            }
            StepStatus::Completed { .. }
            | StepStatus::Failed { .. }
            | StepStatus::Aborted { .. }
            | StepStatus::WillNotBeRun { .. } => {
                // Ignore the status if the step has already been marked
                // terminated.
            }
        }
    }

    fn mark_will_not_be_run(
        &mut self,
        reason: WillNotBeRunReason,
        root_event_index: RootEventIndex,
    ) {
        match self.step_status {
            StepStatus::NotStarted => {
                self.step_status = StepStatus::WillNotBeRun { reason };
                self.update_root_event_index(root_event_index);
            }
            StepStatus::Running { .. } => {
                // This is a weird situation. We should never encounter it in
                // normal use -- if we do encounter it, just ignore it.
            }
            StepStatus::Completed { .. }
            | StepStatus::Failed { .. }
            | StepStatus::Aborted { .. }
            | StepStatus::WillNotBeRun { .. } => {
                // Ignore the status if the step has already been marked
                // terminated.
            }
        }
    }

    fn set_progress(&mut self, current_progress: ProgressEvent<S>) {
        match &mut self.step_status {
            StepStatus::NotStarted => {
                self.step_status = StepStatus::Running {
                    low_priority: VecDeque::new(),
                    progress_event: current_progress,
                };
            }
            StepStatus::Running { progress_event, .. } => {
                *progress_event = current_progress;
            }
            StepStatus::Aborted { last_progress, .. } => {
                *last_progress = Some(current_progress);
            }
            StepStatus::Completed { .. }
            | StepStatus::Failed { .. }
            | StepStatus::WillNotBeRun { .. } => {
                // Ignore progress events for completed steps.
            }
        }
    }

    fn update_root_event_index(&mut self, root_event_index: RootEventIndex) {
        debug_assert!(
            root_event_index >= self.last_root_event_index,
            "event index must be monotonically increasing"
        );
        self.last_root_event_index =
            self.last_root_event_index.max(root_event_index);
    }
}

/// The step status as last seen by events.
#[derive_where(Clone, Debug)]
pub enum StepStatus<S: StepSpec> {
    NotStarted,

    /// The step is currently running.
    Running {
        // Invariant: stored in sorted order by index.
        low_priority: VecDeque<StepEvent<S>>,
        progress_event: ProgressEvent<S>,
    },

    /// The step has completed execution.
    Completed {
        /// The reason for completion.
        reason: CompletionReason,
    },

    /// The step has failed.
    Failed {
        /// The reason for the failure.
        reason: FailureReason,
    },

    /// Execution was aborted while this step was running.
    Aborted {
        /// The reason for the abort.
        reason: AbortReason,

        /// The last progress seen, if any.
        last_progress: Option<ProgressEvent<S>>,
    },

    /// The step will not be executed because a prior step failed.
    WillNotBeRun {
        /// The step that failed and caused this step to not be run.
        reason: WillNotBeRunReason,
    },
}

impl<S: StepSpec> StepStatus<S> {
    /// Returns true if this step is currently running.
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running { .. })
    }

    /// For completed steps, return the completion reason, otherwise None.
    pub fn completion_reason(&self) -> Option<&CompletionReason> {
        match self {
            Self::Completed { reason, .. } => Some(reason),
            _ => None,
        }
    }

    /// For failed steps, return the failure reason, otherwise None.
    pub fn failure_reason(&self) -> Option<&FailureReason> {
        match self {
            Self::Failed { reason, .. } => Some(reason),
            _ => None,
        }
    }

    /// For aborted steps, return the abort reason, otherwise None.
    pub fn abort_reason(&self) -> Option<&AbortReason> {
        // TODO: probably want to move last_progress into the `AbortReason`
        // enum so that we can return it in a reasonable manner here.
        match self {
            Self::Aborted { reason, .. } => Some(reason),
            _ => None,
        }
    }

    /// For will-not-be-run steps, return the reason, otherwise None.
    pub fn will_not_be_run_reason(&self) -> Option<&WillNotBeRunReason> {
        match self {
            Self::WillNotBeRun { reason } => Some(reason),
            _ => None,
        }
    }

    /// Returns low-priority events for this step, if any.
    ///
    /// Events are sorted by event index.
    pub fn low_priority(&self) -> impl Iterator<Item = &StepEvent<S>> {
        match self {
            Self::Running { low_priority, .. } => {
                Either::Left(low_priority.iter())
            }
            Self::NotStarted
            | Self::Completed { .. }
            | Self::Failed { .. }
            | Self::Aborted { .. }
            | Self::WillNotBeRun { .. } => Either::Right(std::iter::empty()),
        }
    }

    /// Returns the associated progress event for this step, if any.
    pub fn progress_event(&self) -> Option<&ProgressEvent<S>> {
        match self {
            Self::Running { progress_event, .. } => Some(progress_event),
            Self::Aborted { last_progress, .. } => last_progress.as_ref(),
            Self::NotStarted
            | Self::Completed { .. }
            | Self::Failed { .. }
            | Self::WillNotBeRun { .. } => None,
        }
    }
}

/// Step sort key.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct StepSortKey {
    // The tuples here are (defined at index, step index) pairs.
    values: Vec<(usize, usize)>,
}

impl StepSortKey {
    fn new(
        parent: Option<&Self>,
        defined_at_index: usize,
        step_index: usize,
    ) -> Self {
        let mut values = if let Some(parent) = parent {
            parent.values.clone()
        } else {
            Vec::new()
        };
        values.push((defined_at_index, step_index));
        Self { values }
    }
}

/// Keys for the event tree.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum EventTreeNode {
    Root(ExecutionUuid),
    Step(StepKey),
}

/// A unique identifier for a group of step or progress events.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct StepKey {
    pub execution_id: ExecutionUuid,
    pub index: usize,
}

/// A newtype to track root event indexes within [`EventBuffer`]s, to ensure
/// that we aren't mixing them with leaf event indexes in this code.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct RootEventIndex(pub usize);

impl fmt::Display for RootEventIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
