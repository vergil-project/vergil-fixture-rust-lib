# vergil-fixture-rust-lib

This is a Vergil test fixture: a small Rust library with no users. It exists so
Vergil's Rust support has a real library crate to validate, release and
document, and it serves as a reference example of a well-formed Vergil Rust
library. Why it exists and how it is used are set out in the
[language fixture repos spec](https://github.com/vergil-project/.github/blob/develop/epics/1091-language-fixture-repos/spec.md)
(`epics/1091-language-fixture-repos/spec.md` in `vergil-project/.github`).

## Table of Contents

- [Status](#status)
- [Overview](#overview)
- [Getting Started](#getting-started)
- [License](#license)

## Status

A test fixture. It is never published to crates.io.

## Overview

Each fixture run makes a small change to the crate, one more function, and
drives it through Vergil's whole cycle for real: the PR, CI, the merge, the
release and the docs site. The fixture exercises:

- `vrg-validate` for Rust: `cargo fmt`, `cargo clippy`, `cargo check`, the
  tests under `cargo llvm-cov` at 100% line and region coverage, the
  doctests under `cargo test --doc`, the release build
  (`cargo build --release --locked --workspace`), and the `cargo deny` audit against Vergil's packaged
  policy;
- the CI and release workflows of a Vergil Rust library, with every fleet
  automation enabled;
- three real runtime dependencies, `serde`, `serde_json` and `thiserror`.

Its public API is made up of the functions `greet`, `parse_greeting`,
`word_count`, `initials`, `reverse_words`, `longest_word`, `letter_count`,
`title_case`, `is_palindrome`, `shortest_word`, `unique_word_count`,
`word_lengths` and `last_word`, plus the `Greeting` and `Error` types.

## Getting Started

```bash
vrg-container-run -- vrg-validate
```

See the [documentation](https://vergil-project.github.io/vergil-fixture-rust-lib/)
for the API and the crate's layout.

## License

MIT — see [LICENSE](LICENSE).
