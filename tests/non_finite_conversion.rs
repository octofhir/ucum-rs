//! A conversion reports an operand that is not a real number (infinite or NaN) and a result
//! too large or too small for an `f64` instead of returning them as a success, and does not
//! panic on factors of any magnitude.

mod common;

use common::{assert_non_finite, assert_overflow};
use octofhir_ucum::{
    AdvancedConversionContext, DecimalPrecision, ErrorKind, Number, RoundingMode, convert,
    convert_with_context,
};

fn context(precision: DecimalPrecision) -> AdvancedConversionContext {
    AdvancedConversionContext {
        precision,
        ..Default::default()
    }
}

#[test]
fn a_zero_unit_is_not_ucum() {
    // A number is a positive integer (UCUM §8.1), so no unit has a factor of zero
    for (from, to) in [("1", "0"), ("0", "1"), ("0.m", "m"), ("m", "m/0")] {
        let err = convert(5.0, from, to).unwrap_err();
        assert!(
            matches!(err.kind, ErrorKind::InvalidExpression { .. }),
            "{from} to {to}: {:?}",
            err.kind
        );
    }
}

#[test]
fn converted_value_out_of_range_is_an_error() {
    assert_overflow(convert(1e300, "km", "nm"), "out of range");
    assert_overflow(convert(1e300, "Ym", "ym"), "out of range");
}

#[test]
fn converted_value_too_small_is_an_error() {
    // 1e-326 is not zero, and no f64 is closer to it than zero
    assert_overflow(convert(1e-323, "m", "km"), "too small");
    assert_overflow(convert(1e-300, "ym", "Ym"), "too small");
    assert_eq!(convert(1e-320, "m", "km").unwrap(), 1e-323);
}

#[test]
fn operand_that_is_not_a_number_is_an_error() {
    // A measurement value is a real number (UCUM §18.5)
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_non_finite(convert(value, "km", "m"), "operand");
        assert_non_finite(convert(value, "Cel", "K"), "operand of a temperature");
        assert_non_finite(convert(value, "dB", "B"), "operand of a level");
        assert_non_finite(convert(value, "[pH]", "mol/L"), "operand of a function");
    }
}

#[test]
fn rounding_does_not_hide_a_value_that_is_not_a_number() {
    for precision in [DecimalPrecision::Fixed(2), DecimalPrecision::Significant(3)] {
        assert_overflow(
            convert_with_context(1e300, "km", "nm", &context(precision)),
            "rounded",
        );
    }
}

#[test]
fn error_names_the_conversion() {
    let err = convert(1e300, "km", "nm").unwrap_err();
    assert!(err.to_string().contains("1e300"), "{err}");
}

#[test]
fn factors_of_any_magnitude_convert() {
    // The ratio 1e48 does not fit in a `Decimal`
    assert_eq!(convert(2.0, "Ym", "ym").unwrap(), 2e48);
    assert_eq!(convert(2e48, "ym", "Ym").unwrap(), 2.0);
    assert_eq!(convert(1.0, "Ym3", "ym3").unwrap(), 1e144);
}

#[test]
fn finite_conversions_still_work() {
    let default = context(DecimalPrecision::Default);
    assert_eq!(convert(1.0, "km", "m").unwrap(), 1000.0);
    assert_eq!(convert(0.0, "km", "m").unwrap(), 0.0);
    assert_eq!(convert(-2.5, "km", "m").unwrap(), -2500.0);
    let rounded = convert_with_context(1234.5678, "m", "km", &context(DecimalPrecision::Fixed(2)));
    assert_eq!(rounded.unwrap().value, 1.23);
    let significant = convert_with_context(
        1234.5678,
        "m",
        "km",
        &context(DecimalPrecision::Significant(2)),
    );
    assert_eq!(significant.unwrap().value, 1.2);
    assert_eq!(
        convert_with_context(1.0, "km", "m", &default)
            .unwrap()
            .factor,
        Some(Number::from(1000))
    );
}

#[test]
fn rounding_modes_are_applied() {
    let round = |rounding| {
        let context = AdvancedConversionContext {
            precision: DecimalPrecision::Fixed(0),
            rounding,
            ..Default::default()
        };
        convert_with_context(-1.6, "m", "m", &context)
            .unwrap()
            .value
    };
    assert_eq!(round(RoundingMode::Nearest), -2.0);
    assert_eq!(round(RoundingMode::Up), -1.0);
    assert_eq!(round(RoundingMode::Down), -2.0);
    assert_eq!(round(RoundingMode::Truncate), -1.0);
}

#[cfg(feature = "fhir")]
mod fhir {
    use octofhir_ucum::UcumError;
    use octofhir_ucum::fhir::{FhirError, FhirQuantity, are_equivalent, convert_quantity};

    fn ucum(err: FhirError) -> UcumError {
        match err {
            FhirError::UcumError(err) => err,
            other => UcumError::invalid_expression(&other.to_string()),
        }
    }

    #[test]
    fn a_zero_unit_is_not_ucum() {
        let five = FhirQuantity::with_ucum_code(5.0, "1");
        assert!(convert_quantity(&five, "0").is_err());
        let zero_unit = FhirQuantity::with_ucum_code(5.0, "0");
        assert!(zero_unit.to_ucum_quantity().is_err());
        assert!(are_equivalent(&zero_unit, &five).is_err());
    }

    #[test]
    fn factor_ratio_larger_than_a_decimal_is_converted() {
        let quantity = FhirQuantity::with_ucum_code(2.0, "Ym");
        let converted = convert_quantity(&quantity, "ym").unwrap();
        assert_eq!(converted.value, 2e48);

        let same = FhirQuantity::with_ucum_code(2e48, "ym");
        assert!(are_equivalent(&same, &quantity).unwrap());
        assert!(are_equivalent(&quantity, &same).unwrap());
    }

    #[test]
    fn converted_value_out_of_range_is_an_error() {
        let quantity = FhirQuantity::with_ucum_code(1e300, "km");
        super::assert_overflow(
            convert_quantity(&quantity, "nm").map_err(ucum),
            "out of range",
        );
        let nan = FhirQuantity::with_ucum_code(f64::NAN, "km");
        super::assert_non_finite(convert_quantity(&nan, "m").map_err(ucum), "NaN");
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
