//! Semantics pinned to GutTS 2.2.2 string-predicates and string-truncate-words.
#![allow(clippy::module_name_repetitions)]

/// JavaScript String.trim and regular-expression whitespace (ECMAScript WhiteSpace + LineTerminator).
/// Rust char::is_whitespace differs at U+0085 and U+FEFF, so it is not used.
fn is_js_whitespace(value: char) -> bool {
    matches!(
        value,
        '\u{0009}'..='\u{000D}'
            | '\u{0020}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200A}'
            | '\u{2028}'..='\u{2029}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
            | '\u{FEFF}'
    )
}

pub fn is_blank(value: &str) -> bool {
    value.trim_matches(is_js_whitespace).is_empty()
}

pub fn is_non_blank(value: &str) -> bool {
    !is_blank(value)
}

pub fn starts_with_any(value: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| value.starts_with(prefix))
}

pub fn ends_with_any(value: &str, suffixes: &[&str]) -> bool {
    suffixes.iter().any(|suffix| value.ends_with(suffix))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordCountError {
    Negative,
}

/// Preserve GutTS's original string when it already fits, including whitespace.
/// max_words uses f64 to preserve JS comparisons and Array.slice truncation behavior.
pub fn truncate_words(value: &str, max_words: f64, suffix: &str) -> Result<String, WordCountError> {
    if max_words < 0.0 {
        return Err(WordCountError::Negative);
    }
    if value.is_empty() {
        return Ok(String::new());
    }
    let trimmed = value.trim_matches(is_js_whitespace);
    // JavaScript "".split(/\s+/) returns a single empty string.
    let words: Vec<&str> = if trimmed.is_empty() {
        vec![""]
    } else {
        trimmed
            .split(is_js_whitespace)
            .filter(|word| !word.is_empty())
            .collect()
    };
    if words.len() as f64 <= max_words {
        return Ok(value.to_owned());
    }
    let count = if max_words.is_nan() {
        0
    } else {
        max_words.trunc().max(0.0).min(words.len() as f64) as usize
    };
    Ok(format!("{}{}", words[..count].join(" "), suffix))
}

pub fn truncate_words_default(value: &str, max_words: f64) -> Result<String, WordCountError> {
    truncate_words(value, max_words, "…")
}
