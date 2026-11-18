//! WHATWG-style URL operations backed by the Rust url crate.
//! Corresponds to GutTS src/urls/url-operations.ts.
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidUrl;

pub fn is_url(value: &str) -> bool {
    Url::parse(value).is_ok()
}

/// Returns the first decoded value for a query key, or None when absent.
pub fn get_query_param(input: &str, key: &str) -> Result<Option<String>, InvalidUrl> {
    let parsed = Url::parse(input).map_err(|_| InvalidUrl)?;
    Ok(parsed
        .query_pairs()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value.into_owned()))
}

/// Replaces all occurrences of key with one value at its first position.
/// Other query pairs retain their original relative ordering.
pub fn set_query_param(input: &str, key: &str, value: &str) -> Result<String, InvalidUrl> {
    let mut parsed = Url::parse(input).map_err(|_| InvalidUrl)?;
    let mut pairs: Vec<(String, String)> = parsed
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    if let Some(first) = pairs.iter().position(|(k, _)| k == key) {
        pairs[first].1 = value.to_owned();
        let mut index = pairs.len();
        while index > first + 1 {
            index -= 1;
            if pairs[index].0 == key {
                pairs.remove(index);
            }
        }
    } else {
        pairs.push((key.to_owned(), value.to_owned()));
    }
    parsed.query_pairs_mut().clear().extend_pairs(pairs.iter().map(|(k, v)| (&**k, &**v)));
    Ok(parsed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_absolute_urls() {
        assert!(is_url("https://example.com/path"));
        assert!(!is_url("/relative"));
        assert!(!is_url("not a url"));
    }

    #[test]
    fn reads_first_decoded_parameter() {
        assert_eq!(get_query_param("https://example.com/?a=one+two&a=three", "a").unwrap(), Some("one two".into()));
        assert_eq!(get_query_param("https://example.com/?a=", "a").unwrap(), Some(String::new()));
        assert_eq!(get_query_param("https://example.com/", "a").unwrap(), None);
        assert!(get_query_param("/relative", "a").is_err());
    }

    #[test]
    fn replaces_duplicates_and_encodes_reserved_characters() {
        assert_eq!(
            set_query_param("https://example.com/?a=1&b=2&a=3&c=4", "a", "x y&z").unwrap(),
            "https://example.com/?a=x+y%26z&b=2&c=4"
        );
        assert_eq!(
            set_query_param("https://example.com/path#frag", "new", "✓").unwrap(),
            "https://example.com/path?new=%E2%9C%93#frag"
        );
        assert!(set_query_param("invalid", "a", "b").is_err());
    }
}
