// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Helpers for building `x-rust-type` JSON Schema extensions.
//!
//! The `x-rust-type` extension tells code generators like typify and
//! progenitor to use an existing Rust type rather than generating a
//! new one. This module provides the shared constants and builder
//! used by the manual `JsonSchema` implementations throughout this
//! crate.

/// Information for the `x-rust-type` JSON Schema extension.
///
/// When an [`EngineSpec`](crate::spec::EngineSpec) provides this,
/// generic types parameterized by that spec will include the
/// `x-rust-type` extension in their JSON Schema.
///
/// This describes only the **spec type** (the generic parameter).
/// The outer types (`StepEvent`, `ProgressEvent`, `EventReport`)
/// always live in `oxide-update-engine-types` and use the module
/// constants directly.
#[derive(Clone, Debug)]
pub struct RustTypeInfo {
    /// The crate that defines the spec type (e.g.
    /// `"oxide-update-engine-types"` for `GenericSpec`, or
    /// `"my-crate"` for a user-defined spec).
    pub crate_name: &'static str,
    /// The version requirement for the spec's crate (e.g. `"*"`).
    pub version: &'static str,
    /// The full path to the spec type (e.g.
    /// `"oxide_update_engine_types::spec::GenericSpec"` or
    /// `"my_crate::MySpec"`).
    pub path: &'static str,
}

/// Crate name used in `x-rust-type` for types in this crate.
pub(crate) const CRATE_NAME: &str = "oxide-update-engine-types";

/// Version requirement used in `x-rust-type` for types in this
/// crate.
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Module path for `events` types in this crate.
#[cfg(feature = "schemars08")]
pub(crate) const EVENTS_MODULE: &str = "oxide_update_engine_types::events";

/// Full path to `GenericSpec` in this crate.
pub(crate) const GENERIC_SPEC_PATH: &str =
    "oxide_update_engine_types::spec::GenericSpec";

/// Attaches a description to a schema, mirroring what the schemars
/// derive does for doc comments on struct fields.
///
/// If the schema is a `$ref`, it is wrapped in `allOf` so that the
/// description can sit alongside the reference (JSON Schema does not
/// allow sibling keywords next to `$ref` in draft-07).
#[cfg(feature = "schemars08")]
pub(crate) fn with_description(
    schema: schemars::schema::Schema,
    description: &str,
) -> schemars::schema::Schema {
    use schemars::schema::{Metadata, Schema, SchemaObject};

    match schema {
        Schema::Object(obj) if obj.reference.is_some() => {
            // Wrap `$ref` in allOf so the description is
            // preserved.
            SchemaObject {
                metadata: Some(Box::new(Metadata {
                    description: Some(description.to_owned()),
                    ..Default::default()
                })),
                subschemas: Some(Box::new(
                    schemars::schema::SubschemaValidation {
                        all_of: Some(vec![Schema::Object(obj)]),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            }
            .into()
        }
        Schema::Object(mut obj) => {
            let metadata = obj.metadata.get_or_insert_with(Default::default);
            metadata.description = Some(description.to_owned());
            obj.into()
        }
        // Schema::Bool (e.g. `true` for unconstrained types like
        // serde_json::Value) cannot carry metadata directly.
        // Wrap it in allOf so we can attach the description,
        // mirroring the $ref case above.
        other => SchemaObject {
            metadata: Some(Box::new(Metadata {
                description: Some(description.to_owned()),
                ..Default::default()
            })),
            subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                all_of: Some(vec![other]),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into(),
    }
}

/// Builds the `x-rust-type` extension value for a type in the
/// `events` module of this crate.
#[cfg(feature = "schemars08")]
pub(crate) fn rust_type_for_events(type_path: &str) -> serde_json::Value {
    serde_json::json!({
        "crate": CRATE_NAME,
        "version": VERSION,
        "path": type_path,
    })
}

/// Builds the `x-rust-type` extension value for a generic type
/// parameterized by a spec (e.g. `StepEvent<GenericSpec>`).
///
/// The outer type (e.g. `StepEvent`) always lives in
/// `oxide-update-engine-types`, so the top-level crate/version/path
/// come from the module constants. The `parameters` array contains
/// an inline schema with its own `x-rust-type` pointing to the
/// spec type, which may live in a different crate.
#[cfg(feature = "schemars08")]
pub(crate) fn rust_type_for_generic(
    info: &RustTypeInfo,
    type_name: &str,
) -> serde_json::Value {
    serde_json::json!({
        "crate": CRATE_NAME,
        "version": VERSION,
        "path": format!(
            "{}::{}",
            EVENTS_MODULE, type_name,
        ),
        "parameters": [
            {
                "x-rust-type": {
                    "crate": info.crate_name,
                    "version": info.version,
                    "path": info.path,
                }
            }
        ],
    })
}

// NOTE: We only add `x-rust-type` to the outermost types such as `EventReport`.
// Ideally, `StepEventKind` and other inner types below would also carry
// `x-rust-type` in their schemas. However, schemars 0.8 does not provide a way
// to intercept or transform a derived schema, so adding `x-rust-type` requires
// a fully manual `JsonSchema` impl. Manual impls are quite fragile, and changes
// to the shape of the type can silently break the schema.
//
// In practice, this is acceptable because these inner types are always accessed
// through the top-level types which *do* carry `x-rust-type`, so typify
// replaces the entire type tree.
