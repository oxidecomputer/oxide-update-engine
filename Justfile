default:
    @just --list

# Run feature powerset (pass extra args like `--partition 1/10 nextest run`).
powerset *args:
    NEXTEST_NO_TESTS=pass cargo hack --feature-powerset --workspace {{ args }}

# Format code.
xfmt:
    cargo xfmt

# Build docs.
rustdoc:
    RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS='-D warnings --cfg=doc_cfg' cargo doc --no-deps --all-features --workspace

# Generate README.md files using `cargo-sync-rdme`.
generate-readmes:
    cargo sync-rdme --toolchain nightly-2025-11-05 --workspace --all-features

# Run cargo release in CI.
ci-cargo-release:
    # cargo-release requires a release off a branch.
    git checkout -B to-release
    cargo release publish --publish --execute --no-confirm --workspace
    git checkout -
    git branch -D to-release
