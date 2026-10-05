//! Quantity and unit arithmetic must report a result that is not a number (infinite or
//! NaN) or that is too small for an `f64`, like the evaluator does for `m/0`, instead of
//! returning it as a success.

mod common;

use common::assert_overflow;
use octofhir_ucum::{divide_by, multiply, unit_divide, unit_multiply};

#[test]
fn division_by_a_zero_unit_is_an_error() {
    // These two used to return an infinite factor and an infinite value
    assert_overflow(unit_divide("m", "0"), "unit_divide");
    assert_overflow(divide_by(10.0, "m", 2.0, "0"), "divide_by");
    // 0/0 used to be NaN
    assert_overflow(unit_divide("0", "0"), "unit_divide 0/0");
    assert_overflow(divide_by(0.0, "m", 2.0, "0"), "divide_by 0/0");
    assert_overflow(unit_divide("m", "0.s"), "unit_divide compound");
}

#[test]
fn value_out_of_range_is_an_error() {
    assert_overflow(multiply(1e300, "m", 1e300, "s"), "multiply");
    assert_overflow(divide_by(1e300, "m", 1e-300, "s"), "divide_by");
    // The values fit, the value scaled by the unit factor does not
    assert_overflow(multiply(1e300, "km", 1e5, "km"), "multiply scaled");
}

#[test]
fn only_the_result_has_to_fit() {
    // The product and the quotient of the values alone do not fit in an f64
    let product = multiply(1e155, "nm", 1e155, "nm").unwrap().value;
    assert!((product / 1e292 - 1.0).abs() < 1e-12, "{product}");
    let quotient = divide_by(1e300, "nm", 1e-10, "m").unwrap().value;
    assert!((quotient / 1e301 - 1.0).abs() < 1e-12, "{quotient}");
    // And these do not underflow to zero
    let product = multiply(1e-170, "Ym", 1e-170, "Ym").unwrap().value;
    assert!((product / 1e-292 - 1.0).abs() < 1e-12, "{product}");
    let quotient = divide_by(1e-170, "Ym", 1e170, "ym").unwrap().value;
    assert!((quotient / 1e-292 - 1.0).abs() < 1e-12, "{quotient}");
}

#[test]
fn non_finite_operands_are_an_error() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_overflow(multiply(value, "m", 2.0, "s"), "multiply left");
        assert_overflow(multiply(2.0, "m", value, "s"), "multiply right");
        assert_overflow(divide_by(value, "m", 2.0, "s"), "divide_by dividend");
        assert_overflow(divide_by(2.0, "m", value, "s"), "divide_by divisor");
    }
}

#[test]
fn error_names_the_operation_and_the_operands() {
    let err = unit_divide("m", "0").unwrap_err();
    assert!(err.to_string().contains("division: m / 0"), "{err}");
    let err = unit_divide("m", "0/s").unwrap_err();
    assert!(err.to_string().contains("division: m / (0/s)"), "{err}");
    let err = multiply(1e300, "m", 1e300, "s").unwrap_err();
    assert!(
        err.to_string()
            .contains("multiplication: 1e300 m * 1e300 s"),
        "{err}"
    );
}

#[test]
fn division_by_a_zero_value_is_the_same_error() {
    assert_overflow(divide_by(10.0, "m", 0.0, "s"), "divide_by");
    assert_overflow(divide_by(0.0, "m", 0.0, "s"), "divide_by 0/0");
}

#[test]
fn result_too_small_is_an_error() {
    // These used to be a silent zero
    assert_overflow(multiply(1e-200, "m", 1e-200, "s"), "multiply");
    assert_overflow(divide_by(1e-300, "m", 1e300, "s"), "divide_by");
    assert_overflow(divide_by(1.0, "m", 1e-320, "ym"), "divide_by divisor");
}

#[test]
fn finite_results_still_work() {
    assert_eq!(multiply(5.0, "m", 2.0, "s").unwrap().value, 10.0);
    assert_eq!(divide_by(10.0, "m", 2.0, "s").unwrap().value, 5.0);
    assert_eq!(unit_multiply("km", "km").unwrap().factor, 1e6);
    assert_eq!(unit_divide("km", "s").unwrap().factor, 1e3);

    // A zero is a number: only the division by it is rejected
    assert_eq!(unit_multiply("m", "0").unwrap().factor, 0.0);
    assert_eq!(unit_divide("0", "m").unwrap().factor, 0.0);
    assert_eq!(multiply(0.0, "m", 2.0, "s").unwrap().value, 0.0);
    assert_eq!(divide_by(0.0, "m", 2.0, "s").unwrap().value, 0.0);

    // A large value that fits is kept
    assert!(multiply(1e150, "m", 1e150, "s").unwrap().value.is_finite());
}
