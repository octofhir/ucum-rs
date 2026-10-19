//! Helpers shared by the integration tests.
#![allow(dead_code)]

use octofhir_ucum::{ErrorKind, UcumError};

fn assert_kind<T: std::fmt::Debug>(
    result: Result<T, UcumError>,
    what: &str,
    expected: fn(&ErrorKind) -> bool,
) {
    let err = result.expect_err(what);
    assert!(
        expected(&err.kind),
        "{what}: unexpected error {:?}",
        err.kind
    );
}

/// Asserts that `result` is a `PrecisionOverflow` error.
pub fn assert_overflow<T: std::fmt::Debug>(result: Result<T, UcumError>, what: &str) {
    assert_kind(result, what, |kind| {
        matches!(kind, ErrorKind::PrecisionOverflow { .. })
    });
}

/// Asserts that `result` is a `NonFiniteValue` error.
pub fn assert_non_finite<T: std::fmt::Debug>(result: Result<T, UcumError>, what: &str) {
    assert_kind(result, what, |kind| {
        matches!(kind, ErrorKind::NonFiniteValue { .. })
    });
}

/// Asserts that `result` is a `DivisionByZero` error.
pub fn assert_division_by_zero<T: std::fmt::Debug>(result: Result<T, UcumError>, what: &str) {
    assert_kind(result, what, |kind| {
        matches!(kind, ErrorKind::DivisionByZero { .. })
    });
}
