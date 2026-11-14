//! Greeting functions.

use serde::{Deserialize, Serialize};

use crate::Error;

/// The name greeted when none is given.
const NOBODY: &str = "world";

/// Greets `name`, trimmed of surrounding whitespace.
///
/// A blank name greets the world.
///
/// ```
/// use vergil_fixture_rust_lib::greet;
///
/// assert_eq!(greet("Ada"), "Hello, Ada");
/// assert_eq!(greet(""), "Hello, world");
/// ```
#[must_use]
pub fn greet(name: &str) -> String {
    let name = name.trim();
    let name = if name.is_empty() { NOBODY } else { name };
    format!("Hello, {name}")
}

/// A greeting read from, or written as, a JSON document.
///
/// The document is an object with a required `name` and an optional
/// `punctuation`, for example `{"name": "Ada", "punctuation": "!"}`. Any other
/// field is an error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Greeting {
    /// Who is greeted.
    pub name: String,
    /// What ends the greeting, if anything.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punctuation: Option<String>,
}

impl Greeting {
    /// The greeting as text: [`greet`] of the name, then the punctuation.
    ///
    /// ```
    /// use vergil_fixture_rust_lib::Greeting;
    ///
    /// let greeting = Greeting { name: "Ada".to_owned(), punctuation: Some("!".to_owned()) };
    /// assert_eq!(greeting.render(), "Hello, Ada!");
    /// ```
    #[must_use]
    pub fn render(&self) -> String {
        let punctuation = self.punctuation.as_deref().unwrap_or_default();
        format!("{}{punctuation}", greet(&self.name))
    }
}

/// Parses a [`Greeting`] from a JSON document, trimming its name.
///
/// ```
/// use vergil_fixture_rust_lib::parse_greeting;
///
/// let greeting = parse_greeting(r#"{"name": " Ada "}"#)?;
/// assert_eq!(greeting.name, "Ada");
/// # Ok::<(), vergil_fixture_rust_lib::Error>(())
/// ```
///
/// # Errors
///
/// [`Error::Json`] when `json` is not a valid greeting document, and
/// [`Error::BlankName`] when its name is empty or only whitespace.
pub fn parse_greeting(json: &str) -> Result<Greeting, Error> {
    let mut greeting: Greeting = serde_json::from_str(json)?;
    greeting.name = greeting.name.trim().to_owned();
    if greeting.name.is_empty() {
        return Err(Error::BlankName);
    }
    Ok(greeting)
}
