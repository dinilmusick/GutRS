//! Exponent-aware numeric rounding from GutTS 2.2.2.
//!
//! Reference: dinilmusick/GutTS@d14ba93ef203135ad7ac536c8ad1ec1251b5c0b5,
//! src/numbers/number-operations.ts, which wraps lodash-es round.
//! A scientific-notation shift avoids common multiply-by-10 binary artifacts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundError {
    NonIntegerPrecision,
    NegativePrecision,
    UnsupportedPrecision,
}

fn js_math_round(value: f64) -> f64 {
    if !value.is_finite() || value == 0.0 {
        return value;
    }
    // Adding 0.5 before floor can round a value just below a half-tie
    // up to the tie itself (e.g. the f64 immediately below 0.5).
    // Compare the fractional part instead, preserving JS tie direction.
    let lower = value.floor();
    let rounded = if value - lower >= 0.5 {
        lower + 1.0
    } else {
        lower
    };
    if rounded == 0.0 && value.is_sign_negative() {
        -0.0
    } else {
        rounded
    }
}

fn shift_exponent(value: f64, amount: i32) -> Option<f64> {
    let scientific = format!("{value:e}");
    let (significand, exponent) = scientific.split_once('e')?;
    let exponent: i32 = exponent.parse().ok()?;
    let adjusted = exponent.checked_add(amount)?;
    format!("{significand}e{adjusted}").parse::<f64>().ok()
}

/// Decimal rounding with JavaScript ties-to-positive-infinity behavior,
/// including negative zero and exponent-aware shifting.
///
/// `decimals` is a floating-point parameter so malformed/non-integral
/// JavaScript numeric precision values can fail closed. This first Rust
/// implementation explicitly rejects precision above 308 rather than silently
/// guessing about unsupported extreme-number cases.
pub fn round_to(value: f64, decimals: f64) -> Result<f64, RoundError> {
    if !decimals.is_finite() || decimals.fract() != 0.0 {
        return Err(RoundError::NonIntegerPrecision);
    }
    if decimals < 0.0 {
        return Err(RoundError::NegativePrecision);
    }
    if decimals > 308.0 {
        return Err(RoundError::UnsupportedPrecision);
    }
    if !value.is_finite() || value == 0.0 {
        return Ok(value);
    }
    if decimals == 0.0 {
        return Ok(js_math_round(value));
    }

    let precision = decimals as i32;
    let shifted = shift_exponent(value, precision).ok_or(RoundError::UnsupportedPrecision)?;
    if !shifted.is_finite() {
        // More decimal places cannot alter the integral value of a large f64.
        return Ok(value);
    }
    let rounded = js_math_round(shifted);
    if rounded == 0.0 {
        return Ok(if value.is_sign_negative() { -0.0 } else { 0.0 });
    }
    shift_exponent(rounded, -precision).ok_or(RoundError::UnsupportedPrecision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_decimal_ties_with_javascript_semantics() {
        assert_eq!(round_to(1.005, 2.0), Ok(1.01));
        assert_eq!(round_to(1.2345, 2.0), Ok(1.23));
        assert_eq!(round_to(-1.5, 0.0), Ok(-1.0));
        assert_eq!(round_to(-2.5, 0.0), Ok(-2.0));
        assert_eq!(round_to(2.5, 0.0), Ok(3.0));
        assert_eq!(round_to(f64::from_bits(0x3fdfffffffffffff), 0.0), Ok(0.0));
        assert_eq!(round_to(f64::from_bits(0xbfe0000000000001), 0.0), Ok(-1.0));
        assert_eq!(round_to(0.5, 0.0), Ok(1.0));
        assert!(round_to(-0.5, 0.0).unwrap().is_sign_negative());
    }

    #[test]
    fn preserves_negative_zero_and_nonfinite_values() {
        let zero = round_to(-0.0001, 2.0).unwrap();
        assert_eq!(zero, 0.0);
        assert!(zero.is_sign_negative());
        assert!(round_to(-0.0, 2.0).unwrap().is_sign_negative());
        assert!(round_to(f64::NAN, 2.0).unwrap().is_nan());
        assert_eq!(round_to(f64::INFINITY, 2.0), Ok(f64::INFINITY));
    }

    #[test]
    fn rejects_invalid_precision_without_silent_coercion() {
        assert_eq!(round_to(1.0, -1.0), Err(RoundError::NegativePrecision));
        assert_eq!(round_to(1.0, 1.5), Err(RoundError::NonIntegerPrecision));
        assert_eq!(
            round_to(1.0, f64::NAN),
            Err(RoundError::NonIntegerPrecision)
        );
        assert_eq!(round_to(1.0, 309.0), Err(RoundError::UnsupportedPrecision));
    }
}
