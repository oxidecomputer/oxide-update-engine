// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Integration tests for [`EventBuffer`].
//!
//! These live outside the crate because `generate_test_events` requires the
//! engine, and adding the engine as a dev-dependency of the types crate would
//! create a duplicate-crate issue (the types crate would appear twice in the
//! dependency graph).

use anyhow::{Context, bail, ensure};
use indexmap::IndexSet;
use oxide_update_engine_test_utils::{
    GenerateTestEventsKind, TestSpec, generate_test_events,
};
use oxide_update_engine_types::{
    buffer::{
        EventBuffer, ExecutionStatus, RootEventIndex, StepKey, TerminalKind,
    },
    events::{
        Event, EventReport, ExecutionUuid, ProgressCounter, ProgressEvent,
        ProgressEventKind, StepEvent, StepEventKind, StepEventPriority,
    },
    spec::StepSpec,
};
use serde::{Deserialize, de::IntoDeserializer};
use std::collections::HashSet;

#[tokio::test]
async fn test_buffer() {
    let log = slog::Logger::root(slog::Discard, slog::o!());
    let generated_events =
        generate_test_events(&log, GenerateTestEventsKind::Completed).await;

    let test_cx = BufferTestContext::new(generated_events);

    test_cx
        .run_all_elements_test(
            "all events passed in one-by-one",
            |buffer, event| buffer.add_event(event.clone()),
            1,
        )
        .unwrap();

    test_cx
        .run_all_elements_test(
            "all events duplicated (1)",
            |buffer, event| {
                buffer.add_event(event.clone());
                buffer.add_event(event.clone());
            },
            1,
        )
        .unwrap();

    test_cx
        .run_all_elements_test(
            "all events duplicated (2)",
            |buffer, event| {
                buffer.add_event(event.clone());
            },
            2,
        )
        .unwrap();

    test_cx
        .run_filtered_test(
            "all events passed in",
            |buffer, event| {
                buffer.add_event(event.clone());
                true
            },
            WithDeltas::No,
        )
        .unwrap();

    test_cx
        .run_filtered_test(
            "progress events skipped",
            |buffer, event| match event {
                Event::Step(event) => {
                    buffer.add_step_event(event.clone());
                    true
                }
                Event::Progress(_) => false,
            },
            WithDeltas::Both,
        )
        .unwrap();

    test_cx
        .run_filtered_test(
            "low-priority events skipped",
            |buffer, event| match event {
                Event::Step(event) => match event.kind.priority() {
                    StepEventPriority::High => {
                        buffer.add_step_event(event.clone());
                        true
                    }
                    StepEventPriority::Low => false,
                },
                Event::Progress(event) => {
                    buffer.add_progress_event(event.clone());
                    true
                }
            },
            WithDeltas::Both,
        )
        .unwrap();

    test_cx
        .run_filtered_test(
            "low-priority and progress events skipped",
            |buffer, event| match event {
                Event::Step(event) => match event.kind.priority() {
                    StepEventPriority::High => {
                        buffer.add_step_event(event.clone());
                        true
                    }
                    StepEventPriority::Low => false,
                },
                Event::Progress(_) => {
                    // Don't add progress events.
                    false
                }
            },
            WithDeltas::Both,
        )
        .unwrap();
}

/// This number is small enough that it will cause low-priority events to be
/// dropped in some cases.
const MAX_LOW_PRIORITY: usize = 4;

#[derive(Debug)]
struct BufferTestContext {
    root_execution_id: ExecutionUuid,
    generated_events: Vec<Event<TestSpec>>,

    // Data derived from generated_events.
    generated_step_events: Vec<StepEvent<TestSpec>>,
}

