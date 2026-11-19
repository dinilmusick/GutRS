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


/// Capitalize the first Unicode scalar and lowercase the remainder.
/// Mirrors the common portable subset of GutTS lodash capitalize; lodash's
/// Unicode word/symbol segmentation remains a differential-testing gap.
pub fn capitalize(value: &str) -> String {
    // Lowercase the complete string first so context-sensitive Unicode mappings 
    // (notably Greek final sigma) match JavaScript's toLowerCase behavior.
    let lower = value.to_lowercase();
    let mut chars = lower.chars();
    let Some(first) = chars.next() else { return String::new(); };
    let mut output = first.to_uppercase().collect::<String>();
    output.extend(chars);
    output
}

#[cfg(test)]
mod capitalize_tests {
    use super::capitalize;

    #[test]
    fn handles_ascii_and_unicode_without_byte_slicing() {
        assert_eq!(capitalize("hELLO WORLD"), "Hello world");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("éCOLE"), "École");
        assert_eq!(capitalize("猫ABC"), "猫abc");
        assert_eq!(capitalize("a"), "A");
        assert_eq!(capitalize("ΟΣ"), "Ος");
        assert_eq!(capitalize("ΟΣ ΑΒ"), "Ος αβ");
    }
}

#[cfg(test)]
mod truncate_words_contract_tests {
    use super::{is_blank, is_non_blank, starts_with_any, ends_with_any, truncate_words, truncate_words_default, WordCountError};

    #[test]
    fn preserves_original_when_word_limit_does_not_truncate() {
        assert_eq!(truncate_words("  alpha   beta  ", 2.0, "!").unwrap(), "  alpha   beta  ");
        assert_eq!(truncate_words("", 0.0, "!").unwrap(), "");
        assert_eq!(truncate_words("alpha beta", f64::INFINITY, "!").unwrap(), "alpha beta");
    }

    #[test]
    fn truncates_at_ecmascript_whitespace_boundaries() {
        assert_eq!(truncate_words(" alpha\u{00a0}beta\u{feff}gamma ", 2.0, "!").unwrap(), "alpha beta!");
        assert_eq!(truncate_words_default("one two three", 1.0).unwrap(), "one…");
        assert_eq!(truncate_words("one two three", 1.9, "--").unwrap(), "one--");
        assert_eq!(truncate_words("one two", 0.0, "!").unwrap(), "!");
        assert_eq!(truncate_words("one two", f64::NAN, "!").unwrap(), "!");
    }

    #[test]
    fn validates_negative_limits_and_js_whitespace() {
        assert_eq!(truncate_words("a b", -0.5, "!"), Err(WordCountError::Negative));
        assert!(is_blank("\u{feff}\u{2028}\u{00a0}"));
        assert!(!is_blank("\u{0085}"));
        assert!(is_non_blank("\u{0085}"));
        assert!(starts_with_any("hello", &["no", "he"]));
        assert!(ends_with_any("hello", &["no", "lo"]));
        assert!(!starts_with_any("hello", &[]));
    }
}
