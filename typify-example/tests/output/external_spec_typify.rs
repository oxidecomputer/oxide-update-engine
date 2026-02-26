/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`Duration`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "nanos",
///    "secs"
///  ],
///  "properties": {
///    "nanos": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "secs": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Duration {
    pub nanos: u32,
    pub secs: u64,
}
/**A wrapper struct that references `EventReport<ExternalSpec>`.

The outer types should resolve to `::oxide_update_engine_types::...` while the parameter should resolve to `::typify_example::ExternalSpec`.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ExternalWrapper",
///  "description": "A wrapper struct that references `EventReport<ExternalSpec>`.\n\nThe outer types should resolve to `::oxide_update_engine_types::...` while the parameter should resolve to `::typify_example::ExternalSpec`.",
///  "type": "object",
///  "required": [
///    "report"
///  ],
///  "properties": {
///    "report": {
///      "$ref": "#/definitions/EventReportForExternalSpec"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExternalWrapper {
    pub report: ::oxide_update_engine_types::events::EventReport<
        ::typify_example::ExternalSpec,
    >,
}
///`ProgressEventKindForExternalSpec`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "The update engine is waiting for a progress message.\n\nThe update engine sends this message immediately after a [`StepEvent`] corresponding to a new step.",
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The attempt number currently being executed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "Total time elapsed since the start of the attempt.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "waiting_for_progress"
///          ]
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "metadata",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The attempt number currently being executed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "Total time elapsed since the start of the attempt.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "progress"
///          ]
///        },
///        "metadata": {
///          "description": "Metadata that was returned with progress."
///        },
///        "progress": {
///          "description": "Current progress.",
///          "anyOf": [
///            {
///              "$ref": "#/definitions/ProgressCounter"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "event",
///        "kind",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The attempt number currently being executed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "event": {
///          "description": "The event that occurred.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/ProgressEventForGenericSpec"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "nested"
///          ]
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "Future variants that might be unknown.",
///      "type": "object",
///      "required": [
///        "kind"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "unknown"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "kind")]
pub enum ProgressEventKindForExternalSpec {
    /**The update engine is waiting for a progress message.

The update engine sends this message immediately after a [`StepEvent`] corresponding to a new step.*/
    #[serde(rename = "waiting_for_progress")]
    WaitingForProgress {
        ///The attempt number currently being executed.
        attempt: u32,
        ///Total time elapsed since the start of the attempt.
        attempt_elapsed: Duration,
        ///Information about the step.
        step: StepInfoWithMetadataForExternalSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    #[serde(rename = "progress")]
    Progress {
        ///The attempt number currently being executed.
        attempt: u32,
        ///Total time elapsed since the start of the attempt.
        attempt_elapsed: Duration,
        ///Metadata that was returned with progress.
        metadata: ::serde_json::Value,
        ///Current progress.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        progress: ::std::option::Option<
            ::oxide_update_engine_types::events::ProgressCounter,
        >,
        ///Information about the step.
        step: StepInfoWithMetadataForExternalSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    #[serde(rename = "nested")]
    Nested {
        ///The attempt number currently being executed.
        attempt: u32,
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///The event that occurred.
        event: ::oxide_update_engine_types::events::ProgressEvent<
            ::oxide_update_engine_types::spec::GenericSpec,
        >,
        ///Information about the step.
        step: StepInfoWithMetadataForExternalSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    #[serde(rename = "unknown")]
    Unknown,
}
///`ProgressEventKindForGenericSpec`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "The update engine is waiting for a progress message.\n\nThe update engine sends this message immediately after a [`StepEvent`] corresponding to a new step.",
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The attempt number currently being executed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "Total time elapsed since the start of the attempt.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "waiting_for_progress"
///          ]
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "metadata",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The attempt number currently being executed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "Total time elapsed since the start of the attempt.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "progress"
///          ]
///        },
///        "metadata": {
///          "description": "Metadata that was returned with progress."
///        },
///        "progress": {
///          "description": "Current progress.",
///          "anyOf": [
///            {
///              "$ref": "#/definitions/ProgressCounter"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "event",
///        "kind",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The attempt number currently being executed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "event": {
///          "description": "The event that occurred.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/ProgressEventForGenericSpec"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "nested"
///          ]
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "Future variants that might be unknown.",
///      "type": "object",
///      "required": [
///        "kind"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "unknown"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "kind")]
pub enum ProgressEventKindForGenericSpec {
    /**The update engine is waiting for a progress message.

The update engine sends this message immediately after a [`StepEvent`] corresponding to a new step.*/
    #[serde(rename = "waiting_for_progress")]
    WaitingForProgress {
        ///The attempt number currently being executed.
        attempt: u32,
        ///Total time elapsed since the start of the attempt.
        attempt_elapsed: Duration,
        ///Information about the step.
        step: StepInfoWithMetadataForGenericSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    #[serde(rename = "progress")]
    Progress {
        ///The attempt number currently being executed.
        attempt: u32,
        ///Total time elapsed since the start of the attempt.
        attempt_elapsed: Duration,
        ///Metadata that was returned with progress.
        metadata: ::serde_json::Value,
        ///Current progress.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        progress: ::std::option::Option<
            ::oxide_update_engine_types::events::ProgressCounter,
        >,
        ///Information about the step.
        step: StepInfoWithMetadataForGenericSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    #[serde(rename = "nested")]
    Nested {
        ///The attempt number currently being executed.
        attempt: u32,
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///The event that occurred.
        event: ::oxide_update_engine_types::events::ProgressEvent<
            ::oxide_update_engine_types::spec::GenericSpec,
        >,
        ///Information about the step.
        step: StepInfoWithMetadataForGenericSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    #[serde(rename = "unknown")]
    Unknown,
}
///`StepComponentSummaryForExternalSpec`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "component",
///    "total_component_steps"
///  ],
///  "properties": {
///    "component": {
///      "description": "The component."
///    },
///    "total_component_steps": {
///      "description": "The number of steps present in this component.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StepComponentSummaryForExternalSpec {
    ///The component.
    pub component: ::serde_json::Value,
    ///The number of steps present in this component.
    pub total_component_steps: u32,
}
///`StepComponentSummaryForGenericSpec`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "component",
///    "total_component_steps"
///  ],
///  "properties": {
///    "component": {
///      "description": "The component."
///    },
///    "total_component_steps": {
///      "description": "The number of steps present in this component.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StepComponentSummaryForGenericSpec {
    ///The component.
    pub component: ::serde_json::Value,
    ///The number of steps present in this component.
    pub total_component_steps: u32,
}
///`StepEventKindForExternalSpec`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "No steps were defined, and the executor exited without doing anything.\n\nThis is a terminal event: it is guaranteed that no more events will be seen after this one.",
///      "type": "object",
///      "required": [
///        "kind"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "no_steps_defined"
///          ]
///        }
///      }
///    },
///    {
///      "description": "Execution was started.\n\nThis is an initial event -- it is always expected to be the first event received from the event stream.",
///      "type": "object",
///      "required": [
///        "components",
///        "first_step",
///        "kind",
///        "steps"
///      ],
///      "properties": {
///        "components": {
///          "description": "A list of components, along with the number of items each component has.",
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/StepComponentSummaryForExternalSpec"
///          }
///        },
///        "first_step": {
///          "description": "Information about the first step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "execution_started"
///          ]
///        },
///        "steps": {
///          "description": "The list of steps that will be executed.",
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/StepInfoForExternalSpec"
///          }
///        }
///      }
///    },
///    {
///      "description": "Progress was reset along an attempt, and this attempt is going down a different path.",
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "message",
///        "metadata",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The current attempt number.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The amount of time this attempt has taken so far.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "progress_reset"
///          ]
///        },
///        "message": {
///          "description": "A message associated with the reset.",
///          "type": "string"
///        },
///        "metadata": {
///          "description": "Progress-related metadata associated with this attempt."
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "An attempt failed and this step is being retried.",
///      "type": "object",
///      "required": [
///        "attempt_elapsed",
///        "kind",
///        "message",
///        "next_attempt",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt_elapsed": {
///          "description": "The amount of time the previous attempt took.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "attempt_retry"
///          ]
///        },
///        "message": {
///          "description": "A message associated with the retry.",
///          "type": "string"
///        },
///        "next_attempt": {
///          "description": "The attempt number for the next attempt.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "A step is complete and the next step has been started.",
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "next_step",
///        "outcome",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The attempt number that completed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "step_completed"
///          ]
///        },
///        "next_step": {
///          "description": "The next step that is being started.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "outcome": {
///          "description": "The outcome of the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepOutcomeForExternalSpec"
///            }
///          ]
///        },
///        "step": {
///          "description": "Information about the step that just completed.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "Execution is complete.\n\nThis is a terminal event: it is guaranteed that no more events will be seen after this one.",
///      "type": "object",
///      "required": [
///        "attempt_elapsed",
///        "kind",
///        "last_attempt",
///        "last_outcome",
///        "last_step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "execution_completed"
///          ]
///        },
///        "last_attempt": {
///          "description": "The attempt number that completed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "last_outcome": {
///          "description": "The outcome of the last step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepOutcomeForExternalSpec"
///            }
///          ]
///        },
///        "last_step": {
///          "description": "Information about the last step that completed.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "Execution failed.\n\nThis is a terminal event: it is guaranteed that no more events will be seen after this one.",
///      "type": "object",
///      "required": [
///        "attempt_elapsed",
///        "causes",
///        "failed_step",
///        "kind",
///        "message",
///        "step_elapsed",
///        "total_attempts"
///      ],
///      "properties": {
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "causes": {
///          "description": "A chain of causes associated with the failure.",
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        "failed_step": {
///          "description": "Information about the step that failed.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "execution_failed"
///          ]
///        },
///        "message": {
///          "description": "A message associated with the failure.",
///          "type": "string"
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "total_attempts": {
///          "description": "The total number of attempts that were performed before the step failed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        }
///      }
///    },
///    {
///      "description": "Execution aborted by an external user.\n\nThis is a terminal event: it is guaranteed that no more events will be seen after this one.",
///      "type": "object",
///      "required": [
///        "aborted_step",
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "message",
///        "step_elapsed"
///      ],
///      "properties": {
///        "aborted_step": {
///          "description": "Information about the step that was running at the time execution was aborted.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "attempt": {
///          "description": "The attempt that was running at the time the step was aborted.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "execution_aborted"
///          ]
///        },
///        "message": {
///          "description": "A message associated with the abort.",
///          "type": "string"
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "A nested step event occurred.",
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "event",
///        "kind",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The current attempt number.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "event": {
///          "description": "The event that occurred.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepEventForGenericSpec"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "nested"
///          ]
///        },
///        "step": {
///          "description": "Information about the step that's occurring.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForExternalSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "Future variants that might be unknown.",
///      "type": "object",
///      "required": [
///        "kind"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "unknown"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "kind")]
pub enum StepEventKindForExternalSpec {
    #[serde(rename = "no_steps_defined")]
    NoStepsDefined,
    /**Execution was started.

This is an initial event -- it is always expected to be the first event received from the event stream.*/
    #[serde(rename = "execution_started")]
    ExecutionStarted {
        ///A list of components, along with the number of items each component has.
        components: ::std::vec::Vec<StepComponentSummaryForExternalSpec>,
        ///Information about the first step.
        first_step: StepInfoWithMetadataForExternalSpec,
        ///The list of steps that will be executed.
        steps: ::std::vec::Vec<StepInfoForExternalSpec>,
    },
    ///Progress was reset along an attempt, and this attempt is going down a different path.
    #[serde(rename = "progress_reset")]
    ProgressReset {
        ///The current attempt number.
        attempt: u32,
        ///The amount of time this attempt has taken so far.
        attempt_elapsed: Duration,
        ///A message associated with the reset.
        message: ::std::string::String,
        ///Progress-related metadata associated with this attempt.
        metadata: ::serde_json::Value,
        ///Information about the step.
        step: StepInfoWithMetadataForExternalSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    ///An attempt failed and this step is being retried.
    #[serde(rename = "attempt_retry")]
    AttemptRetry {
        ///The amount of time the previous attempt took.
        attempt_elapsed: Duration,
        ///A message associated with the retry.
        message: ::std::string::String,
        ///The attempt number for the next attempt.
        next_attempt: u32,
        ///Information about the step.
        step: StepInfoWithMetadataForExternalSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    ///A step is complete and the next step has been started.
    #[serde(rename = "step_completed")]
    StepCompleted {
        ///The attempt number that completed.
        attempt: u32,
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///The next step that is being started.
        next_step: StepInfoWithMetadataForExternalSpec,
        ///The outcome of the step.
        outcome: StepOutcomeForExternalSpec,
        ///Information about the step that just completed.
        step: StepInfoWithMetadataForExternalSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    /**Execution is complete.

This is a terminal event: it is guaranteed that no more events will be seen after this one.*/
    #[serde(rename = "execution_completed")]
    ExecutionCompleted {
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///The attempt number that completed.
        last_attempt: u32,
        ///The outcome of the last step.
        last_outcome: StepOutcomeForExternalSpec,
        ///Information about the last step that completed.
        last_step: StepInfoWithMetadataForExternalSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    /**Execution failed.

This is a terminal event: it is guaranteed that no more events will be seen after this one.*/
    #[serde(rename = "execution_failed")]
    ExecutionFailed {
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///A chain of causes associated with the failure.
        causes: ::std::vec::Vec<::std::string::String>,
        ///Information about the step that failed.
        failed_step: StepInfoWithMetadataForExternalSpec,
        ///A message associated with the failure.
        message: ::std::string::String,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
        ///The total number of attempts that were performed before the step failed.
        total_attempts: u32,
    },
    /**Execution aborted by an external user.

This is a terminal event: it is guaranteed that no more events will be seen after this one.*/
    #[serde(rename = "execution_aborted")]
    ExecutionAborted {
        ///Information about the step that was running at the time execution was aborted.
        aborted_step: StepInfoWithMetadataForExternalSpec,
        ///The attempt that was running at the time the step was aborted.
        attempt: u32,
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///A message associated with the abort.
        message: ::std::string::String,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    ///A nested step event occurred.
    #[serde(rename = "nested")]
    Nested {
        ///The current attempt number.
        attempt: u32,
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///The event that occurred.
        event: ::oxide_update_engine_types::events::StepEvent<
            ::oxide_update_engine_types::spec::GenericSpec,
        >,
        ///Information about the step that's occurring.
        step: StepInfoWithMetadataForExternalSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    #[serde(rename = "unknown")]
    Unknown,
}
///`StepEventKindForGenericSpec`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "No steps were defined, and the executor exited without doing anything.\n\nThis is a terminal event: it is guaranteed that no more events will be seen after this one.",
///      "type": "object",
///      "required": [
///        "kind"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "no_steps_defined"
///          ]
///        }
///      }
///    },
///    {
///      "description": "Execution was started.\n\nThis is an initial event -- it is always expected to be the first event received from the event stream.",
///      "type": "object",
///      "required": [
///        "components",
///        "first_step",
///        "kind",
///        "steps"
///      ],
///      "properties": {
///        "components": {
///          "description": "A list of components, along with the number of items each component has.",
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/StepComponentSummaryForGenericSpec"
///          }
///        },
///        "first_step": {
///          "description": "Information about the first step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "execution_started"
///          ]
///        },
///        "steps": {
///          "description": "The list of steps that will be executed.",
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/StepInfoForGenericSpec"
///          }
///        }
///      }
///    },
///    {
///      "description": "Progress was reset along an attempt, and this attempt is going down a different path.",
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "message",
///        "metadata",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The current attempt number.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The amount of time this attempt has taken so far.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "progress_reset"
///          ]
///        },
///        "message": {
///          "description": "A message associated with the reset.",
///          "type": "string"
///        },
///        "metadata": {
///          "description": "Progress-related metadata associated with this attempt."
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "An attempt failed and this step is being retried.",
///      "type": "object",
///      "required": [
///        "attempt_elapsed",
///        "kind",
///        "message",
///        "next_attempt",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt_elapsed": {
///          "description": "The amount of time the previous attempt took.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "attempt_retry"
///          ]
///        },
///        "message": {
///          "description": "A message associated with the retry.",
///          "type": "string"
///        },
///        "next_attempt": {
///          "description": "The attempt number for the next attempt.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "step": {
///          "description": "Information about the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "A step is complete and the next step has been started.",
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "next_step",
///        "outcome",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The attempt number that completed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "step_completed"
///          ]
///        },
///        "next_step": {
///          "description": "The next step that is being started.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "outcome": {
///          "description": "The outcome of the step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepOutcomeForGenericSpec"
///            }
///          ]
///        },
///        "step": {
///          "description": "Information about the step that just completed.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "Execution is complete.\n\nThis is a terminal event: it is guaranteed that no more events will be seen after this one.",
///      "type": "object",
///      "required": [
///        "attempt_elapsed",
///        "kind",
///        "last_attempt",
///        "last_outcome",
///        "last_step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "execution_completed"
///          ]
///        },
///        "last_attempt": {
///          "description": "The attempt number that completed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "last_outcome": {
///          "description": "The outcome of the last step.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepOutcomeForGenericSpec"
///            }
///          ]
///        },
///        "last_step": {
///          "description": "Information about the last step that completed.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "Execution failed.\n\nThis is a terminal event: it is guaranteed that no more events will be seen after this one.",
///      "type": "object",
///      "required": [
///        "attempt_elapsed",
///        "causes",
///        "failed_step",
///        "kind",
///        "message",
///        "step_elapsed",
///        "total_attempts"
///      ],
///      "properties": {
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "causes": {
///          "description": "A chain of causes associated with the failure.",
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        "failed_step": {
///          "description": "Information about the step that failed.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "execution_failed"
///          ]
///        },
///        "message": {
///          "description": "A message associated with the failure.",
///          "type": "string"
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "total_attempts": {
///          "description": "The total number of attempts that were performed before the step failed.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        }
///      }
///    },
///    {
///      "description": "Execution aborted by an external user.\n\nThis is a terminal event: it is guaranteed that no more events will be seen after this one.",
///      "type": "object",
///      "required": [
///        "aborted_step",
///        "attempt",
///        "attempt_elapsed",
///        "kind",
///        "message",
///        "step_elapsed"
///      ],
///      "properties": {
///        "aborted_step": {
///          "description": "Information about the step that was running at the time execution was aborted.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "attempt": {
///          "description": "The attempt that was running at the time the step was aborted.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "execution_aborted"
///          ]
///        },
///        "message": {
///          "description": "A message associated with the abort.",
///          "type": "string"
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "A nested step event occurred.",
///      "type": "object",
///      "required": [
///        "attempt",
///        "attempt_elapsed",
///        "event",
///        "kind",
///        "step",
///        "step_elapsed"
///      ],
///      "properties": {
///        "attempt": {
///          "description": "The current attempt number.",
///          "type": "integer",
///          "format": "uint",
///          "minimum": 0.0
///        },
///        "attempt_elapsed": {
///          "description": "The time it took for this attempt to complete.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        },
///        "event": {
///          "description": "The event that occurred.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepEventForGenericSpec"
///            }
///          ]
///        },
///        "kind": {
///          "type": "string",
///          "enum": [
///            "nested"
///          ]
///        },
///        "step": {
///          "description": "Information about the step that's occurring.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/StepInfoWithMetadataForGenericSpec"
///            }
///          ]
///        },
///        "step_elapsed": {
///          "description": "Total time elapsed since the start of the step. Includes prior attempts.",
///          "allOf": [
///            {
///              "$ref": "#/definitions/Duration"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "description": "Future variants that might be unknown.",
///      "type": "object",
///      "required": [
///        "kind"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "unknown"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "kind")]
pub enum StepEventKindForGenericSpec {
    #[serde(rename = "no_steps_defined")]
    NoStepsDefined,
    /**Execution was started.

This is an initial event -- it is always expected to be the first event received from the event stream.*/
    #[serde(rename = "execution_started")]
    ExecutionStarted {
        ///A list of components, along with the number of items each component has.
        components: ::std::vec::Vec<StepComponentSummaryForGenericSpec>,
        ///Information about the first step.
        first_step: StepInfoWithMetadataForGenericSpec,
        ///The list of steps that will be executed.
        steps: ::std::vec::Vec<StepInfoForGenericSpec>,
    },
    ///Progress was reset along an attempt, and this attempt is going down a different path.
    #[serde(rename = "progress_reset")]
    ProgressReset {
        ///The current attempt number.
        attempt: u32,
        ///The amount of time this attempt has taken so far.
        attempt_elapsed: Duration,
        ///A message associated with the reset.
        message: ::std::string::String,
        ///Progress-related metadata associated with this attempt.
        metadata: ::serde_json::Value,
        ///Information about the step.
        step: StepInfoWithMetadataForGenericSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    ///An attempt failed and this step is being retried.
    #[serde(rename = "attempt_retry")]
    AttemptRetry {
        ///The amount of time the previous attempt took.
        attempt_elapsed: Duration,
        ///A message associated with the retry.
        message: ::std::string::String,
        ///The attempt number for the next attempt.
        next_attempt: u32,
        ///Information about the step.
        step: StepInfoWithMetadataForGenericSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    ///A step is complete and the next step has been started.
    #[serde(rename = "step_completed")]
    StepCompleted {
        ///The attempt number that completed.
        attempt: u32,
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///The next step that is being started.
        next_step: StepInfoWithMetadataForGenericSpec,
        ///The outcome of the step.
        outcome: StepOutcomeForGenericSpec,
        ///Information about the step that just completed.
        step: StepInfoWithMetadataForGenericSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    /**Execution is complete.

This is a terminal event: it is guaranteed that no more events will be seen after this one.*/
    #[serde(rename = "execution_completed")]
    ExecutionCompleted {
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///The attempt number that completed.
        last_attempt: u32,
        ///The outcome of the last step.
        last_outcome: StepOutcomeForGenericSpec,
        ///Information about the last step that completed.
        last_step: StepInfoWithMetadataForGenericSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    /**Execution failed.

This is a terminal event: it is guaranteed that no more events will be seen after this one.*/
    #[serde(rename = "execution_failed")]
    ExecutionFailed {
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///A chain of causes associated with the failure.
        causes: ::std::vec::Vec<::std::string::String>,
        ///Information about the step that failed.
        failed_step: StepInfoWithMetadataForGenericSpec,
        ///A message associated with the failure.
        message: ::std::string::String,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
        ///The total number of attempts that were performed before the step failed.
        total_attempts: u32,
    },
    /**Execution aborted by an external user.

This is a terminal event: it is guaranteed that no more events will be seen after this one.*/
    #[serde(rename = "execution_aborted")]
    ExecutionAborted {
        ///Information about the step that was running at the time execution was aborted.
        aborted_step: StepInfoWithMetadataForGenericSpec,
        ///The attempt that was running at the time the step was aborted.
        attempt: u32,
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///A message associated with the abort.
        message: ::std::string::String,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    ///A nested step event occurred.
    #[serde(rename = "nested")]
    Nested {
        ///The current attempt number.
        attempt: u32,
        ///The time it took for this attempt to complete.
        attempt_elapsed: Duration,
        ///The event that occurred.
        event: ::oxide_update_engine_types::events::StepEvent<
            ::oxide_update_engine_types::spec::GenericSpec,
        >,
        ///Information about the step that's occurring.
        step: StepInfoWithMetadataForGenericSpec,
        ///Total time elapsed since the start of the step. Includes prior attempts.
        step_elapsed: Duration,
    },
    #[serde(rename = "unknown")]
    Unknown,
}
///Serializable information about a step.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Serializable information about a step.",
///  "type": "object",
///  "required": [
///    "component",
///    "component_index",
///    "description",
///    "id",
///    "index",
///    "total_component_steps"
///  ],
///  "properties": {
///    "component": {
///      "description": "The component that this step is part of."
///    },
///    "component_index": {
///      "description": "The index of the step within the component.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "description": {
///      "description": "The description for this step.",
///      "type": "string"
///    },
///    "id": {
///      "description": "An identifier for this step."
///    },
///    "index": {
///      "description": "The index of the step within all steps to be executed.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "total_component_steps": {
///      "description": "The total number of steps in this component.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StepInfoForExternalSpec {
    ///The component that this step is part of.
    pub component: ::serde_json::Value,
    ///The index of the step within the component.
    pub component_index: u32,
    ///The description for this step.
    pub description: ::std::string::String,
    ///An identifier for this step.
    pub id: ::serde_json::Value,
    ///The index of the step within all steps to be executed.
    pub index: u32,
    ///The total number of steps in this component.
    pub total_component_steps: u32,
}
///Serializable information about a step.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Serializable information about a step.",
///  "type": "object",
///  "required": [
///    "component",
///    "component_index",
///    "description",
///    "id",
///    "index",
///    "total_component_steps"
///  ],
///  "properties": {
///    "component": {
///      "description": "The component that this step is part of."
///    },
///    "component_index": {
///      "description": "The index of the step within the component.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "description": {
///      "description": "The description for this step.",
///      "type": "string"
///    },
///    "id": {
///      "description": "An identifier for this step."
///    },
///    "index": {
///      "description": "The index of the step within all steps to be executed.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "total_component_steps": {
///      "description": "The total number of steps in this component.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StepInfoForGenericSpec {
    ///The component that this step is part of.
    pub component: ::serde_json::Value,
    ///The index of the step within the component.
    pub component_index: u32,
    ///The description for this step.
    pub description: ::std::string::String,
    ///An identifier for this step.
    pub id: ::serde_json::Value,
    ///The index of the step within all steps to be executed.
    pub index: u32,
    ///The total number of steps in this component.
    pub total_component_steps: u32,
}
///Serializable information about a step.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Serializable information about a step.",
///  "type": "object",
///  "required": [
///    "info"
///  ],
///  "properties": {
///    "info": {
///      "description": "Information about this step.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/StepInfoForExternalSpec"
///        }
///      ]
///    },
///    "metadata": {
///      "description": "Additional metadata associated with this step."
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StepInfoWithMetadataForExternalSpec {
    ///Information about this step.
    pub info: StepInfoForExternalSpec,
    ///Additional metadata associated with this step.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::serde_json::Value>,
}
///Serializable information about a step.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Serializable information about a step.",
///  "type": "object",
///  "required": [
///    "info"
///  ],
///  "properties": {
///    "info": {
///      "description": "Information about this step.",
///      "allOf": [
///        {
///          "$ref": "#/definitions/StepInfoForGenericSpec"
///        }
///      ]
///    },
///    "metadata": {
///      "description": "Additional metadata associated with this step."
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StepInfoWithMetadataForGenericSpec {
    ///Information about this step.
    pub info: StepInfoForGenericSpec,
    ///Additional metadata associated with this step.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::serde_json::Value>,
}
///`StepOutcomeForExternalSpec`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "The step completed successfully.",
///      "type": "object",
///      "required": [
///        "kind"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "success"
///          ]
///        },
///        "message": {
///          "description": "An optional message associated with this step.",
///          "type": [
///            "string",
///            "null"
///          ]
///        },
///        "metadata": {
///          "description": "Optional completion metadata associated with the step."
///        }
///      }
///    },
///    {
///      "description": "The step completed with a warning.",
///      "type": "object",
///      "required": [
///        "kind",
///        "message"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "warning"
///          ]
///        },
///        "message": {
///          "description": "A warning message.",
///          "type": "string"
///        },
///        "metadata": {
///          "description": "Optional completion metadata associated with the step."
///        }
///      }
///    },
///    {
///      "description": "The step was skipped with a message.",
///      "type": "object",
///      "required": [
///        "kind",
///        "message"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "skipped"
///          ]
///        },
///        "message": {
///          "description": "Message associated with the skip.",
///          "type": "string"
///        },
///        "metadata": {
///          "description": "Optional metadata associated with the skip."
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "kind")]
pub enum StepOutcomeForExternalSpec {
    ///The step completed successfully.
    #[serde(rename = "success")]
    Success {
        ///An optional message associated with this step.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        message: ::std::option::Option<::std::string::String>,
        ///Optional completion metadata associated with the step.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        metadata: ::std::option::Option<::serde_json::Value>,
    },
    ///The step completed with a warning.
    #[serde(rename = "warning")]
    Warning {
        ///A warning message.
        message: ::std::string::String,
        ///Optional completion metadata associated with the step.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        metadata: ::std::option::Option<::serde_json::Value>,
    },
    ///The step was skipped with a message.
    #[serde(rename = "skipped")]
    Skipped {
        ///Message associated with the skip.
        message: ::std::string::String,
        ///Optional metadata associated with the skip.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        metadata: ::std::option::Option<::serde_json::Value>,
    },
}
///`StepOutcomeForGenericSpec`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "The step completed successfully.",
///      "type": "object",
///      "required": [
///        "kind"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "success"
///          ]
///        },
///        "message": {
///          "description": "An optional message associated with this step.",
///          "type": [
///            "string",
///            "null"
///          ]
///        },
///        "metadata": {
///          "description": "Optional completion metadata associated with the step."
///        }
///      }
///    },
///    {
///      "description": "The step completed with a warning.",
///      "type": "object",
///      "required": [
///        "kind",
///        "message"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "warning"
///          ]
///        },
///        "message": {
///          "description": "A warning message.",
///          "type": "string"
///        },
///        "metadata": {
///          "description": "Optional completion metadata associated with the step."
///        }
///      }
///    },
///    {
///      "description": "The step was skipped with a message.",
///      "type": "object",
///      "required": [
///        "kind",
///        "message"
///      ],
///      "properties": {
///        "kind": {
///          "type": "string",
///          "enum": [
///            "skipped"
///          ]
///        },
///        "message": {
///          "description": "Message associated with the skip.",
///          "type": "string"
///        },
///        "metadata": {
///          "description": "Optional metadata associated with the skip."
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "kind")]
pub enum StepOutcomeForGenericSpec {
    ///The step completed successfully.
    #[serde(rename = "success")]
    Success {
        ///An optional message associated with this step.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        message: ::std::option::Option<::std::string::String>,
        ///Optional completion metadata associated with the step.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        metadata: ::std::option::Option<::serde_json::Value>,
    },
    ///The step completed with a warning.
    #[serde(rename = "warning")]
    Warning {
        ///A warning message.
        message: ::std::string::String,
        ///Optional completion metadata associated with the step.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        metadata: ::std::option::Option<::serde_json::Value>,
    },
    ///The step was skipped with a message.
    #[serde(rename = "skipped")]
    Skipped {
        ///Message associated with the skip.
        message: ::std::string::String,
        ///Optional metadata associated with the skip.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        metadata: ::std::option::Option<::serde_json::Value>,
    },
}
