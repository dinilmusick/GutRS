//! Trusted-origin API URL resolution. No provider-specific credentials or SDKs.
#![forbid(unsafe_code)]
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustedUrlError {
    InvalidOrigin,
    InvalidPath,
    UnsafeSegment,
}

fn validate_segment(segment: &str) -> Result<(), TrustedUrlError> {
    let mut current = segment.to_string();
    for _ in 0..8 {
        let bytes = current.as_bytes();
        let mut decoded = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' {
                if i + 2 >= bytes.len() {
                    return Err(TrustedUrlError::UnsafeSegment);
                }
                let digit = |b: u8| (b as char).to_digit(16).map(|n| n as u8);
                let hi = digit(bytes[i + 1]).ok_or(TrustedUrlError::UnsafeSegment)?;
                let lo = digit(bytes[i + 2]).ok_or(TrustedUrlError::UnsafeSegment)?;
                decoded.push(hi * 16 + lo);
                i += 3;
            } else {
                decoded.push(bytes[i]);
                i += 1;
            }
        }
        current = String::from_utf8(decoded).map_err(|_| TrustedUrlError::UnsafeSegment)?;
        if current.split('/').any(|p| p == "." || p == "..")
            || current
                .bytes()
                .any(|b| b == b'\\' || b == b'?' || b == b'#' || b < 32 || b == 127)
        {
            return Err(TrustedUrlError::UnsafeSegment);
        }
        if !current
            .as_bytes()
            .windows(3)
            .any(|w| w[0] == b'%' && w[1].is_ascii_hexdigit() && w[2].is_ascii_hexdigit())
        {
            return Ok(());
        }
    }
    Err(TrustedUrlError::UnsafeSegment)
}

/// Resolve a root-relative API path without permitting another origin or credentials.
/// Encoded slashes are supported for provider identifiers, but encoded traversal is not.
pub fn resolve_trusted_api_url(origin: &str, path: &str) -> Result<Url, TrustedUrlError> {
    let base = Url::parse(origin).map_err(|_| TrustedUrlError::InvalidOrigin)?;
    if base.scheme() != "https"
        || base.host_str().is_none()
        || !base.username().is_empty()
        || base.password().is_some()
        || base.path() != "/"
        || base.query().is_some_and(|query| !query.is_empty())
        || base.fragment().is_some_and(|fragment| !fragment.is_empty())
    {
        return Err(TrustedUrlError::InvalidOrigin);
    }
    if !path.starts_with('/')
        || path.starts_with("//")
        || path.split('?').next().unwrap_or("").contains('#')
        || path.contains('\n')
        || path.contains('\r')
        || path
            .split('?')
            .next()
            .unwrap_or("")
            .bytes()
            .any(|b| b == b'\\' || b < 32 || b == 127)
    {
        return Err(TrustedUrlError::InvalidPath);
    }
    for part in path
        .split('?')
        .next()
        .unwrap_or("")
        .split('/')
        .filter(|p| !p.is_empty())
    {
        validate_segment(part)?;
    }
    let target = base.join(path).map_err(|_| TrustedUrlError::InvalidPath)?;
    if target.origin() != base.origin()
        || !target.username().is_empty()
        || target.password().is_some()
        || target
            .fragment()
            .is_some_and(|fragment| !fragment.is_empty())
    {
        return Err(TrustedUrlError::InvalidPath);
    }
    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn permits_encoded_project_identifier() {
        let result =
            resolve_trusted_api_url("https://gitlab.example", "/api/projects/group%2Frepo?x=1")
                .unwrap();
        assert_eq!(result.host_str(), Some("gitlab.example"));
        assert!(result.as_str().contains("group%2Frepo"));
    }
    #[test]
    fn rejects_untrusted_origins_and_paths() {
        for origin in [
            "http://example.com",
            "https://user@example.com",
            "https://example.com/base",
            "https://example.com?x=1",
        ] {
            assert!(resolve_trusted_api_url(origin, "/api").is_err(), "{origin}");
        }
        for path in [
            "//evil.example/path",
            "/%2e%2e/admin",
            "/%252e%252e/admin",
            "/a%3fb",
            "/a%23b",
            "/a%5cb",
            "/a/%",
            "/a#fragment",
        ] {
            assert!(
                resolve_trusted_api_url("https://example.com", path).is_err(),
                "{path}"
            );
        }
    }
    #[test]
    fn accepts_empty_origin_delimiters_like_web_url() {
        // GutTS checks URL.search and URL.hash, both empty for bare '?' or '#'.
        for origin in ["https://api.example.com?", "https://api.example.com#"] {
            let resolved = resolve_trusted_api_url(origin, "/v1").unwrap();
            assert_eq!(resolved.as_str(), "https://api.example.com/v1");
        }
        // Nonempty query and fragment remain forbidden.
        for origin in ["https://api.example.com?x=1", "https://api.example.com#x"] {
            assert!(resolve_trusted_api_url(origin, "/v1").is_err());
        }
    }

    #[test]
    fn matches_gutts_pinned_path_safety_golden_cases() {
        // GutTS tests/path-safety.test.ts at 3e2cbc0990ba29830ce91c5d1a1d552675d5334c.
        let valid = resolve_trusted_api_url(
            "https://gitlab.com",
            "/api/v4/projects/team%2Frepo?per_page=10",
        )
        .unwrap();
        assert_eq!(
            valid.as_str(),
            "https://gitlab.com/api/v4/projects/team%2Frepo?per_page=10"
        );
        for path in [
            "//attacker.test/api",
            "/\\attacker.test",
            "/%2e%2e/escape",
            "/%252e%252e/escape",
            "/x/../escape",
            "/x/%5c/escape",
            "/x#fragment",
            "\n/evil",
        ] {
            assert!(
                resolve_trusted_api_url("https://api.example.com", path).is_err(),
                "accepted upstream-forbidden path {path:?}"
            );
        }
        for origin in [
            "http://api.example.com",
            "https://user:token@api.example.com",
            "https://api.example.com/v1",
            "https://api.example.com/?secret=x",
        ] {
            assert!(
                resolve_trusted_api_url(origin, "/v1").is_err(),
                "accepted upstream-forbidden origin {origin:?}"
            );
        }
    }
    #[test]
    fn empty_query_fragment_matches_pinned_javascript_url_semantics() {
        let resolved = resolve_trusted_api_url("https://api.example.com", "/v1?x=1#").unwrap();
        assert_eq!(resolved.to_string(), "https://api.example.com/v1?x=1#");
        assert!(resolve_trusted_api_url("https://api.example.com", "/v1#").is_err());
        assert!(resolve_trusted_api_url("https://api.example.com", "/v1?x=1#fragment").is_err());
        for path in ["/v1?x=1\n", "/v1?x=1\r"] {
            assert!(resolve_trusted_api_url("https://api.example.com", path).is_err());
        }
    }
}
