//! Portable subset of GutTS lodash-backed string truncation.
//! Upstream: GutTS src/strings/string-transform.ts (lodash truncate).
//! Uses Unicode scalar values rather than byte offsets to avoid invalid UTF-8.
//! Grapheme-cluster and lodash's UTF-16/Unicode symbol segmentation still need differential tests.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruncateError {
    NegativeLength,
}

/// Truncate to at most `max_length` Unicode scalar values.
/// Like lodash truncate, when the omission is at least as long as the limit,
/// the full omission is returned. No word-boundary separator is requested.
pub fn truncate(value: &str, max_length: i64, omission: &str) -> Result<String, TruncateError> {
    if max_length < 0 {
        return Err(TruncateError::NegativeLength);
    }
    let limit = max_length as usize;
    let value_len = value.chars().count();
    if value_len <= limit {
        return Ok(value.to_owned());
    }
    let omission_len = omission.chars().count();
    if limit <= omission_len {
        return Ok(omission.to_owned());
    }
    let prefix: String = value.chars().take(limit - omission_len).collect();
    Ok(prefix + omission)
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
    fn rejects_negative_lengths_and_keeps_utf8_boundaries() {
        assert_eq!(truncate_default("abc", -1), Err(TruncateError::NegativeLength));
        assert_eq!(truncate_default("résumé", 4), Ok("rés…".into()));
        assert_eq!(truncate_default("猫犬鳥魚", 3), Ok("猫犬…".into()));
    }
}
