// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Defines type aliases for engine-crate types parameterized by a
/// particular [`EngineSpec`](crate::types::spec::EngineSpec).
///
/// This macro defines a number of type aliases. For example:
///
/// ```ignore
/// oxide_update_engine::define_update_engine!(pub(crate) MySpec);
/// ```
///
/// defines a number of type aliases, each of which are of the form:
///
/// ```ignore
/// pub(crate) type UpdateEngine<'a, S = MySpec> =
///     oxide_update_engine::UpdateEngine<'a, S>;
/// pub(crate) type StepContext<S = MySpec> =
///     oxide_update_engine::StepContext<S>;
/// // ... and so on.
/// ```
///
/// For types-crate aliases (events, buffers, etc.), use the companion
/// macro
/// [`oxide_update_engine_types::define_update_engine_types!`](https://docs.rs/oxide-update-engine-types).
#[macro_export]
macro_rules! define_update_engine {
    ($v:vis $spec_type:ty) => {
        $v type UpdateEngine<'a, S = $spec_type> =
            $crate::UpdateEngine<'a, S>;
        $v type ComponentRegistrar<'engine, 'a, S = $spec_type> =
            $crate::ComponentRegistrar<'engine, 'a, S>;
        $v type StepContext<S = $spec_type> =
            $crate::StepContext<S>;
        $v type StepHandle<T, S = $spec_type> =
            $crate::StepHandle<T, S>;
        $v type SharedStepHandle<T, S = $spec_type> =
            $crate::SharedStepHandle<T, S>;
        $v type StepResult<T, S = $spec_type> =
            $crate::StepResult<T, S>;
        $v type StepSuccess<T, S = $spec_type> =
            $crate::StepSuccess<T, S>;
        $v type StepWarning<T, S = $spec_type> =
            $crate::StepWarning<T, S>;
        $v type StepSkipped<T, S = $spec_type> =
            $crate::StepSkipped<T, S>;
        $v type ExecutionError<S = $spec_type> =
            $crate::ExecutionError<S>;
    };
}
