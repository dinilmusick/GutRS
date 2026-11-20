//! Deterministic golden cases pinned to GutTS d14ba93.
use gutrs::arrays::{chunk, compact, flatten, last, unique, ChunkError, OneOrMany};
use gutrs::numbers::{clamp, in_range, is_finite_number, sum, RangeError};
use gutrs::urls::join_url;

#[test]
fn chunk_matches_lodash_and_validates_size() {
    assert_eq!(chunk(&[1, 2, 3], 2.0), Ok(vec![vec![1, 2], vec![3]]));
    assert_eq!(chunk::<i32>(&[], 2.0), Ok(vec![]));
    assert_eq!(chunk(&[1, 2], 999_999.0), Ok(vec![vec![1, 2]]));
    for size in [0.0, -2.0, 1.5, f64::NAN, f64::INFINITY] {
        assert_eq!(chunk(&[1, 2], size), Err(ChunkError::InvalidSize));
    }
}

#[test]
fn flatten_and_last_preserve_order_and_empty() {
    let input = vec![
        OneOrMany::One(1),
        OneOrMany::Many(vec![2, 3]),
        OneOrMany::One(4),
    ];
    assert_eq!(flatten(&input), vec![1, 2, 3, 4]);
    assert_eq!(last::<i32>(&[]), None);
    assert_eq!(last(&[1, 2, 3]), Some(&3));
}

#[test]
fn uniq_uses_js_same_value_zero_for_floats() {
    let values = [f64::NAN, 2.0, f64::NAN, -0.0, 0.0, 2.0];
    let result = unique(&values);
    assert_eq!(result.len(), 3);
    assert!(result[0].is_nan());
    assert_eq!(result[1], 2.0);
    assert_eq!(result[2], 0.0);
    assert_eq!(unique(&["a", "a", "b"]), vec!["a", "b"]);
}

#[test]
fn compact_removes_js_falsy_nullable_scalars() {
    assert_eq!(compact(&[None, Some(0), Some(2), Some(3)]), vec![2, 3]);
    assert_eq!(compact(&[Some(false), Some(true)]), vec![true]);
    assert_eq!(compact(&[Some(""), Some("yes"), None]), vec!["yes"]);
    let floating = compact(&[Some(0.0), Some(f64::NAN), Some(1.0)]);
    assert_eq!(floating, vec![1.0]);
}

#[test]
fn numeric_ranges_have_half_open_bounds() {
    assert_eq!(clamp(12.0, 0.0, 10.0), Ok(10.0));
    assert_eq!(clamp(-4.0, 0.0, 10.0), Ok(0.0));
    assert_eq!(clamp(5.0, 10.0, 0.0), Err(RangeError::MinExceedsMax));
    assert_eq!(in_range(0.0, 0.0, 2.0), Ok(true));
    assert_eq!(in_range(2.0, 0.0, 2.0), Ok(false));
    assert_eq!(in_range(1.0, 2.0, 0.0), Err(RangeError::MinExceedsMax));
    assert_eq!(in_range(f64::NAN, 0.0, 2.0), Ok(false));
    assert_eq!(sum(&[1.0, 2.0, 3.0]), 6.0);
    assert_eq!(sum(&[]), 0.0);
    assert!(!is_finite_number(f64::INFINITY));
    assert!(!is_finite_number(f64::NAN));
    assert!(is_finite_number(1.0));
}

#[test]
fn join_url_follows_gutts_string_rules() {
    assert_eq!(
        join_url("https://example.com/", &["/api/", "v1"]),
        "https://example.com/api/v1"
    );
    assert_eq!(join_url("/root///", &["///a///", "", "b"]), "/root/a/b");
    assert_eq!(join_url("", &["/", "a", "//"]), "a");
    assert_eq!(join_url("https://", &[]), "https:");
    assert_eq!(join_url("x", &["a//b"]), "x/a//b");
    assert_eq!(join_url("", &[]), "");
}
