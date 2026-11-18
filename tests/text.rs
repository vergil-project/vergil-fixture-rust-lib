//! Tests for the text functions.

use vergil_fixture_rust_lib::{initials, letter_count, longest_word, reverse_words, word_count};

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

#[test]
fn reverse_words_reverses_the_word_order() {
    assert_eq!(reverse_words("Ada Lovelace"), "Lovelace Ada");
}

#[test]
fn reverse_words_joins_with_single_spaces() {
    assert_eq!(
        reverse_words("  the quick\t\tbrown\nfox "),
        "fox brown quick the"
    );
}

#[test]
fn reverse_words_keeps_a_single_word() {
    assert_eq!(reverse_words("émilie"), "émilie");
}

#[test]
fn reverse_words_of_blank_text_is_empty() {
    assert_eq!(reverse_words(""), "");
    assert_eq!(reverse_words(" \n\t "), "");
}

#[test]
fn longest_word_finds_the_longest_word() {
    assert_eq!(longest_word("Ada Lovelace"), Some("Lovelace"));
}

#[test]
fn longest_word_keeps_the_first_of_equally_long_words() {
    assert_eq!(longest_word("the quick brown fox"), Some("quick"));
}

#[test]
fn longest_word_counts_characters_not_bytes() {
    assert_eq!(longest_word("émilie abcdef"), Some("émilie"));
    assert_eq!(longest_word("abcdef émilie"), Some("abcdef"));
}

#[test]
fn longest_word_splits_on_any_whitespace() {
    assert_eq!(longest_word("  a\tbb\nccc "), Some("ccc"));
}

#[test]
fn longest_word_of_blank_text_is_none() {
    assert_eq!(longest_word(""), None);
    assert_eq!(longest_word(" \n\t "), None);
}

#[test]
fn letter_count_skips_whitespace() {
    assert_eq!(letter_count("  the quick\tbrown\nfox "), 16);
}

#[test]
fn letter_count_counts_characters_not_bytes() {
    assert_eq!(letter_count("émilie"), 6);
}

#[test]
fn letter_count_of_blank_text_is_zero() {
    assert_eq!(letter_count(""), 0);
    assert_eq!(letter_count(" \n\t "), 0);
}
