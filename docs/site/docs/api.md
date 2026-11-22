# API

Everything public is re-exported from the crate root,
`vergil_fixture_rust_lib`.

## Greetings

### `greet`

```rust
pub fn greet(name: &str) -> String
```

Greets `name`, trimmed of surrounding whitespace. A blank name greets the
world.

```rust
use vergil_fixture_rust_lib::greet;

assert_eq!(greet("Ada"), "Hello, Ada");
assert_eq!(greet("  Grace "), "Hello, Grace");
assert_eq!(greet(""), "Hello, world");
```

### `parse_greeting` and `Greeting`

```rust
pub fn parse_greeting(json: &str) -> Result<Greeting, Error>

pub struct Greeting {
    pub name: String,
    pub punctuation: Option<String>,
}

impl Greeting {
    pub fn render(&self) -> String
}
```

`parse_greeting` reads a greeting from a JSON object with a required `name`
and an optional `punctuation`, and trims the name. `Greeting` also serializes
back to the same JSON with `serde_json`. `render` gives the greeting as text.

```rust
use vergil_fixture_rust_lib::parse_greeting;

let greeting = parse_greeting(r#"{"name": "Ada", "punctuation": "!"}"#)?;
assert_eq!(greeting.render(), "Hello, Ada!");
```

It fails with:

- `Error::Json` when the input is not JSON, has no `name`, or has a field
  other than `name` and `punctuation`;
- `Error::BlankName` when the name is empty or only whitespace.

## Text

### `word_count`

```rust
pub fn word_count(text: &str) -> usize
```

Counts the whitespace-separated words in `text`.

```rust
use vergil_fixture_rust_lib::word_count;

assert_eq!(word_count("the quick  brown fox"), 4);
assert_eq!(word_count(""), 0);
```

### `initials`

```rust
pub fn initials(text: &str) -> String
```

The upper-cased first letter of each whitespace-separated word in `text`.

```rust
use vergil_fixture_rust_lib::initials;

assert_eq!(initials("Ada Lovelace"), "AL");
assert_eq!(initials("émilie du châtelet"), "ÉDC");
```

### `reverse_words`

```rust
pub fn reverse_words(text: &str) -> String
```

The whitespace-separated words of `text` in reverse order, joined by single
spaces.

```rust
use vergil_fixture_rust_lib::reverse_words;

assert_eq!(reverse_words("the quick  brown fox"), "fox brown quick the");
assert_eq!(reverse_words("   "), "");
```

### `longest_word`

```rust
pub fn longest_word(text: &str) -> Option<&str>
```

The longest whitespace-separated word in `text`, counted in characters, or
`None` when `text` has no words. Of several equally long words, the first
wins.

```rust
use vergil_fixture_rust_lib::longest_word;

assert_eq!(longest_word("the quick  brown fox"), Some("quick"));
assert_eq!(longest_word("   "), None);
```

### `letter_count`

```rust
pub fn letter_count(text: &str) -> usize
```

Counts the characters of `text` that are not whitespace.

```rust
use vergil_fixture_rust_lib::letter_count;

assert_eq!(letter_count("the quick  brown fox"), 16);
assert_eq!(letter_count("   "), 0);
```

### `title_case`

```rust
pub fn title_case(text: &str) -> String
```

The whitespace-separated words of `text` in title case: the first letter of
each word upper-cased and the rest lower-cased, joined by single spaces.

```rust
use vergil_fixture_rust_lib::title_case;

assert_eq!(title_case("the QUICK  brown fox"), "The Quick Brown Fox");
assert_eq!(title_case("   "), "");
```

### `is_palindrome`

```rust
pub fn is_palindrome(text: &str) -> bool
```

Whether `text` reads the same forwards and backwards, ignoring whitespace and
letter case. Text with no letters at all is a palindrome.

```rust
use vergil_fixture_rust_lib::is_palindrome;

assert!(is_palindrome("Never odd  or even"));
assert!(!is_palindrome("the quick brown fox"));
```

## Errors

```rust
pub enum Error {
    Json(serde_json::Error),
    BlankName,
}
```

The error from the library's fallible functions. It implements
`std::error::Error`; for `Error::Json` the source is the `serde_json` error.
