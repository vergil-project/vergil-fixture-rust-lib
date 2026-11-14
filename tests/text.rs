//! Tests for the text functions.

use vergil_fixture_rust_lib::{initials, word_count};

#[test]
fn word_count_counts_whitespace_separated_words() {
    assert_eq!(word_count("the quick  brown\tfox\n"), 4);
}

#[test]
fn word_count_of_blank_text_is_zero() {
    assert_eq!(word_count(""), 0);
    assert_eq!(word_count(" \n\t "), 0);
}

#[test]
fn initials_takes_the_first_letter_of_each_word() {
    assert_eq!(initials("Ada Lovelace"), "AL");
}

#[test]
fn initials_upper_cases_the_letters() {
    assert_eq!(initials("grace brewster murray hopper"), "GBMH");
}

#[test]
fn initials_handles_non_ascii_letters() {
    assert_eq!(initials("émilie du châtelet"), "ÉDC");
}

#[test]
fn initials_of_blank_text_is_empty() {
    assert_eq!(initials("   "), "");
}
