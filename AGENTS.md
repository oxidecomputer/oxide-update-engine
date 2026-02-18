# Instructions for LLMs

This file provides guidance to LLMs when working with code in this repository.

## Build and test commands

- **Format**: `cargo xfmt` (alias defined in `.cargo/config.toml`; adds `imports_granularity=Crate` and `group_imports=One` on top of `rustfmt.toml`)
- **Check format**: `cargo xfmt --check`
- **Clippy**: `cargo clippy --all-targets --all-features`
- **Build**: `cargo build --workspace`
- **Test (always use nextest)**: `cargo nextest run --workspace --all-features`
- **Single test**: `cargo nextest run --workspace --all-features -E 'test(test_name)'`
- **Doctests** (nextest doesn't support these): `cargo test --doc --workspace --all-features`
- **Feature powerset verification**: `just powerset check` or `just powerset nextest run` (uses `cargo hack --feature-powerset`; CI runs this)
- **Build docs**: `just rustdoc`

CI runs clippy and tests against the full feature powerset on both MSRV (1.85) and stable. `RUSTFLAGS=-D warnings` is enforced.

## Project overview

A framework for declaring and executing **sequential, retryable update steps** with a **serializable event stream**. Built for firmware/software update workflows where steps must run in order, progress must be observable (locally and remotely), and steps can spawn nested sub-executions (including on remote servers).

## Workspace structure

```
crates/
  oxide-update-engine-types/      # Serializable types only (no execution logic)
  oxide-update-engine/            # Execution engine (depends on types)
  oxide-update-engine-display/    # Human-readable rendering (depends on types)
  oxide-update-engine-test-utils/ # Shared test helpers (not published)
```

The key design split: `oxide-update-engine-types` has **zero dependency on the execution engine**, so API clients or remote consumers can depend only on the types crate.

## Core architecture

### `StepSpec` trait (types crate, `spec.rs`)

Everything is parameterized by `S: StepSpec`, a trait that bundles associated types (Component, StepId, StepMetadata, ProgressMetadata, CompletionMetadata, SkippedMetadata, Error). This acts as a "type family" that fully describes what domain-specific types flow through the engine.

Two built-in instantiations:
- **`GenericSpec<E>`**: all metadata fields are `serde_json::Value`, used as a lowest-common-denominator type for cross-engine communication. Concrete specs round-trip through `into_generic()`/`from_generic()`.
- **`NestedSpec`** (`= GenericSpec<NestedError>`): for nested engine events flowing upward through the event tree.

### Event system (types crate, `events.rs`)

- **`StepEvent<S>`**: lifecycle transitions (ExecutionStarted, StepCompleted, ExecutionCompleted/Failed/Aborted, AttemptRetry, Nested, etc.)
- **`ProgressEvent<S>`**: fine-grained progress within a step (Progress with current/total, Nested, etc.)
- All event enums have an `Unknown` variant with `#[serde(other)]` for forward-compatible deserialization.
- `StepEventKind` carries a `priority()` (High/Low). High-priority events are never dropped from buffers; low-priority events can be evicted.

### Execution engine (engine crate, `engine.rs`)

`UpdateEngine::new(log, sender)` -> register steps via `engine.new_step(...)` -> `engine.execute()` returns `ExecutionHandle`.

Internally, execution uses a double-select pattern: an inner select drives the step future and payload receiver, while an outer select handles abort signals. This is intentional; merging them would break the "exit when both done" semantics.

**Sender polymorphism**: `SenderImpl<S>` trait object allows `DefaultSender` (top-level engine) and `NestedSender` (nested engine with a different `StepSpec`) behind a single `Arc<dyn SenderImpl<S>>`.

### Step handles and context (engine crate, `context.rs`)

- **`StepContext<S>`**: passed to each step function. Provides `send_progress()`, `with_nested_engine()`, `send_nested_report()`.
- **`StepHandle<T, S>`**: passes data between steps via a oneshot channel. Token-gated retrieval (`into_value(token)`) prevents deadlocks from premature awaiting.
- **`StepResult<T, S>`**: steps return `Result<StepResult<T, S>, S::Error>` with outcomes `Success`/`Warning`/`Skipped`.

### Event buffer (types crate, `buffer/`)

Converts push-based events into pull-based `EventReport`s. Uses a `petgraph::DiGraphMap` to model nested execution topology. Per-step data tracks high-priority events (kept indefinitely) and low-priority events (capped, oldest evicted). `generate_report_since()` supports incremental polling via cursor.

### Display (display crate)

- **`LineDisplay<W>`**: stateful line-oriented writer for incremental formatting from an `EventBuffer`.
- **`GroupDisplay<K, W, S>`**: manages multiple `LineDisplay`s keyed by `K` for concurrent update sources.

## Key design patterns

- **`LinearMap`** for `component_counts` because `S::Component` is only required to be `Eq`, not `Hash` or `Ord` (accommodates `serde_json::Value`).
- **`derive_where`** for conditional derives that avoid requiring bounds on `S` itself.
- **`newtype-uuid`** via `TypedUuid<ExecutionUuidKind>` for `ExecutionUuid`.
- **Backpressure** via oneshot sync channels in `StepContextPayload` variants.
- **`define_update_engine!` macro** generates type aliases for all generic event types parameterized to a specific `StepSpec`.

## Style notes

- `rustfmt.toml` uses `max_width = 80` and `use_small_heuristics = "max"`.
- Rust edition 2024, MSRV 1.85.
- Optional feature: `schemars08` for JSON Schema generation.
