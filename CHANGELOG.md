# Changelog

<!-- next-header -->
## Unreleased - ReleaseDate

### Added

- `FailureReason::message_display` returns a `FailureMessageDisplay` that renders
  a failure's message followed by its causes (`message: cause1: cause2`), and
  for parent failures prefixes the parent step's description. This method
  mirrors the existing `AbortReason::message_display`.

## [0.1.0] - 2026-02-27

Initial extraction from the [omicron](https://github.com/oxidecomputer/omicron)
repository.

<!-- next-url -->
[0.1.0]: https://github.com/oxidecomputer/oxide-update-engine/releases/tag/oxide-update-engine-0.1.0
