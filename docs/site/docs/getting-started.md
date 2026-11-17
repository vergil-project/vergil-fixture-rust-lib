# Getting Started

The crate is not on crates.io (a fixture never reaches a package registry), so
work on it from a clone of the repository.

## Validate

Validation runs in Vergil's Rust dev image, the same way CI runs it:

```bash
vrg-container-run -- vrg-validate
```

That checks formatting, lints, type-checks, tests at 100% line and region
coverage, runs the doctests, builds the release profile and audits the
dependencies. There is no other validation command.

## Layout

| Path | What it holds |
|---|---|
| `src/lib.rs` | The crate root: its modules and the `pub use` lines that make up the public API |
| `src/greeting.rs` | The greeting functions |
| `src/text.rs` | The text functions |
| `src/error.rs` | The library's error type |
| `tests/` | One integration-test file per module |

## Adding a function

Each fixture run adds one function, without restructuring the crate:

1. Write its tests in the `tests/` file for its theme, or a new one.
2. Add the function, with a doc comment, to the module for its theme, or to a
   new module declared in `src/lib.rs`.
3. Add it to the `pub use` lines in `src/lib.rs`.
4. Document it on the [API](api.md) page.
5. Add it to the function list in `README.md`.
6. Run `vrg-container-run -- vrg-validate`.
