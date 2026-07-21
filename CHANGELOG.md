# Changelog

<!-- next-header -->
## Unreleased - ReleaseDate

### Added

- `ProgressEventKind::leaf_progress` returns progress for an event, recursing through `Nested` events. This is similar to `ProgressEventKind::progress_counter` except it distinguishes "waiting for progress" from "unknown progress".

## [0.1.1] - 2026-07-20

### Added

- `EventBuffer::iter_steps_for_execution` returns the steps of a single execution
  in step-index order (empty for unknown executions).
- `EventBufferStepData::child_execution_ids` returns the child executions nested
  under a step, in child-index order. The `define_update_engine_types!` macro now
  also generates an `EventBufferStepData` alias.
- `FailureReason::message_display` returns a `FailureMessageDisplay` that renders
  a failure's message followed by its causes (`message: cause1: cause2`), and
  for parent failures prefixes the parent step's description. This method
  mirrors the existing `AbortReason::message_display`.

## [0.1.0] - 2026-02-27

Initial extraction from the [omicron](https://github.com/oxidecomputer/omicron)
repository.

<!-- next-url -->
[0.1.1]: https://github.com/oxidecomputer/oxide-update-engine/releases/tag/oxide-update-engine-0.1.1
[0.1.0]: https://github.com/oxidecomputer/oxide-update-engine/releases/tag/oxide-update-engine-0.1.0
