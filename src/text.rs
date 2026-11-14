//! Text functions.

/// Counts the whitespace-separated words in `text`.
///
/// ```
/// use vergil_fixture_rust_lib::word_count;
///
/// assert_eq!(word_count("the quick  brown fox"), 4);
/// assert_eq!(word_count("   "), 0);
/// ```
#[must_use]
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// The upper-cased first letter of each whitespace-separated word in `text`.
///
/// ```
/// use vergil_fixture_rust_lib::initials;
///
/// assert_eq!(initials("Ada Lovelace"), "AL");
/// assert_eq!(initials("émilie du châtelet"), "ÉDC");
/// ```
#[must_use]
pub fn initials(text: &str) -> String {
    text.split_whitespace()
        .filter_map(|word| word.chars().next())
        .flat_map(char::to_uppercase)
        .collect()
}
