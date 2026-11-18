//! RoundTo golden cases from GutTS 2.2.2 lodash-es round semantics.
use gutrs::numbers::{round_to, RoundError};

#[test]
fn decimal_exponent_shift_avoids_binary_multiplication_errors() {
    assert_eq!(round_to(1.005, 2.0), Ok(1.01));
    assert_eq!(round_to(1.004, 2.0), Ok(1.0));
    assert_eq!(round_to(123.4567, 3.0), Ok(123.457));
    assert_eq!(round_to(123456.789, 1.0), Ok(123456.8));
    assert_eq!(round_to(1.239, 2.0), Ok(1.24));
}

#[test]
fn javascript_ties_round_toward_positive_infinity() {
    assert_eq!(round_to(2.5, 0.0), Ok(3.0));
    assert_eq!(round_to(-1.5, 0.0), Ok(-1.0));
    assert_eq!(round_to(-1.005, 2.0), Ok(-1.0));
}

#[test]
fn negative_zero_and_non_finite_input_are_preserved() {
    let negative_half = round_to(-0.5, 0.0).unwrap();
    assert_eq!(negative_half, 0.0);
    assert!(negative_half.is_sign_negative());
    let small_negative = round_to(-0.01, 0.0).unwrap();
    assert!(small_negative.is_sign_negative());
    assert!(round_to(-0.0, 5.0).unwrap().is_sign_negative());
    assert_eq!(round_to(f64::INFINITY, 2.0), Ok(f64::INFINITY));
    assert!(round_to(f64::NAN, 2.0).unwrap().is_nan());
}

#[test]
fn invalid_precision_is_rejected_before_numeric_computation() {
    assert_eq!(round_to(1.0, -1.0), Err(RoundError::NegativePrecision));
    for precision in [f64::NAN, f64::INFINITY, 1.5] {
        assert_eq!(
            round_to(1.0, precision),
            Err(RoundError::NonIntegerPrecision)
        );
    }
    assert_eq!(
        round_to(1.0, 309.0),
        Err(RoundError::UnsupportedPrecision)
    );
}

#[test]
fn large_values_and_decimal_padding_do_not_overflow() {
    assert_eq!(round_to(f64::MAX, 2.0), Ok(f64::MAX));
    assert_eq!(round_to(1.23, 2.0), Ok(1.23));
    assert_eq!(round_to(1.0, 308.0), Ok(1.0));
}
