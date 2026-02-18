// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::anyhow;
use indent_write::fmt::IndentWriter;
use serde::{Serialize, de::DeserializeOwned};
use std::{fmt, fmt::Write, marker::PhantomData};

/// A specification for an `UpdateEngine`.
///
/// This defines the set of types required to use an `UpdateEngine`.
pub trait StepSpec: Send + 'static {
    /// The name of this specification, used to identify it in
    /// serialized events.
    fn spec_name() -> String;

    /// A component associated with each step.
    type Component: Clone
        + fmt::Debug
        + DeserializeOwned
        + Serialize
        + Eq
        + Send
        + Sync;

    /// The step identifier.
    type StepId: Clone
        + fmt::Debug
        + DeserializeOwned
        + Serialize
        + Eq
        + Send
        + Sync;

    /// Metadata associated with each step.
    ///
    /// This can be `()` if there's no metadata associated with the
    /// step, or `serde_json::Value` for freeform metadata.
    type StepMetadata: Clone
        + fmt::Debug
        + DeserializeOwned
        + Serialize
        + Eq
        + Send
        + Sync;

    /// Metadata associated with an individual progress event.
    ///
    /// This can be `()` if there's no metadata associated with the
    /// step, or `serde_json::Value` for freeform metadata.
    type ProgressMetadata: Clone
        + fmt::Debug
        + DeserializeOwned
        + Serialize
        + Eq
        + Send
        + Sync;

    /// Metadata associated with each step's completion.
    ///
    /// This can be `()` if there's no metadata associated with the
    /// step, or `serde_json::Value` for freeform metadata.
    type CompletionMetadata: Clone
        + fmt::Debug
        + DeserializeOwned
        + Serialize
        + Eq
        + Send
        + Sync;

    /// Metadata associated with a step being skipped.
    ///
    /// This can be `()` if there's no metadata associated with the
    /// step, or `serde_json::Value` for freeform metadata.
    type SkippedMetadata: Clone
        + fmt::Debug
        + DeserializeOwned
        + Serialize
        + Eq
        + Send
        + Sync;

    /// The error type associated with each step.
    ///
    /// Ideally this would have a trait bound of `std::error::Error`;
    /// however, `anyhow::Error` doesn't implement `std::error::Error`.
    /// Both can be converted to a dynamic `Error`, though. We use
    /// `AsError` to abstract over both sorts of errors.
    type Error: AsError + fmt::Debug + Send + Sync;
}

/// Represents a fully generic step specification, as can be serialized
/// over JSON.
///
/// Since errors aren't directly serialized, they can be any type that
/// implements [`AsError`].
///
/// Use this if you don't care about assigning types to any of the
/// metadata components.
pub struct GenericSpec<E> {
    _marker: PhantomData<E>,
}

// Manual impl: GenericSpec serializes the same way regardless of E,
// so we don't require E: JsonSchema.
#[cfg(feature = "schemars08")]
impl<E> schemars::JsonSchema for GenericSpec<E> {
    fn schema_name() -> String {
        "GenericSpec".to_owned()
    }

    fn json_schema(
        _: &mut schemars::r#gen::SchemaGenerator,
    ) -> schemars::schema::Schema {
        // "Accept any value here" -- GenericSpec only cares about
        // the schema name, not about the shape of E.
        schemars::schema::Schema::Bool(true)
    }
}

impl<E: AsError> StepSpec for GenericSpec<E> {
    fn spec_name() -> String {
        "GenericSpec".to_owned()
    }

    type Component = serde_json::Value;
    type StepId = serde_json::Value;
    type StepMetadata = serde_json::Value;
    type ProgressMetadata = serde_json::Value;
    type CompletionMetadata = serde_json::Value;
    type SkippedMetadata = serde_json::Value;
    type Error = E;
}

/// A generic spec used for nested errors.
pub type NestedSpec = GenericSpec<SerializableError>;

/// A serializable representation of an error chain.
///
/// This is the error type for [`NestedSpec`]. It captures the message
/// and source chain of any `std::error::Error`, enabling errors to be
/// serialized across process or network boundaries.
#[derive(Clone, Debug)]
pub struct SerializableError {
    message: String,
    source: Option<Box<SerializableError>>,
}

impl SerializableError {
    /// Creates a new `SerializableError` from an error.
    pub fn new(error: &dyn std::error::Error) -> Self {
        Self {
            message: format!("{}", error),
            source: error.source().map(|s| Box::new(Self::new(s))),
        }
    }

    /// Creates a new `SerializableError` from a message and a list of
    /// causes.
    pub fn from_message_and_causes(
        message: String,
        causes: Vec<String>,
    ) -> Self {
        // Yes, this is an actual singly-linked list. You rarely ever
        // see them in Rust but they're required to implement
        // Error::source.
        let mut next = None;
        for cause in causes.into_iter().rev() {
            let error = Self { message: cause, source: next.map(Box::new) };
            next = Some(error);
        }
        Self { message, source: next.map(Box::new) }
    }

    /// Returns the message associated with this error.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the causes of this error as an iterator.
    pub fn sources(&self) -> SerializableErrorSources<'_> {
        SerializableErrorSources { current: self.source.as_deref() }
    }
}

