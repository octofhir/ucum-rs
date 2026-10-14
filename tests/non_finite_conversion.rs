//! A conversion must report a result that is not a number (infinite or NaN) instead of
//! returning it as a success, and must not panic on a factor ratio that does not fit in a
//! `Decimal`.

mod common;

use common::assert_overflow;
use octofhir_ucum::{AdvancedConversionContext, DecimalPrecision, convert_with_context};

fn context(precision: DecimalPrecision) -> AdvancedConversionContext {
    AdvancedConversionContext {
        precision,
        ..Default::default()
    }
}

#[test]
fn conversion_to_a_zero_unit_is_an_error() {
    let default = context(DecimalPrecision::Default);
    // This used to be an infinite value
    assert_overflow(convert_with_context(5.0, "1", "0", &default), "to zero");
    // 0 * inf used to be NaN
    assert_overflow(
        convert_with_context(0.0, "1", "0", &default),
        "zero to zero",
    );
    assert_overflow(convert_with_context(5.0, "0", "0", &default), "0/0");
}

#[test]
fn converted_value_out_of_range_is_an_error() {
    let default = context(DecimalPrecision::Default);
    assert_overflow(
        convert_with_context(1e300, "km", "nm", &default),
        "out of range",
    );
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_overflow(convert_with_context(value, "km", "m", &default), "operand");
    }
}

#[test]
fn rounding_does_not_hide_a_value_that_is_not_a_number() {
    for precision in [DecimalPrecision::Fixed(2), DecimalPrecision::Significant(3)] {
        assert_overflow(
            convert_with_context(5.0, "1", "0", &context(precision)),
            "rounded",
        );
    }
}

#[test]
fn error_names_the_conversion() {
    let default = context(DecimalPrecision::Default);
    let err = convert_with_context(5.0, "1", "0", &default).unwrap_err();
    assert!(err.to_string().contains("conversion: 5.0 1 to 0"), "{err}");
}

#[test]
fn finite_conversions_still_work() {
    let default = context(DecimalPrecision::Default);
    assert_eq!(
        convert_with_context(1.0, "km", "m", &default)
            .unwrap()
            .value,
        1000.0
    );
    assert_eq!(
        convert_with_context(0.0, "km", "m", &default)
            .unwrap()
            .value,
        0.0
    );
    // A zero source unit is a number
    assert_eq!(
        convert_with_context(5.0, "0", "1", &default).unwrap().value,
        0.0
    );
    let rounded = convert_with_context(1234.5678, "m", "km", &context(DecimalPrecision::Fixed(2)));
    assert_eq!(rounded.unwrap().value, 1.23);
}

#[cfg(feature = "fhir")]
mod fhir {
    use octofhir_ucum::fhir::{FhirError, FhirQuantity, are_equivalent, convert_quantity};
    use octofhir_ucum::{ErrorKind, UcumError};

    fn assert_overflow<T: std::fmt::Debug>(result: Result<T, FhirError>, what: &str) {
        let result = result.map_err(|err| match err {
            FhirError::UcumError(err) => err,
            other => UcumError::new(
                ErrorKind::InvalidExpression {
                    reason: other.to_string(),
                },
                other.to_string(),
            ),
        });
        super::assert_overflow(result, what);
    }

    #[test]
    fn conversion_to_a_zero_unit_is_an_error() {
        // These used to panic with "Division by zero"
        let five = FhirQuantity::with_ucum_code(5.0, "1");
        assert_overflow(convert_quantity(&five, "0"), "convert_quantity");
        let zero_unit = FhirQuantity::with_ucum_code(5.0, "0");
        assert_overflow(are_equivalent(&zero_unit, &five), "are_equivalent");
    }

    #[test]
    fn factor_ratio_larger_than_a_decimal_is_converted() {
        // The ratio 1e48 does not fit in a `Decimal`: these used to panic
        let quantity = FhirQuantity::with_ucum_code(2.0, "Ym");
        let converted = convert_quantity(&quantity, "ym").unwrap();
        assert!(
            (converted.value / 2e48 - 1.0).abs() < 1e-12,
            "{converted:?}"
        );

        let same = FhirQuantity::with_ucum_code(2e48, "ym");
        assert!(are_equivalent(&same, &quantity).unwrap());
        assert!(are_equivalent(&quantity, &same).unwrap());
    }

    #[test]
    fn converted_value_out_of_range_is_an_error() {
        let quantity = FhirQuantity::with_ucum_code(1e300, "km");
        assert_overflow(convert_quantity(&quantity, "nm"), "out of range");
        let nan = FhirQuantity::with_ucum_code(f64::NAN, "km");
        assert_overflow(convert_quantity(&nan, "m"), "NaN");
    }

    #[test]
    fn finite_conversions_still_work() {
        let quantity = FhirQuantity::with_ucum_code(1.0, "km");
        assert_eq!(convert_quantity(&quantity, "m").unwrap().value, 1000.0);
        let grams = FhirQuantity::with_ucum_code(1.0, "g");
        let milligrams = FhirQuantity::with_ucum_code(1000.0, "mg");
        assert!(are_equivalent(&grams, &milligrams).unwrap());
    }
}