impl BufferTestContext {
    fn new(generated_events: Vec<Event<TestSpec>>) -> Self {
        // The first event is always a root event.
        let root_execution_id =
            match generated_events.first().expect("at least one event") {
                Event::Step(event) => event.execution_id,
                Event::Progress(_) => {
                    panic!("first event should always be a step event")
                }
            };

        // Ensure that nested step 2 produces progress events in the
        // expected order and in succession.
        let mut progress_check = NestedProgressCheck::new();
        for event in &generated_events {
            if let Event::Progress(event) = event {
                let progress_counter = event.kind.progress_counter();
                if progress_counter
                    == Some(&ProgressCounter::new(2, 3, "steps"))
                {
                    progress_check.two_out_of_three_seen();
                } else if progress_check
                    == NestedProgressCheck::TwoOutOfThreeSteps
                {
                    assert_eq!(
                        progress_counter,
                        Some(&ProgressCounter::current(50, "units"))
                    );
                    progress_check.fifty_units_seen();
                } else if progress_check == NestedProgressCheck::FiftyUnits {
                    assert_eq!(
                        progress_counter,
                        Some(&ProgressCounter::new(3, 3, "steps"))
                    );
                    progress_check.three_out_of_three_seen();
                }
            }
        }
        progress_check.assert_done();

        // Ensure that events are never seen twice.
        let mut event_indexes_seen = HashSet::new();
        let mut leaf_event_indexes_seen = HashSet::new();
        let generated_step_events: Vec<_> = generated_events
            .iter()
            .filter_map(|event| match event {
                Event::Step(event) => {
                    if event_indexes_seen.contains(&event.event_index) {
                        panic!(
                            "duplicate event seen for index {}",
                            event.event_index
                        );
                    }
                    event_indexes_seen.insert(event.event_index);

                    let leaf_event_key =
                        (event.leaf_execution_id(), event.leaf_event_index());
                    if leaf_event_indexes_seen.contains(&leaf_event_key) {
                        panic!(
                            "duplicate leaf event seen \
                             for key {leaf_event_key:?}",
                        );
                    }
                    leaf_event_indexes_seen.insert(leaf_event_key);

                    Some(event.clone())
                }
                Event::Progress(_) => None,
            })
            .collect();

        // Create two buffers and feed events.
        // * The incremental buffer has each event fed into it one-by-one.
        // * The "idempotent" buffer has events 0, 0..1, 0..2, 0..3, etc
        //   fed into it one by one. The name is because this is really
        //   testing the idempotency of the event buffer.

        println!("** generating incremental and idempotent buffers **");
        let mut incremental_buffer = EventBuffer::default();
        let mut idempotent_buffer = EventBuffer::default();
        for event in &generated_events {
            incremental_buffer.add_event(event.clone());
            let report = incremental_buffer.generate_report();
            idempotent_buffer.add_event_report(report);
        }

        // Check that the two buffers above are similar.
        ensure_buffers_similar(&incremental_buffer, &idempotent_buffer)
            .expect("idempotent buffer is similar to incremental buffer");

        // Also generate a buffer with a single event report.
        println!("** generating oneshot buffer **");
        let mut oneshot_buffer = EventBuffer::default();
        oneshot_buffer.add_event_report(incremental_buffer.generate_report());

        ensure_buffers_similar(&incremental_buffer, &oneshot_buffer)
            .expect("oneshot buffer is similar to incremental buffer");

        Self { root_execution_id, generated_events, generated_step_events }
    }

