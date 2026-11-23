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

/// The whitespace-separated words of `text` in reverse order, joined by
/// single spaces.
///
/// ```
/// use vergil_fixture_rust_lib::reverse_words;
///
/// assert_eq!(reverse_words("the quick  brown fox"), "fox brown quick the");
/// assert_eq!(reverse_words("   "), "");
/// ```
#[must_use]
pub fn reverse_words(text: &str) -> String {
    text.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

/// The longest whitespace-separated word in `text`, counted in characters,
/// or `None` when `text` has no words. Of several equally long words, the
/// first wins.
///
/// ```
/// use vergil_fixture_rust_lib::longest_word;
///
/// assert_eq!(longest_word("the quick  brown fox"), Some("quick"));
/// assert_eq!(longest_word("   "), None);
/// ```
#[must_use]
pub fn longest_word(text: &str) -> Option<&str> {
    // `max_by_key` keeps the last of equal maxima, so walking the words in
    // reverse makes the first one win.
    text.split_whitespace()
        .rev()
        .max_by_key(|word| word.chars().count())
}

/// Counts the characters of `text` that are not whitespace.
///
/// ```
/// use vergil_fixture_rust_lib::letter_count;
///
/// assert_eq!(letter_count("the quick  brown fox"), 16);
/// assert_eq!(letter_count("   "), 0);
/// ```
#[must_use]
pub fn letter_count(text: &str) -> usize {
    text.chars().filter(|c| !c.is_whitespace()).count()
}

/// The whitespace-separated words of `text` in title case: the first letter
/// of each word upper-cased and the rest lower-cased, joined by single
/// spaces.
///
/// ```
/// use vergil_fixture_rust_lib::title_case;
///
/// assert_eq!(title_case("the QUICK  brown fox"), "The Quick Brown Fox");
/// assert_eq!(title_case("   "), "");
/// ```
#[must_use]
pub fn title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut titled = String::with_capacity(word.len());
            for (index, letter) in word.chars().enumerate() {
                if index == 0 {
                    titled.extend(letter.to_uppercase());
                } else {
                    titled.extend(letter.to_lowercase());
                }
            }
            titled
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Whether `text` reads the same forwards and backwards, ignoring whitespace
/// and letter case. Text with no letters at all is a palindrome.
///
/// ```
/// use vergil_fixture_rust_lib::is_palindrome;
///
/// assert!(is_palindrome("Never odd  or even"));
/// assert!(!is_palindrome("the quick brown fox"));
/// ```
#[must_use]
pub fn is_palindrome(text: &str) -> bool {
    let letters: Vec<char> = text
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect();
    letters.iter().eq(letters.iter().rev())
}

/// The shortest whitespace-separated word in `text`, counted in characters,
/// or `None` when `text` has no words. Of several equally short words, the
/// first wins.
///
/// ```
/// use vergil_fixture_rust_lib::shortest_word;
///
/// assert_eq!(shortest_word("the quick  brown fox"), Some("the"));
/// assert_eq!(shortest_word("   "), None);
/// ```
#[must_use]
pub fn shortest_word(text: &str) -> Option<&str> {
    // `min_by_key` keeps the first of equal minima.
    text.split_whitespace()
        .min_by_key(|word| word.chars().count())
}

/// Counts the distinct whitespace-separated words in `text`. Words are
/// compared exactly, so words that differ only in letter case are distinct.
///
/// ```
/// use vergil_fixture_rust_lib::unique_word_count;
///
/// assert_eq!(unique_word_count("the cat and the  hat"), 4);
/// assert_eq!(unique_word_count("   "), 0);
/// ```
#[must_use]
pub fn unique_word_count(text: &str) -> usize {
    text.split_whitespace()
        .collect::<std::collections::HashSet<_>>()
        .len()
}

/// The length of each whitespace-separated word in `text`, counted in
/// characters, in the order the words appear.
///
/// ```
/// use vergil_fixture_rust_lib::word_lengths;
///
/// assert_eq!(word_lengths("the quick  brown fox"), vec![3, 5, 5, 3]);
/// assert_eq!(word_lengths("   "), Vec::<usize>::new());
/// ```
#[must_use]
pub fn word_lengths(text: &str) -> Vec<usize> {
    text.split_whitespace()
        .map(|word| word.chars().count())
        .collect()
}
