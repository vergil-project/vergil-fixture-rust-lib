//! A small library that exists only as a Vergil test fixture.
//!
//! It has no users. It gives Vergil's Rust support a real crate to build,
//! lint, test, audit, document and release, so a fixture run exercises every
//! step for real (epic vergil-project/.github#1091).
//!
//! The public API is a handful of plain functions, grouped by theme into
//! modules and re-exported here. Each fixture run adds one more function: a
//! new function goes into the module for its theme (or a new module), gains a
//! `pub use` line below, and gets its tests in the matching file under
//! `tests/`.
//!
//! ```
//! use vergil_fixture_rust_lib::{greet, initials, word_count};
//!
//! assert_eq!(greet("Ada"), "Hello, Ada");
//! assert_eq!(initials("Ada Lovelace"), "AL");
//! assert_eq!(word_count("the quick brown fox"), 4);
//! ```

mod error;
mod greeting;
mod text;

pub use error::Error;
pub use greeting::{Greeting, greet, parse_greeting};
pub use text::{initials, longest_word, reverse_words, word_count};