    /// Runs a test in a scenario where all elements should be seen.
    ///
    /// Each event is added `times` times.
    fn run_all_elements_test(
        &self,
        description: &str,
        mut event_fn: impl FnMut(&mut EventBuffer<TestSpec>, &Event<TestSpec>),
        times: usize,
    ) -> anyhow::Result<()> {
        let mut buffer: EventBuffer<TestSpec> =
            EventBuffer::new(MAX_LOW_PRIORITY);
        let mut reported_step_events = Vec::new();
        let mut last_seen = None;

        for (i, event) in self.generated_events.iter().enumerate() {
            for time in 0..times {
                (event_fn)(&mut buffer, event);
                let report = buffer.generate_report_since(&mut last_seen);
                let is_last_event = i == self.generated_events.len() - 1;
                self.assert_general_properties(&buffer, &report, is_last_event)
                    .with_context(|| {
                        format!(
                            "{description}, at index {i} (time {time}), \
                            properties not met"
                        )
                    })
                    .unwrap();
                reported_step_events.extend(report.step_events);

                // Ensure that the last root index was updated for this
                // event's corresponding steps, but not for any others.
                if let Event::Step(event) = event {
                    check_last_root_event_index(event, &buffer).with_context(
                        || {
                            format!(
                                "{description}, at index {i} (time {time}):\
                                 error with last root event index"
                            )
                        },
                    )?;
                }

                // Call last_seen without feeding a new event in to ensure that
                // a report with no step events is produced.
                let mut last_seen_2 = last_seen;
                let report = buffer.generate_report_since(&mut last_seen_2);
                ensure!(
                    report.step_events.is_empty(),
                    "{description}, at index {i} (time {time}),\
                    no step events are seen"
                );
                ensure!(
                    report.last_seen == last_seen,
                    "{description}, at index {i} (time {time}), \
                    report.last_seen {:?} matches last_seen {:?}",
                    report.last_seen,
                    last_seen,
                );
            }
        }

        ensure!(
            self.generated_step_events == reported_step_events,
            "all generated step events were reported"
        );

        Ok(())
    }

    /// Runs a test in a scenario where not all events might be replayed.
    fn run_filtered_test(
        &self,
        event_fn_description: &str,
        mut event_fn: impl FnMut(
            &mut EventBuffer<TestSpec>,
            &Event<TestSpec>,
        ) -> bool,
        with_deltas: WithDeltas,
    ) -> anyhow::Result<()> {
        match with_deltas {
            WithDeltas::Yes => {
                self.run_filtered_test_inner(&mut event_fn, true)
                    .context(event_fn_description.to_owned())?;
            }
            WithDeltas::No => {
                self.run_filtered_test_inner(&mut event_fn, false)
                    .context(event_fn_description.to_owned())?;
            }
            WithDeltas::Both => {
                self.run_filtered_test_inner(&mut event_fn, true)
                    .context(event_fn_description.to_owned())?;
                self.run_filtered_test_inner(&mut event_fn, false)
                    .context(event_fn_description.to_owned())?;
            }
        }

        Ok(())
    }

    fn run_filtered_test_inner(
        &self,
        mut event_fn: impl FnMut(
            &mut EventBuffer<TestSpec>,
            &Event<TestSpec>,
        ) -> bool,
        with_deltas: bool,
    ) -> anyhow::Result<()> {
        let description = format!("with deltas = {with_deltas}");
        let mut buffer = EventBuffer::new(MAX_LOW_PRIORITY);
        let mut last_high_priority = Vec::new();

        // This buffer has a large enough capacity that it never drops
        // events.
        let mut receive_buffer = EventBuffer::new(1024);
        // last_seen_opt is None if with_deltas is false.
        //
        // Some(None) if with_deltas is true and no events have been seen
        // so far.
        //
        // Some(Some(index)) if with_deltas is true and events have been
        // seen.
        let mut last_seen_opt = with_deltas.then_some(None);

        for (i, event) in self.generated_events.iter().enumerate() {
            let event_added = (event_fn)(&mut buffer, event);

            let report = match &mut last_seen_opt {
                Some(last_seen) => buffer.generate_report_since(last_seen),
                None => buffer.generate_report(),
            };

            let is_last_event = i == self.generated_events.len() - 1;
            self.assert_general_properties(&buffer, &report, is_last_event)
                .with_context(|| {
                    format!("{description}, at index {i}, properties not met")
                })
                .unwrap();

            if let Event::Step(event) = event {
                if event_added {
                    check_last_root_event_index(event, &buffer).with_context(
                        || {
                            format!(
                                "{description}, at index {i}: \
                                error with last root event index"
                            )
                        },
                    )?;
                }
            }

            receive_buffer.add_event_report(report.clone());
            let this_step_events = receive_buffer.generate_report().step_events;
            let this_high_priority: Vec<_> = this_step_events
                .iter()
                .filter(|event| {
                    event.kind.priority() == StepEventPriority::High
                })
                .cloned()
                .collect();

            if !with_deltas {
                let report_high_priority: Vec<_> = report
                    .step_events
                    .iter()
                    .filter(|event| {
                        event.kind.priority() == StepEventPriority::High
                    })
                    .cloned()
                    .collect();
                ensure!(
                    this_high_priority == report_high_priority,
                    "{description}, at index {i}, \
                     all high-priority events reported"
                );
            }

            if this_high_priority.len() == last_high_priority.len() {
                // event is not a high-priority event. All old high-priority
                // events must be reported as well.
                ensure!(
                    this_high_priority == last_high_priority,
                    "{description}, at index {i}, \
                     all old high-priority events reported (1)"
                );
            } else if this_high_priority.len() == last_high_priority.len() + 1 {
                // The first N events must match.
                ensure!(
                    this_high_priority[0..last_high_priority.len()]
                        == last_high_priority,
                    "{description}, at index {i}, \
                     all old high-priority events reported (2)"
                );
            }

            last_high_priority = this_high_priority;
        }

        Ok(())
    }

