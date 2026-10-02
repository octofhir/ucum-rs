//! Quantity and unit arithmetic must report a dimension exponent that does not fit in `i8`,
//! like the evaluator does, instead of overflowing or saturating.

use octofhir_ucum::{
    Dimension, ErrorKind, UcumError, divide_by, multiply, unit_divide, unit_multiply,
};

fn assert_overflow<T: std::fmt::Debug>(result: Result<T, UcumError>, what: &str) {
    let err = result.expect_err(what);
    assert!(
        matches!(err.kind, ErrorKind::PrecisionOverflow { .. }),
        "{what}: unexpected error {:?}",
        err.kind
    );
}

#[test]
fn quantity_arithmetic_reports_exponent_overflow() {
    // These two used to panic with "attempt to add with overflow" in debug builds
    assert_overflow(multiply(1.0, "m100", 1.0, "m100"), "multiply");
    assert_overflow(divide_by(1.0, "m100", 1.0, "/m100"), "divide_by");
}

#[test]
fn unit_arithmetic_reports_exponent_overflow() {
    // These two used to saturate at 127
    assert_overflow(unit_multiply("m100", "m100"), "unit_multiply");
    assert_overflow(unit_divide("m100", "/m100"), "unit_divide");
}

#[test]
fn negative_exponent_overflow_is_reported_too() {
    assert_overflow(unit_multiply("/m64", "/m65"), "unit_multiply");
    assert_overflow(multiply(1.0, "/m64", 1.0, "/m65"), "multiply");
    assert_overflow(unit_divide("/m64", "m65"), "unit_divide");
    assert_overflow(divide_by(1.0, "/m64", 1.0, "m65"), "divide_by");
}

#[test]
fn error_names_the_exponents_that_overflow() {
    let err = multiply(1.0, "m100", 1.0, "m100").unwrap_err();
    assert!(err.to_string().contains("100 + 100 * 1"), "{err}");
}

#[test]
fn exponents_in_range_still_work() {
    let product = multiply(2.0, "m60", 3.0, "m67").unwrap();
    assert_eq!(product.dimension, Dimension([0, 127, 0, 0, 0, 0, 0]));
    let quotient = unit_divide("m2", "m3").unwrap();
    assert_eq!(quotient.dimension, Dimension([0, -1, 0, 0, 0, 0, 0]));

    // -128 is the lowest exponent that fits
    let lowest = unit_divide("/m64", "m64").unwrap();
    assert_eq!(lowest.dimension, Dimension([0, -128, 0, 0, 0, 0, 0]));
    let lowest = divide_by(1.0, "/m64", 1.0, "m64").unwrap();
    assert_eq!(lowest.dimension, Dimension([0, -128, 0, 0, 0, 0, 0]));
    let highest = unit_multiply("m60", "m67").unwrap();
    assert_eq!(highest.dimension, Dimension([0, 127, 0, 0, 0, 0, 0]));
}
