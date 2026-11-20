//! Portable GutTS utility contracts. See PARITY.md for gaps and pinned source.
#![forbid(unsafe_code)]

pub mod path_safety;

pub mod arrays {
    #[derive(Clone, Debug, PartialEq)]
    pub enum OneOrMany<T> {
        One(T),
        Many(Vec<T>),
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ChunkError {
        InvalidSize,
    }

    pub fn chunk<T: Clone>(values: &[T], size: f64) -> Result<Vec<Vec<T>>, ChunkError> {
        if !size.is_finite() || size <= 0.0 || size.fract() != 0.0 {
            return Err(ChunkError::InvalidSize);
        }
        if values.is_empty() {
            return Ok(Vec::new());
        }
        if size >= values.len() as f64 {
            return Ok(vec![values.to_vec()]);
        }
        Ok(values
            .chunks(size as usize)
            .map(|part| part.to_vec())
            .collect())
    }

    pub fn flatten<T: Clone>(values: &[OneOrMany<T>]) -> Vec<T> {
        let mut result = Vec::new();
        for value in values {
            match value {
                OneOrMany::One(item) => result.push(item.clone()),
                OneOrMany::Many(items) => result.extend(items.iter().cloned()),
            }
        }
        result
    }

    pub fn last<T>(values: &[T]) -> Option<&T> {
        values.last()
    }

    // SameValueZero is lodash uniq's equality for scalar values.
    pub trait SameValueZero {
        fn same_value_zero(&self, other: &Self) -> bool;
    }

    impl SameValueZero for f64 {
        fn same_value_zero(&self, other: &Self) -> bool {
            self == other || (self.is_nan() && other.is_nan())
        }
    }

    impl SameValueZero for f32 {
        fn same_value_zero(&self, other: &Self) -> bool {
            self == other || (self.is_nan() && other.is_nan())
        }
    }

    macro_rules! scalar_same_value_zero {
        ($($ty:ty),+ $(,)?) => {
            $(
                impl SameValueZero for $ty {
                    fn same_value_zero(&self, other: &Self) -> bool {
                        self == other
                    }
                }
            )+
        };
    }

    scalar_same_value_zero!(
        bool, char, String, &str, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );

    pub fn unique<T: Clone + SameValueZero>(values: &[T]) -> Vec<T> {
        let mut seen: Vec<T> = Vec::new();
        for value in values {
            if !seen.iter().any(|existing| existing.same_value_zero(value)) {
                seen.push(value.clone());
            }
        }
        seen
    }

    pub trait Truthy {
        fn is_truthy(&self) -> bool;
    }

    impl Truthy for bool {
        fn is_truthy(&self) -> bool {
            *self
        }
    }

    impl Truthy for f64 {
        fn is_truthy(&self) -> bool {
            *self != 0.0 && !self.is_nan()
        }
    }

    impl Truthy for f32 {
        fn is_truthy(&self) -> bool {
            *self != 0.0 && !self.is_nan()
        }
    }

    macro_rules! number_truthy {
        ($($ty:ty),+ $(,)?) => {
            $(
                impl Truthy for $ty {
                    fn is_truthy(&self) -> bool {
                        *self != 0
                    }
                }
            )+
        };
    }

    number_truthy!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

    impl Truthy for String {
        fn is_truthy(&self) -> bool {
            !self.is_empty()
        }
    }

    impl Truthy for &str {
        fn is_truthy(&self) -> bool {
            !self.is_empty()
        }
    }

    pub fn compact<T: Clone + Truthy>(values: &[Option<T>]) -> Vec<T> {
        values
            .iter()
            .filter_map(|value| value.as_ref().filter(|item| item.is_truthy()).cloned())
            .collect()
    }
}

pub mod rounding;

pub mod numbers {
    pub use crate::rounding::{round_to, RoundError};
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RangeError {
        MinExceedsMax,
    }

    pub fn clamp(value: f64, min: f64, max: f64) -> Result<f64, RangeError> {
        if min > max {
            return Err(RangeError::MinExceedsMax);
        }
        if value.is_nan() {
            return Ok(value);
        }
        if value < min {
            Ok(min)
        } else if value > max {
            Ok(max)
        } else {
            Ok(value)
        }
    }

    pub fn in_range(value: f64, min: f64, max: f64) -> Result<bool, RangeError> {
        if min > max {
            return Err(RangeError::MinExceedsMax);
        }
        Ok(value >= min && value < max)
    }

    pub fn is_finite_number(value: f64) -> bool {
        value.is_finite()
    }

    pub fn sum(values: &[f64]) -> f64 {
        values.iter().fold(0.0, |total, item| total + item)
    }
}

pub mod urls {
    // String joining, deliberately NOT RFC3986 resolution.
    pub fn join_url(base: &str, parts: &[&str]) -> String {
        let mut segments: Vec<&str> = Vec::with_capacity(parts.len() + 1);
        let base = base.trim_end_matches('/');
        if !base.is_empty() {
            segments.push(base);
        }
        for part in parts {
            let part = part.trim_matches('/');
            if !part.is_empty() {
                segments.push(part);
            }
        }
        segments.join("/")
    }
}

pub mod strings;
pub mod truncate;

pub mod trusted_api;

/// WHATWG-compatible URL query operations (portable GutTS subset).
pub mod url_operations;

/// Role-tagged application, deployment, user and workspace identities.
pub mod identity;
/// JSON parse/stringify and typed JSON values.
pub mod json_utils;
/// Explicit shallow JSON object operations.
pub mod objects;

/// Shape validation for OAuth clients, capabilities and bounded invocations.
pub mod auth_contracts;

/// Luth session and user wire data, separate from authentication authority.
pub mod auth_types;
/// Serializable host-agnostic engineering request, artifact and grant wire types.
pub mod engineering;

/// Pinned model metadata, native load/unload adapter and shared residency.
pub mod models;

/// API definition registry, FOD node conversion and executable utility dispatch.
pub mod api;

/// Unicode-aware native case conversion and email-address validation.
pub mod string_ext;

/// Deterministic semantic parsing and model-backed resolution via explicit host.
pub mod semantic_input;