    fn assert_general_properties<S: StepSpec>(
        &self,
        buffer: &EventBuffer<TestSpec>,
        report: &EventReport<S>,
        is_last_event: bool,
    ) -> anyhow::Result<()> {
        let mut progress_keys_seen = HashSet::new();
        for event in &report.progress_events {
            let key = progress_event_key(event);
            // Ensure that there's one progress event per report.
            if progress_keys_seen.contains(&key) {
                bail!("progress event key {key:?} seen twice in event map")
            }
            progress_keys_seen.insert(key);
            // Check that the buffer has an event in the tree and map
            // corresponding to any reports seen.
            if !buffer.__test_step_key_in_event_tree(&key) {
                bail!("progress event key {key:?} not found in event tree");
            }
            if !buffer.__test_step_key_in_map(&key) {
                bail!("progress event key {key:?} not found in event map");
            }
        }

        // Assert that steps are always in order. To check this, we use step
        // IDs, which we externally define to be in order.
        let steps = buffer.steps();
        for window in steps.as_slice().windows(2) {
            let (_, data1) = window[0];
            let (_, data2) = window[1];
            let data1_id: usize = Deserialize::deserialize(
                data1.step_info().id.clone().into_deserializer(),
            )
            .expect("data1.id is a usize");
            let data2_id: usize = Deserialize::deserialize(
                data2.step_info().id.clone().into_deserializer(),
            )
            .expect("data2.id is a usize");
            ensure!(
                data1_id < data2_id,
                "data 1 ID {data1_id} < data 2 ID {data2_id}"
            );
        }

        // The root execution ID should have a summary associated with it.
        let root_execution_id = buffer
            .root_execution_id()
            .expect("at least one event => root execution ID exists");
        ensure!(
            root_execution_id == self.root_execution_id,
            "root execution ID matches"
        );
        let summary = steps.summarize();
        ensure!(
            summary.contains_key(&root_execution_id),
            "summary contains root execution ID {root_execution_id:?}"
        );

        if is_last_event {
            ensure!(
                matches!(
                    &summary[&root_execution_id].execution_status,
                    ExecutionStatus::Terminal(info)
                        if info.kind == TerminalKind::Completed
                ),
                "this is the last event so ExecutionStatus must be completed"
            );
            // There are three nested engines.
            ensure!(
                summary.len() == 4,
                "three nested engines (plus one root engine) must be defined"
            );

            let (_, nested_summary) =
                summary.get_index(1).expect("this is the first nested engine");
            ensure!(
                matches!(
                    &nested_summary.execution_status,
                    ExecutionStatus::Terminal(info)
                        if info.kind == TerminalKind::Failed
                ),
                "for this engine, the ExecutionStatus must be failed"
            );

            let (_, nested_summary) =
                summary.get_index(2).expect("this is the second nested engine");
            ensure!(
                matches!(
                    &nested_summary.execution_status,
                    ExecutionStatus::Terminal(info)
                        if info.kind == TerminalKind::Failed
                ),
                "for this engine, the ExecutionStatus must be failed"
            );

            let (_, nested_summary) =
                summary.get_index(3).expect("this is the third nested engine");
            ensure!(
                matches!(
                    &nested_summary.execution_status,
                    ExecutionStatus::Terminal(info)
                        if info.kind == TerminalKind::Completed
                ),
                "for this engine, the ExecutionStatus must be succeeded"
            );
        } else {
            ensure!(
                matches!(
                    summary[&root_execution_id].execution_status,
                    ExecutionStatus::Running { .. },
                ),
                "not the last event so ExecutionStatus must be running"
            );
        }

        // The root execution ID should be the only root in the event tree.
        buffer
            .__test_verify_single_root(root_execution_id)
            .map_err(|msg| anyhow::anyhow!(msg))?;

        Ok(())
    }
}

