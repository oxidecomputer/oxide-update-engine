// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use camino::Utf8PathBuf;
use oxide_update_engine_types::spec::StepSpec;
use serde::{Deserialize, Serialize};
use std::fmt;

oxide_update_engine_types::define_update_engine!(pub(crate) ExampleSpec);

// Engine-crate type aliases with ExampleSpec as the default.
pub(crate) type UpdateEngine<'a, S = ExampleSpec> =
    oxide_update_engine::UpdateEngine<'a, S>;
pub(crate) type ComponentRegistrar<'engine, 'a, S = ExampleSpec> =
    oxide_update_engine::ComponentRegistrar<'engine, 'a, S>;
pub(crate) type StepContext<S = ExampleSpec> =
    oxide_update_engine::StepContext<S>;
pub(crate) type StepSuccess<T, S = ExampleSpec> =
    oxide_update_engine::StepSuccess<T, S>;
pub(crate) type StepWarning<T, S = ExampleSpec> =
    oxide_update_engine::StepWarning<T, S>;
pub(crate) type StepSkipped<T, S = ExampleSpec> =
    oxide_update_engine::StepSkipped<T, S>;
pub(crate) type StepHandle<T, S = ExampleSpec> =
    oxide_update_engine::StepHandle<T, S>;

/// Create a type to hang the engine specification off of. This is an empty
/// enum (no possible values) because we never construct the type.
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
pub(crate) enum ExampleSpec {}

impl StepSpec for ExampleSpec {
    fn spec_name() -> String {
        "ExampleSpec".to_owned()
    }

    /// This defines a larger component under which a particular set of
    /// steps is grouped.
    type Component = ExampleComponent;

    /// This defines a smaller step ID. It is recommended, but not
    /// enforced, that (component, step ID) form a unique index.
    type StepId = ExampleStepId;

    /// Metadata associated with each step. This becomes part of the
    /// `StepInfo`.
    type StepMetadata = ExampleStepMetadata;

    /// Metadata associated with an individual progress event.
    ///
    /// In this example this is a generic `serde_json::Value`, but it
    /// can be a more concrete type as well.
    type ProgressMetadata = serde_json::Value;

    /// Metadata associated with a completion event.
    type CompletionMetadata = ExampleCompletionMetadata;

    /// Metadata associated with a skipped event.
    ///
    /// In this example there is no metadata attached, so this is the
    /// unit type.
    type SkippedMetadata = ();

    type Error = anyhow::Error;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExampleComponent {
    Component1,
    Component2,
}

impl fmt::Display for ExampleComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Component1 => write!(f, "Component 1"),
            Self::Component2 => write!(f, "Component 2"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExampleStepId {
    Download,
    CreateTempDir,
    Write,
    Skipped,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExampleStepMetadata {
    Write { num_bytes: u64 },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExampleCompletionMetadata {
    Download {
        num_bytes: u64,
    },
    CreateTempDir {
        #[cfg_attr(
            feature = "schemars08",
            schemars(schema_with = "paths_schema")
        )]
        paths: Vec<Utf8PathBuf>,
    },
    Write {
        num_bytes: u64,
        #[cfg_attr(
            feature = "schemars08",
            schemars(schema_with = "paths_schema")
        )]
        destinations: Vec<Utf8PathBuf>,
    },
}

/// A new schema for the write step.
///
/// This is used as a nested step.
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
pub(crate) enum ExampleWriteSpec {}

impl StepSpec for ExampleWriteSpec {
    fn spec_name() -> String {
        "ExampleWriteSpec".to_owned()
    }

    type Component = ExampleComponent;
    type StepId = ExampleWriteStepId;
    type StepMetadata = ();
    type ProgressMetadata = ();
    type CompletionMetadata = ();
    type SkippedMetadata = ();
    type Error = anyhow::Error;
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars08", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case", tag = "kind")]
pub(crate) enum ExampleWriteStepId {
    Write {
        #[cfg_attr(
            feature = "schemars08",
            schemars(schema_with = "path_schema")
        )]
        destination: Utf8PathBuf,
    },
}

#[cfg(feature = "schemars08")]
fn path_schema(
    generator: &mut schemars::r#gen::SchemaGenerator,
) -> schemars::schema::Schema {
    use schemars::JsonSchema;
    let mut schema: schemars::schema::SchemaObject =
        <String>::json_schema(generator).into();
    schema.format = Some("Utf8PathBuf".to_owned());
    schema.into()
}

#[cfg(feature = "schemars08")]
fn paths_schema(
    generator: &mut schemars::r#gen::SchemaGenerator,
) -> schemars::schema::Schema {
    use schemars::JsonSchema;
    let mut schema: schemars::schema::SchemaObject =
        <Vec<String>>::json_schema(generator).into();
    schema.format = Some("Vec<Utf8PathBuf>".to_owned());
    schema.into()
}
