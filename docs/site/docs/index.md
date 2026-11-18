# vergil-fixture-rust-lib

!!! warning "A test fixture, not a real library"

    This crate is a Vergil test fixture. It has no users, and it is never
    published to crates.io.

`vergil-fixture-rust-lib` is a small Rust library that exists so Vergil's Rust
support has a real crate to work on. Each fixture run makes a small change to
it and drives that change through Vergil's whole cycle for real: the PR, CI,
the merge, the release and this docs site. A step that breaks shows up here
before it reaches a real Rust repo.

It exercises:

- `vrg-validate` for Rust: `cargo fmt`, `cargo clippy`, `cargo check`, the
  tests under `cargo llvm-cov` at 100% line and region coverage, the
  doctests under `cargo test --doc`, the release build
  (`cargo build --release --locked --workspace`), and the `cargo deny` audit against Vergil's packaged
  license and advisory policy;
- the CI and release workflows a real Rust library gets, with every fleet
  automation enabled;
- three real runtime dependencies, `serde`, `serde_json` and `thiserror`, so
  the audit, the license check and Dependabot have real work to do.

Why the fixtures exist, and how a fixture run works, is set out in the
[language fixture repos spec](https://github.com/vergil-project/.github/blob/develop/epics/1091-language-fixture-repos/spec.md).

## Next

- [Getting Started](getting-started.md): build and test the crate.
- [API](api.md): the library's public functions.
