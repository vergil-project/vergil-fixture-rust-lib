//! Tests for the text functions.

use vergil_fixture_rust_lib::{
    initials, is_palindrome, letter_count, longest_word, reverse_words, shortest_word, title_case,
    unique_word_count, word_count,
};

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

#[test]
fn title_case_capitalizes_each_word() {
    assert_eq!(title_case("ada lovelace"), "Ada Lovelace");
}

#[test]
fn title_case_lower_cases_the_rest_of_each_word() {
    assert_eq!(title_case("GRACE hOPPER"), "Grace Hopper");
}

#[test]
fn title_case_joins_with_single_spaces() {
    assert_eq!(
        title_case("  the quick\t\tbrown\nfox "),
        "The Quick Brown Fox"
    );
}

#[test]
fn title_case_handles_non_ascii_letters() {
    assert_eq!(title_case("émilie du CHÂTELET"), "Émilie Du Châtelet");
}

#[test]
fn title_case_of_blank_text_is_empty() {
    assert_eq!(title_case(""), "");
    assert_eq!(title_case(" \n\t "), "");
}

#[test]
fn is_palindrome_accepts_a_palindrome() {
    assert!(is_palindrome("racecar"));
}

#[test]
fn is_palindrome_rejects_a_non_palindrome() {
    assert!(!is_palindrome("Ada Lovelace"));
}

#[test]
fn is_palindrome_ignores_whitespace_and_case() {
    assert!(is_palindrome("Never odd\tor  EVEN\n"));
}

#[test]
fn is_palindrome_handles_non_ascii_letters() {
    assert!(is_palindrome("Été"));
    assert!(!is_palindrome("été e"));
}

#[test]
fn is_palindrome_of_blank_text_is_true() {
    assert!(is_palindrome(""));
    assert!(is_palindrome(" \n\t "));
}

#[test]
fn shortest_word_finds_the_shortest_word() {
    assert_eq!(shortest_word("Ada Lovelace"), Some("Ada"));
}

#[test]
fn shortest_word_keeps_the_first_of_equally_short_words() {
    assert_eq!(shortest_word("quick fox the brown"), Some("fox"));
}

#[test]
fn shortest_word_counts_characters_not_bytes() {
    assert_eq!(shortest_word("été abc"), Some("été"));
    assert_eq!(shortest_word("abc été"), Some("abc"));
}

#[test]
fn shortest_word_splits_on_any_whitespace() {
    assert_eq!(shortest_word("  ccc\tbb\na "), Some("a"));
}

#[test]
fn shortest_word_of_blank_text_is_none() {
    assert_eq!(shortest_word(""), None);
    assert_eq!(shortest_word(" \n\t "), None);
}

#[test]
fn unique_word_count_counts_each_word_once() {
    assert_eq!(unique_word_count("the cat and the hat"), 4);
}

#[test]
fn unique_word_count_of_distinct_words_is_word_count() {
    assert_eq!(unique_word_count("the quick brown fox"), 4);
}

#[test]
fn unique_word_count_is_case_sensitive() {
    assert_eq!(unique_word_count("The the THE"), 3);
}

#[test]
fn unique_word_count_handles_non_ascii_words() {
    assert_eq!(unique_word_count("été Été été"), 2);
}

#[test]
fn unique_word_count_splits_on_any_whitespace() {
    assert_eq!(unique_word_count("  a\tb\na  b "), 2);
}

#[test]
fn unique_word_count_of_blank_text_is_zero() {
    assert_eq!(unique_word_count(""), 0);
    assert_eq!(unique_word_count(" \n\t "), 0);
}
