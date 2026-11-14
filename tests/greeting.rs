//! Tests for the greeting functions.

use vergil_fixture_rust_lib::{Error, Greeting, greet, parse_greeting};

#[test]
fn greet_says_hello_to_the_name() {
    assert_eq!(greet("Ada"), "Hello, Ada");
}

#[test]
fn greet_trims_the_name() {
    assert_eq!(greet("  Grace \n"), "Hello, Grace");
}

#[test]
fn greet_without_a_name_greets_the_world() {
    assert_eq!(greet(""), "Hello, world");
    assert_eq!(greet("   "), "Hello, world");
}

#[test]
fn parse_greeting_reads_the_name() {
    let greeting = parse_greeting(r#"{"name": "Ada"}"#).expect("valid greeting");
    assert_eq!(
        greeting,
        Greeting {
            name: "Ada".to_owned(),
            punctuation: None,
        }
    );
    assert_eq!(greeting.render(), "Hello, Ada");
}

#[test]
fn parse_greeting_trims_the_name_and_reads_the_punctuation() {
    let greeting =
        parse_greeting(r#"{"name": " Grace ", "punctuation": "!"}"#).expect("valid greeting");
    assert_eq!(greeting.name, "Grace");
    assert_eq!(greeting.render(), "Hello, Grace!");
}

#[test]
fn parse_greeting_rejects_invalid_json() {
    let err = parse_greeting("not json").expect_err("invalid JSON");
    assert!(matches!(err, Error::Json(_)));
    assert!(err.to_string().starts_with("invalid greeting JSON: "));
    assert!(std::error::Error::source(&err).is_some());
}

#[test]
fn parse_greeting_rejects_a_missing_name() {
    let err = parse_greeting("{}").expect_err("missing name");
    assert!(matches!(err, Error::Json(_)));
}

#[test]
fn parse_greeting_rejects_unknown_fields() {
    let err = parse_greeting(r#"{"name": "Ada", "age": 36}"#).expect_err("unknown field");
    assert!(matches!(err, Error::Json(_)));
}

#[test]
fn parse_greeting_rejects_a_blank_name() {
    let err = parse_greeting(r#"{"name": "  "}"#).expect_err("blank name");
    assert!(matches!(err, Error::BlankName));
    assert_eq!(err.to_string(), "a greeting needs a non-blank name");
    assert!(std::error::Error::source(&err).is_none());
}

#[test]
fn a_greeting_round_trips_through_json() {
    let greeting = Greeting {
        name: "Ada".to_owned(),
        punctuation: Some("!".to_owned()),
    };
    let json = serde_json::to_string(&greeting).expect("serializable");
    assert_eq!(json, r#"{"name":"Ada","punctuation":"!"}"#);
    assert_eq!(parse_greeting(&json).expect("valid greeting"), greeting);
}