fn ensure_buffers_similar<S: StepSpec>(
    buf1: &EventBuffer<S>,
    buf2: &EventBuffer<S>,
) -> anyhow::Result<()> {
    // The two should have the same step keys.
    let buf1_steps = buf1.steps();
    let buf2_steps = buf2.steps();

    ensure!(
        buf1_steps.as_slice().len() == buf2_steps.as_slice().len(),
        "buffers have same number of steps ({} vs {})",
        buf1_steps.as_slice().len(),
        buf2_steps.as_slice().len()
    );

    // Collect unique execution IDs to compare per-execution data.
    let mut execution_ids_seen = HashSet::new();

    for (ix, ((k1, data1), (k2, data2))) in buf1_steps
        .as_slice()
        .iter()
        .zip(buf2_steps.as_slice().iter())
        .enumerate()
    {
        ensure!(
            k1 == k2,
            "buffers have same step keys at index {} ({:?} vs {:?})",
            ix,
            k1,
            k2
        );
        ensure!(
            data1.__test_sort_key() == data2.__test_sort_key(),
            "buffers have same sort key at index {} ({:?} vs {:?})",
            ix,
            data1.__test_sort_key(),
            data2.__test_sort_key()
        );

        // Compare per-execution data once per unique execution ID.
        if execution_ids_seen.insert(k1.execution_id) {
            let exec1 = buf1
                .get_execution_data(&k1.execution_id)
                .expect("execution data must exist in buf1");
            let exec2 = buf2
                .get_execution_data(&k1.execution_id)
                .expect("execution data must exist in buf2");
            ensure!(
                exec1.parent_key_and_child_index()
                    == exec2.parent_key_and_child_index(),
                "buffers have same parent key and child index \
                 for execution {:?} ({:?} vs {:?})",
                k1.execution_id,
                exec1.parent_key_and_child_index(),
                exec2.parent_key_and_child_index(),
            );
            ensure!(
                exec1.nest_level() == exec2.nest_level(),
                "buffers have same nest level \
                 for execution {:?} ({:?} vs {:?})",
                k1.execution_id,
                exec1.nest_level(),
                exec2.nest_level(),
            );
            ensure!(
                exec1.total_steps() == exec2.total_steps(),
                "buffers have same total steps \
                 for execution {:?} ({:?} vs {:?})",
                k1.execution_id,
                exec1.total_steps(),
                exec2.total_steps(),
            );
        }
    }

    Ok(())
}

