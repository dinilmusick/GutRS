//! Portable subset of GutTS lodash-backed string truncation.
//! Upstream: GutTS src/strings/string-transform.ts (lodash truncate).
//! Uses extended grapheme clusters rather than byte offsets, preserving combining
//! sequences and emoji ZWJ sequences. Lodash-specific symbol segmentation still
//! requires cross-language differential certification.

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruncateError {
    NegativeLength,
}

/// Truncate to at most `max_length` extended grapheme clusters.
/// Like lodash truncate, when the omission is at least as long as the limit,
/// the full omission is returned. No word-boundary separator is requested.
pub fn truncate(value: &str, max_length: i64, omission: &str) -> Result<String, TruncateError> {
    if max_length < 0 {
        return Err(TruncateError::NegativeLength);
    }
    Ok(truncate_with_limit(value, max_length as usize, omission))
}

/// Accepts the JavaScript numeric length contract at the Rust boundary.
/// Lodash converts NaN to zero, truncates finite fractional lengths toward zero,
/// and treats positive infinity as an effectively unbounded length.
/// Negative values are rejected by GutTS before Lodash is called.
pub fn truncate_numeric(
    value: &str,
    max_length: f64,
    omission: &str,
) -> Result<String, TruncateError> {
    if max_length < 0.0 {
        return Err(TruncateError::NegativeLength);
    }
    let limit = if max_length.is_nan() {
        0usize
    } else if max_length.is_infinite() || max_length >= usize::MAX as f64 {
        usize::MAX
    } else {
        max_length.trunc() as usize
    };
    Ok(truncate_with_limit(value, limit, omission))
}

/// Shared Unicode-safe truncation contract for integer and JavaScript-number inputs.
/// Keep the original string when it fits, even if the omission exceeds the limit.
fn truncate_with_limit(value: &str, limit: usize, omission: &str) -> String {
    let value_len = UnicodeSegmentation::graphemes(value, true).count();
    if value_len <= limit {
        return value.to_owned();
    }
    let omission_len = UnicodeSegmentation::graphemes(omission, true).count();
    if limit <= omission_len {
        return omission.to_owned();
    }
    let prefix: String = UnicodeSegmentation::graphemes(value, true)
        .take(limit - omission_len)
        .collect();
    prefix + omission
}

pub fn truncate_default(value: &str, max_length: i64) -> Result<String, TruncateError> {
    truncate(value, max_length, "…")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_with_default_and_custom_omissions() {
        assert_eq!(truncate_default("abcdef", 4), Ok("abc…".into()));
        assert_eq!(truncate("abcdef", 5, "..."), Ok("ab...".into()));
        assert_eq!(truncate("abcdef", 2, "..."), Ok("...".into()));
        assert_eq!(truncate("abcdef", 0, ""), Ok(String::new()));
        assert_eq!(truncate("abc", 3, "..."), Ok("abc".into()));
    }

    #[test]
    fn numeric_lengths_follow_js_coercion_boundaries() {
        assert_eq!(truncate_numeric("abcdef", 4.9, "..."), Ok("a...".into()));
        assert_eq!(
            truncate_numeric("abcdef", f64::NAN, "..."),
            Ok("...".into())
        );
        assert_eq!(
            truncate_numeric("abcdef", f64::INFINITY, "..."),
            Ok("abcdef".into())
        );
        assert_eq!(truncate_numeric("abcdef", -0.0, "..."), Ok("...".into()));
        assert_eq!(
            truncate_numeric("abcdef", -0.1, "..."),
            Err(TruncateError::NegativeLength)
        );
        assert_eq!(
            truncate_numeric("abcdef", f64::NEG_INFINITY, "..."),
            Err(TruncateError::NegativeLength)
        );
        assert_eq!(truncate_numeric("abcdef", 0.0, ""), Ok(String::new()));
    }

    #[test]
    fn preserves_combining_marks_and_joined_emoji() {
        assert_eq!(truncate("e\u{301}xyz", 2, "…"), Ok("e\u{301}…".into()));
        assert_eq!(truncate("👩‍💻xyz", 2, "…"), Ok("👩‍💻…".into()));
        assert_eq!(truncate_numeric("🇮🇳xyz", 2.0, "…"), Ok("🇮🇳…".into()));
        assert_eq!(truncate("abcdef", 1, "👩‍💻"), Ok("👩‍💻".into()));
    }

    #[test]
    fn rejects_negative_lengths_and_keeps_utf8_boundaries() {
        assert_eq!(
            truncate_default("abc", -1),
            Err(TruncateError::NegativeLength)
        );
        assert_eq!(truncate_default("résumé", 4), Ok("rés…".into()));
        assert_eq!(truncate_default("猫犬鳥魚", 3), Ok("猫犬…".into()));
    }
}
