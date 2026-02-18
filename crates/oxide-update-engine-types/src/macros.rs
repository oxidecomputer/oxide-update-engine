// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Defines type aliases for a particular step specification.
///
/// This macro defines a number of type aliases. For example:
///
/// ```ignore
/// oxide_update_engine_types::define_update_engine!(pub(crate) MySpec);
/// ```
///
/// defines a number of type aliases, each of which are of the form:
///
/// ```ignore
/// pub(crate) type Event<S = MySpec> =
///     oxide_update_engine_types::events::Event<S>;
/// pub(crate) type EventBuffer<S = MySpec> =
///     oxide_update_engine_types::buffer::EventBuffer<S>;
/// // ... and so on.
/// ```
///
/// These aliases make it easy to use a type without having to repeat
/// the name of the specification over and over, while still providing
/// a type parameter as an escape hatch if required.
#[macro_export]
macro_rules! define_update_engine {
    ($v:vis $spec_type:ty) => {
        $v type Event<S = $spec_type> =
            $crate::events::Event<S>;
        $v type StepEvent<S = $spec_type> =
            $crate::events::StepEvent<S>;
        $v type StepEventKind<S = $spec_type> =
            $crate::events::StepEventKind<S>;
        $v type ProgressEvent<S = $spec_type> =
            $crate::events::ProgressEvent<S>;
        $v type ProgressEventKind<S = $spec_type> =
            $crate::events::ProgressEventKind<S>;
        $v type StepInfo<S = $spec_type> =
            $crate::events::StepInfo<S>;
        $v type StepComponentSummary<S = $spec_type> =
            $crate::events::StepComponentSummary<S>;
        $v type StepInfoWithMetadata<S = $spec_type> =
            $crate::events::StepInfoWithMetadata<S>;
        $v type StepProgress<S = $spec_type> =
            $crate::events::StepProgress<S>;
        $v type StepOutcome<S = $spec_type> =
            $crate::events::StepOutcome<S>;
        $v type EventReport<S = $spec_type> =
            $crate::events::EventReport<S>;
    };
}
