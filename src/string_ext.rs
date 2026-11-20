//! Real UTF-8 case transformations and email validation.
//! This approximates lodash's deburr + word splitting, then uses Unicode-aware
//! heck boundaries. Validator.js and RFC email policies can still disagree.
use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase};
use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};
use unicode_segmentation::UnicodeSegmentation;

pub fn is_email(value: &str) -> bool {
    // Match validator.js default policy, which is stricter than the RFC
    // address parser about unquoted consecutive local-part dots, a required
    // top-level domain and bracketed IP-domain literals.
    if value.is_empty() || value.len() > 254 || value.trim() != value {
        return false;
    }
    let Some((local, domain)) = value.rsplit_once('@') else {
        return false;
    };
    if local.is_empty() || domain.starts_with('[') || !domain.contains('.') || domain.ends_with('.')
    {
        return false;
    }
    if !local.starts_with('"')
        && (local.starts_with('.') || local.ends_with('.') || local.contains(".."))
    {
        return false;
    }
    email_address::EmailAddress::is_valid(value)
}

fn latin(ch: char) -> bool {
    matches!(ch, 'A'..='Z' | 'a'..='z'
        | '\u{00c0}'..='\u{024f}' | '\u{1e00}'..='\u{1eff}')
}

/// Lodash deburr transliterates accented Latin letters before word splitting.
/// Do not strip vowel signs or diacritics from non-Latin writing systems.
fn deburr_latin(value: &str) -> String {
    let mut output = String::new();
    for grapheme in value.graphemes(true) {
        let first = grapheme.chars().next();
        if first.is_none_or(|ch| !latin(ch)) {
            output.push_str(grapheme);
            continue;
        }
        for ch in grapheme.nfd().filter(|ch| !is_combining_mark(*ch)) {
            match ch {
                'ß' => output.push_str("ss"),
                'Æ' => output.push_str("Ae"),
                'æ' => output.push_str("ae"),
                'Œ' => output.push_str("Oe"),
                'œ' => output.push_str("oe"),
                'Ø' => output.push('O'),
                'ø' => output.push('o'),
                'Ð' => output.push('D'),
                'ð' => output.push('d'),
                'Þ' => output.push_str("Th"),
                'þ' => output.push_str("th"),
                _ => output.push(ch),
            }
        }
    }
    output
}

/// Lodash capitalize first lowercases the *entire* string and then
/// uppercases its initial Unicode symbol, unlike simply uppercasing the
/// original first grapheme. E.g. Turkish İ lowercases into i + combining dot.
pub fn capitalize(value: &str) -> String {
    let lowered = value.to_lowercase();
    let mut chars = lowered.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    format!("{}{}", first.to_uppercase(), chars.as_str())
}

/// Lodash's word tokenizer treats digit/letter transitions as word boundaries.
/// Heck's default case conversion does not, so introduce only these lexical
/// boundaries before the Unicode-aware case conversion.
fn split_digit_boundaries(value: &str) -> String {
    let mut output = String::new();
    let mut previous: Option<char> = None;
    for ch in value.chars() {
        if previous.is_some_and(|last| {
            (last.is_alphabetic() && ch.is_numeric()) || (last.is_numeric() && ch.is_alphabetic())
        }) {
            output.push(' ');
        }
        output.push(ch);
        previous = Some(ch);
    }
    output
}

pub fn camel_case(value: &str) -> String {
    split_digit_boundaries(&deburr_latin(value)).to_lower_camel_case()
}
pub fn kebab_case(value: &str) -> String {
    split_digit_boundaries(&deburr_latin(value)).to_kebab_case()
}
pub fn snake_case(value: &str) -> String {
    split_digit_boundaries(&deburr_latin(value)).to_snake_case()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn upstream_unicode_and_boundaries() {
        assert_eq!(capitalize("fRED FROG"), "Fred frog");
        assert_eq!(camel_case("Foo Bar_baz"), "fooBarBaz");
        assert_eq!(kebab_case("XMLHttpRequest"), "xml-http-request");
        assert_eq!(snake_case("Déjà Vu"), "deja_vu");
        assert_eq!(snake_case("HTML5Parser_V2"), "html_5_parser_v_2");
        assert_eq!(capitalize("İSTANBUL ABC"), "I\u{307}stanbul abc");
        assert_eq!(camel_case("HTML5Parser_V2"), "html5ParserV2");
        assert_eq!(kebab_case("HTML5Parser_V2"), "html-5-parser-v-2");
        assert!(is_email("jane+tag@example.co.uk"));
        assert!(!is_email("x@@domain.com"));
        assert!(!is_email("a..b@example.com"));
        assert!(!is_email("user@example"));
        assert!(!is_email("user@[127.0.0.1]"));
    }
    #[test]
    fn unicode_non_latin_survives_latin_deburr() {
        assert_eq!(deburr_latin("किताब"), "किताब");
        assert_eq!(deburr_latin("café"), "cafe");
        assert_eq!(deburr_latin("Straße"), "Strasse");
    }
}