fn check_last_root_event_index(
    event: &StepEvent<TestSpec>,
    buffer: &EventBuffer<TestSpec>,
) -> anyhow::Result<()> {
    let root_event_index = RootEventIndex(event.event_index);
    let event_step_keys = step_keys(event);
    let steps = buffer.steps();
    for (step_key, data) in steps.as_slice() {
        let data_index = data.last_root_event_index();
        if event_step_keys.contains(step_key) {
            ensure!(
                data_index == root_event_index,
                "last_root_event_index should have been updated \
                 but wasn't (actual: {data_index}, expected: {root_event_index}) \
                 for step {step_key:?} (event: {event:?})",
            );
        } else {
            ensure!(
                data_index < root_event_index,
                "last_root_event_index should *not* have been updated \
                 but was (current: {data_index}, new: {root_event_index}) \
                 for step {step_key:?} (event: {event:?})",
            );
        }
    }

    Ok(())
}

/// Returns the step keys that this step event would cause updates against,
/// in order from root to leaf.
fn step_keys<S: StepSpec>(event: &StepEvent<S>) -> IndexSet<StepKey> {
    let mut out = IndexSet::new();
    step_keys_impl(event, &mut out);
    out
}

fn step_keys_impl<S: StepSpec>(
    event: &StepEvent<S>,
    out: &mut IndexSet<StepKey>,
) {
    match &event.kind {
        StepEventKind::NoStepsDefined | StepEventKind::Unknown => {}
        StepEventKind::ExecutionStarted { steps, .. } => {
            for step in steps {
                out.insert(StepKey {
                    execution_id: event.execution_id,
                    index: step.index,
                });
            }
        }
        StepEventKind::ProgressReset { step, .. }
        | StepEventKind::AttemptRetry { step, .. }
        | StepEventKind::StepCompleted { step, .. }
        | StepEventKind::ExecutionCompleted { last_step: step, .. }
        | StepEventKind::ExecutionFailed { failed_step: step, .. }
        | StepEventKind::ExecutionAborted { aborted_step: step, .. } => {
            out.insert(StepKey {
                execution_id: event.execution_id,
                index: step.info.index,
            });
        }
        StepEventKind::Nested { step, event, .. } => {
            out.insert(StepKey {
                execution_id: event.execution_id,
                index: step.info.index,
            });
            step_keys_impl(event, out);
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(unused)]
enum WithDeltas {
    Yes,
    No,
    Both,
}

fn progress_event_key<S: StepSpec>(event: &ProgressEvent<S>) -> StepKey {
    match &event.kind {
        ProgressEventKind::WaitingForProgress { step, .. }
        | ProgressEventKind::Progress { step, .. } => {
            StepKey { execution_id: event.execution_id, index: step.info.index }
        }
        ProgressEventKind::Nested { event: nested_event, .. } => {
            progress_event_key(nested_event)
        }
        ProgressEventKind::Unknown => {
            panic!("we should never generate an unknown key")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NestedProgressCheck {
    Initial,
    TwoOutOfThreeSteps,
    FiftyUnits,
    ThreeOutOfThreeSteps,
}

impl NestedProgressCheck {
    fn new() -> Self {
        Self::Initial
    }

    fn two_out_of_three_seen(&mut self) {
        assert_eq!(
            *self,
            Self::Initial,
            "two_out_of_three_seen: expected Initial",
        );
        *self = Self::TwoOutOfThreeSteps;
    }

    fn fifty_units_seen(&mut self) {
        assert_eq!(
            *self,
            Self::TwoOutOfThreeSteps,
            "fifty_units_seen: expected TwoOutOfThreeSteps",
        );
        *self = Self::FiftyUnits;
    }

    fn three_out_of_three_seen(&mut self) {
        assert_eq!(
            *self,
            Self::FiftyUnits,
            "three_out_of_three_seen: expected FiftyUnits",
        );
        *self = Self::ThreeOutOfThreeSteps;
    }

    fn assert_done(&self) {
        assert_eq!(
            *self,
            Self::ThreeOutOfThreeSteps,
            "assert_done: expected ThreeOutOfThreeSteps",
        );
    }
}