impl fmt::Display for SerializableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for SerializableError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|s| s as &(dyn std::error::Error + 'static))
    }
}

/// The sources of a serializable error as an iterator.
#[derive(Debug)]
pub struct SerializableErrorSources<'a> {
    current: Option<&'a SerializableError>,
}

impl<'a> Iterator for SerializableErrorSources<'a> {
    type Item = &'a SerializableError;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = current.source.as_deref();
        Some(current)
    }
}

mod serializable_error_serde {
    use super::*;
    use serde::Deserialize;

    #[derive(Serialize, Deserialize)]
    struct Ser {
        message: String,
        causes: Vec<String>,
    }

    impl Serialize for SerializableError {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            let mut causes = Vec::new();
            let mut cause = self.source.as_ref();
            while let Some(c) = cause {
                causes.push(c.message.clone());
                cause = c.source.as_ref();
            }

            let serialized = Ser { message: self.message.clone(), causes };
            serialized.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for SerializableError {
        fn deserialize<D: serde::Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Self, D::Error> {
            let serialized = Ser::deserialize(deserializer)?;
            Ok(SerializableError::from_message_and_causes(
                serialized.message,
                serialized.causes,
            ))
        }
    }
}

impl AsError for SerializableError {
    fn as_error(&self) -> &(dyn std::error::Error + 'static) {
        self
    }
}

/// Trait that abstracts over concrete errors and `anyhow::Error`.
///
/// This needs to be manually implemented for any custom error types.
pub trait AsError: fmt::Debug + Send + Sync + 'static {
    fn as_error(&self) -> &(dyn std::error::Error + 'static);
}

impl AsError for anyhow::Error {
    fn as_error(&self) -> &(dyn std::error::Error + 'static) {
        self.as_ref()
    }
}

/// A temporary hack to convert a list of anyhow errors into a single
/// `anyhow::Error`. If no errors are provided, panic (this should be
/// handled at a higher level).
///
/// Eventually we should gain first-class support for representing
/// errors as trees, but this will do for now.
pub fn merge_anyhow_list<I>(errors: I) -> anyhow::Error
where
    I: IntoIterator<Item = anyhow::Error>,
{
    let mut iter = errors.into_iter().peekable();
    // How many errors are there?
    let Some(first_error) = iter.next() else {
        // No errors: panic.
        panic!("error_list_to_anyhow called with no errors");
    };

    if iter.peek().is_none() {
        // One error.
        return first_error;
    }

    // Multiple errors.
    let mut out = String::new();
    let mut nerrors = 0;
    for error in std::iter::once(first_error).chain(iter) {
        if nerrors > 0 {
            // Separate errors with a newline (we want there to not
            // be a trailing newline to match anyhow generally).
            writeln!(&mut out).unwrap();
        }
        nerrors += 1;
        let mut current = error.as_error();

        let mut writer = IndentWriter::new_skip_initial("  ", &mut out);
        write!(writer, "Error: {current}").unwrap();

        while let Some(cause) = current.source() {
            // This newline is not part of the `IndentWriter`'s
            // output so that it is unaffected by the indent logic.
            writeln!(&mut out).unwrap();

            // The spaces align the causes with the "Error: " above.
            let mut writer =
                IndentWriter::new_skip_initial("       ", &mut out);
            write!(writer, "     - {cause}").unwrap();
            current = cause;
        }
    }
    anyhow!(out).context(format!("{nerrors} errors encountered"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_merge_anyhow_list() {
        // If the process's environment has `RUST_BACKTRACE=1`, then
        // backtraces get captured and the output doesn't match. As
        // long as we set `RUST_BACKTRACE=0` before the first time a
        // backtrace is captured, we should be fine. Do so at the
        // beginning of this test.
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "0");
        }

        // A single error stays as-is.
        let error = anyhow!("base").context("parent").context("root");

        let merged = merge_anyhow_list(vec![error]);
        assert_eq!(
            format!("{:?}", merged),
            indoc! {"
                root

                Caused by:
                    0: parent
                    1: base"
            },
        );

        // Multiple errors are merged.
        let error1 =
            anyhow!("base1").context("parent1\nparent1 line2").context("root1");
        let error2 = anyhow!("base2").context("parent2").context("root2");

        let merged = merge_anyhow_list(vec![error1, error2]);
        let merged_debug = format!("{:?}", merged);
        println!("merged debug: {}", merged_debug);

        assert_eq!(
            merged_debug,
            indoc! {"
                2 errors encountered

                Caused by:
                    Error: root1
                         - parent1
                           parent1 line2
                         - base1
                    Error: root2
                         - parent2
                         - base2"
            },
        );

        // Ensure that this still looks fine if there's even more
        // context.
        let error3 = merged.context("overall root");
        let error3_debug = format!("{:?}", error3);
        println!("error3 debug: {}", error3_debug);
        assert_eq!(
            error3_debug,
            indoc! {"
                overall root

                Caused by:
                    0: 2 errors encountered
                    1: Error: root1
                            - parent1
                              parent1 line2
                            - base1
                       Error: root2
                            - parent2
                            - base2"
            },
        );
    }
}
