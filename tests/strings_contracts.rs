use gutrs::strings::{
    ends_with_any, is_blank, is_non_blank, starts_with_any, truncate_words,
    truncate_words_default, WordCountError,
};

#[test]
fn js_whitespace_predicates_preserve_ecmascript_rules() {
    assert!(is_blank(" \t\n"));
    assert!(is_blank("\u{FEFF}\u{2009}"));
    assert!(!is_blank("\u{0085}"));
    assert!(!is_blank("\u{200B}"));
    assert!(is_non_blank(" a "));
    assert!(!is_non_blank("\u{00A0}"));
}

#[test]
fn string_prefix_and_suffix_membership_preserves_empty_pattern() {
    assert!(starts_with_any("hello-world", &["no", "hello"]));
    assert!(ends_with_any("hello-world", &["world"]));
    assert!(!starts_with_any("abc", &[]));
    assert!(!ends_with_any("abc", &[]));
    assert!(starts_with_any("abc", &[""]));
    assert!(ends_with_any("abc", &[""]));
}

#[test]
fn word_truncation_follows_original_gutts_behavior() {
    assert_eq!(
        truncate_words_default("the quick brown fox jumps", 3.0),
        Ok("the quick brown…".to_owned())
    );
    assert_eq!(
        truncate_words_default("hello world", 5.0),
        Ok("hello world".to_owned())
    );
    assert_eq!(
        truncate_words_default("  hello world  ", 2.0),
        Ok("  hello world  ".to_owned())
    );
    assert_eq!(
        truncate_words("a\u{00A0}b  c", 2.0, "!"),
        Ok("a b!".to_owned())
    );
    assert_eq!(
        truncate_words_default("a b c", 2.9),
        Ok("a b…".to_owned())
    );
    assert_eq!(truncate_words_default("", 0.0), Ok("".to_owned()));
    assert_eq!(truncate_words_default("   ", 0.0), Ok("…".to_owned()));
    assert_eq!(truncate_words_default("a b", f64::NAN), Ok("…".to_owned()));
    assert_eq!(
        truncate_words_default("a b", -1.0),
        Err(WordCountError::Negative)
    );
}
