// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Verification that typify correctly resolves `x-rust-type`
//! annotations from `oxide-update-engine-types` schemas.
//!
//! The test defines a wrapper struct that *contains* an
//! `EventReport<GenericSpec>`. When typify processes the wrapper's
//! schema, it should replace `EventReport` (and its transitive
//! dependencies like `StepEvent`, `ProgressEvent`, `ProgressCounter`,
//! etc.) with references to the real types via `x-rust-type`, rather
//! than generating new struct definitions for them.

#[cfg(test)]
mod tests {
    use expectorate::assert_contents;
    use oxide_update_engine_types::{
        events::EventReport,
        schema::RustTypeInfo,
        spec::{EngineSpec, GenericSpec},
    };

    /// A wrapper struct that references `EventReport<GenericSpec>`.
    ///
    /// Typify will generate this struct, but should use the real
    /// `EventReport` type (and its transitive dependencies) for
    /// the `report` field via `x-rust-type` replacement.
    #[derive(schemars::JsonSchema)]
    #[expect(dead_code)]
    struct Wrapper {
        report: EventReport<GenericSpec>,
    }

    #[test]
    fn typify_resolves_event_report_schema() {
        let schema = schemars::schema_for!(Wrapper);

        let mut settings = typify::TypeSpaceSettings::default();
        settings.with_crate(
            "oxide-update-engine-types",
            typify::CrateVers::Any,
            None,
        );

        let mut type_space = typify::TypeSpace::new(&settings);
        type_space.add_root_schema(schema).expect("added root schema");

        let code = type_space.to_stream();
        let file =
            syn::parse2::<syn::File>(code).expect("parsed generated code");
        let formatted = prettyplease::unparse(&file);

        assert_contents("tests/output/event_report_typify.rs", &formatted);
    }

    // -- External (user-defined) spec --

    /// A spec type simulating a user-defined spec in an external
    /// crate. Its `rust_type_info()` points to `"typify-example"`
    /// rather than `"oxide-update-engine-types"`.
    enum ExternalSpec {}

    impl EngineSpec for ExternalSpec {
        fn spec_name() -> String {
            "ExternalSpec".to_owned()
        }

        type Component = serde_json::Value;
        type StepId = serde_json::Value;
        type StepMetadata = serde_json::Value;
        type ProgressMetadata = serde_json::Value;
        type CompletionMetadata = serde_json::Value;
        type SkippedMetadata = serde_json::Value;
        type Error = anyhow::Error;

        fn rust_type_info() -> Option<RustTypeInfo> {
            Some(RustTypeInfo {
                crate_name: "typify-example",
                version: "*",
                path: "typify_example::ExternalSpec",
            })
        }
    }

    impl schemars::JsonSchema for ExternalSpec {
        fn schema_name() -> String {
            "ExternalSpec".to_owned()
        }

        fn json_schema(
            _: &mut schemars::r#gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            schemars::schema::Schema::Bool(true)
        }
    }

    /// A wrapper struct that references
    /// `EventReport<ExternalSpec>`.
    ///
    /// The outer types should resolve to
    /// `::oxide_update_engine_types::...` while the parameter
    /// should resolve to `::typify_example::ExternalSpec`.
    #[derive(schemars::JsonSchema)]
    #[expect(dead_code)]
    struct ExternalWrapper {
        report: EventReport<ExternalSpec>,
    }

    #[test]
    fn typify_resolves_external_spec_schema() {
        let schema = schemars::schema_for!(ExternalWrapper);

        let mut settings = typify::TypeSpaceSettings::default();
        settings.with_crate(
            "oxide-update-engine-types",
            typify::CrateVers::Any,
            None,
        );
        settings.with_crate("typify-example", typify::CrateVers::Any, None);

        let mut type_space = typify::TypeSpace::new(&settings);
        type_space.add_root_schema(schema).expect("added root schema");

        let code = type_space.to_stream();
        let file =
            syn::parse2::<syn::File>(code).expect("parsed generated code");
        let formatted = prettyplease::unparse(&file);

        assert_contents("tests/output/external_spec_typify.rs", &formatted);
    }
}
