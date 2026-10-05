//! Helpers shared by the integration tests.

use octofhir_ucum::{ErrorKind, UcumError};

/// Asserts that `result` is a `PrecisionOverflow` error.
pub fn assert_overflow<T: std::fmt::Debug>(result: Result<T, UcumError>, what: &str) {
    let err = result.expect_err(what);
    assert!(
        matches!(err.kind, ErrorKind::PrecisionOverflow { .. }),
        "{what}: unexpected error {:?}",
        err.kind
    );
}
