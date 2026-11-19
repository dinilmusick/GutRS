//! Portable storage path validation matching GutTS path-safety semantics.
//! Provider-specific URL resolution is intentionally not implemented here.
#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathError {
    InvalidCharacter,
    InvalidPercentEncoding,
    UnsafeSegment,
    TooDeeplyEncoded,
    EmptyPath,
}

fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn decode_once(value: &[u8]) -> Result<Vec<u8>, PathError> {
    let mut out = Vec::with_capacity(value.len());
    let mut i = 0;
    while i < value.len() {
        if value[i] == b'%' {
            if i + 2 >= value.len() {
                return Err(PathError::InvalidPercentEncoding);
            }
            let high = hex(value[i + 1]).ok_or(PathError::InvalidPercentEncoding)?;
            let low = hex(value[i + 2]).ok_or(PathError::InvalidPercentEncoding)?;
            out.push(high * 16 + low);
            i += 3;
        } else {
            out.push(value[i]);
            i += 1;
        }
    }
    std::str::from_utf8(&out).map_err(|_| PathError::InvalidPercentEncoding)?;
    Ok(out)
}

fn invalid_char(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .any(|&b| b == b'\\' || b == b'?' || b == b'#' || b < 32 || b == 127)
}

fn has_encoded_escape(value: &[u8]) -> bool {
    value.windows(3).any(|window| {
        window[0] == b'%' && hex(window[1]).is_some() && hex(window[2]).is_some()
    })
}

fn validate_segment(segment: &str) -> Result<(), PathError> {
    let mut value = segment.as_bytes().to_vec();
    for _ in 0..8 {
        value = decode_once(&value)?;
        if invalid_char(&value) || value.contains(&b'/') || value == b"." || value == b".." {
            return Err(PathError::UnsafeSegment);
        }
        // decodeURIComponent accepts a decoded literal percent only when it is
        // not part of another escape sequence. Re-decoding every '%' rejects
        // valid filenames such as 100%25-safe.txt.
        if !has_encoded_escape(&value) {
            return Ok(());
        }
    }
    Err(PathError::TooDeeplyEncoded)
}

/// Normalize a storage-relative path, preserving harmless Unicode and filenames.
pub fn normalize_storage_path(path: &str) -> Result<String, PathError> {
    if invalid_char(path.as_bytes()) {
        return Err(PathError::InvalidCharacter);
    }
    let parts: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
    for part in &parts {
        validate_segment(part)?;
    }
    Ok(parts.join("/"))
}

/// Percent-encode each validated path component, preserving directory separators.
pub fn encode_safe_path_segments(path: &str) -> Result<String, PathError> {
    let normalized = normalize_storage_path(path)?;
    if normalized.is_empty() {
        return Err(PathError::EmptyPath);
    }
    let mut output = String::new();
    for (index, segment) in normalized.split('/').enumerate() {
        if index > 0 {
            output.push('/');
        }
        for byte in segment.as_bytes() {
            if byte.is_ascii_alphanumeric()
                || matches!(
                    *byte,
                    b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')'
                )
            {
                output.push(*byte as char);
            } else {
                use std::fmt::Write;
                write!(&mut output, "%{byte:02X}").expect("String writes cannot fail");
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_slashes_and_unicode() {
        assert_eq!(
            normalize_storage_path("//folder///résumé.txt/").unwrap(),
            "folder/résumé.txt"
        );
        assert_eq!(normalize_storage_path("///").unwrap(), "");
    }

    #[test]
    fn rejects_traversal_at_every_encoding_depth() {
        for path in [
            "../secret",
            "%2e%2e/secret",
            "%252e%252e/secret",
            "dir/%2Fsecret",
            "dir/%255csecret",
            "dir/%3fquery",
            "dir/%00nul",
            "dir/%",
            "dir/%ZZ",
            "dir/%C3%28",
            "dir\\file",
        ] {
            assert!(normalize_storage_path(path).is_err(), "accepted {path}");
        }
    }

    #[test]
    fn rejects_nested_escapes_and_preserves_safe_literal_percent_encodings() {
        // Repeated decoding must not hide a traversal or reserved delimiter.
        let mut traversal = "..".to_string();
        for _ in 0..8 {
            traversal = traversal.replace('%', "%25").replace('.', "%2E");
            assert!(normalize_storage_path(&format!("root/{traversal}/file")).is_err());
        }
        for input in ["a/%252F/b", "a/%25255C/b", "a/%25253F/b", "a/%252500/b"] {
            assert!(normalize_storage_path(input).is_err(), "accepted {input}");
        }
        assert_eq!(normalize_storage_path("docs/100%25-safe.txt").unwrap(), "docs/100%25-safe.txt");
        assert_eq!(encode_safe_path_segments("docs/100%25-safe.txt").unwrap(), "docs/100%2525-safe.txt");
    }

    #[test]
    fn validates_all_percent_encoded_bytes_and_nested_escapes() {
        // Exhaustively exercise each decoded byte, including UTF-8 continuation
        // bytes and reserved ASCII. Invalid standalone UTF-8 must fail closed.
        for byte in 0u8..=255 {
            let path = format!("safe/%{byte:02X}");
            let decoded = [byte];
            let allowed = std::str::from_utf8(&decoded).is_ok()
                && !matches!(byte, b'/' | b'\\' | b'?' | b'#' | b'.')
                && byte >= 32
                && byte != 127;
            // A single dot is a complete traversal segment, but a literal
            // percent is accepted after decoding %25.
            assert_eq!(
                normalize_storage_path(&path).is_ok(),
                allowed,
                "unexpected validation for byte {byte:02X}"
            );
        }

        for path in [
            "safe/%252e%252e",
            "safe/%252f",
            "safe/%255c",
            "safe/%253f",
            "safe/%2523",
            "safe/%2500",
            "safe/%257f",
        ] {
            assert!(normalize_storage_path(path).is_err(), "accepted {path}");
        }
        assert!(normalize_storage_path("safe/%C3%A9").is_ok());
        assert!(normalize_storage_path("safe/%E2%82%AC").is_ok());
        assert!(normalize_storage_path("safe/%F0%9F%8E%B5").is_ok());
    }

    #[test]
    fn encodes_segments_without_encoding_separators() {
        assert_eq!(
            encode_safe_path_segments("a b/résumé.txt").unwrap(),
            "a%20b/r%C3%A9sum%C3%A9.txt"
        );
        assert_eq!(encode_safe_path_segments("///"), Err(PathError::EmptyPath));
        assert_eq!(encode_safe_path_segments("a!~*'()/b").unwrap(), "a!~*'()/b");
    }
}
