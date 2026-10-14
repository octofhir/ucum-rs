//! A conversion between units with an offset (temperatures) must apply both offsets, and
//! divide by the factor of the target unit: `(value * from.factor + from.offset -
//! to.offset) / to.factor`.

use octofhir_ucum::{AdvancedConversionContext, convert_with_context};

fn convert(value: f64, from: &str, to: &str) -> f64 {
    convert_with_context(value, from, to, &AdvancedConversionContext::default())
        .unwrap_or_else(|err| panic!("{value} {from} to {to}: {err}"))
        .value
}

fn assert_close(actual: f64, expected: f64, what: &str) {
    assert!(
        (actual - expected).abs() < 1e-9 * expected.abs().max(1.0),
        "{what}: {actual}, expected {expected}"
    );
}

#[test]
fn celsius_and_fahrenheit() {
    // These used to be 17.78, 0 and 117.78: the offset was not divided by the target factor
    assert_close(convert(0.0, "Cel", "[degF]"), 32.0, "0 Cel");
    assert_close(convert(-40.0, "Cel", "[degF]"), -40.0, "-40 Cel");
    assert_close(convert(100.0, "Cel", "[degF]"), 212.0, "100 Cel");
    assert_close(convert(212.0, "[degF]", "Cel"), 100.0, "212 [degF]");
    assert_close(convert(32.0, "[degF]", "Cel"), 0.0, "32 [degF]");
}

#[test]
fn kelvin_and_rankine() {
    assert_close(convert(0.0, "Cel", "K"), 273.15, "0 Cel to K");
    assert_close(convert(273.15, "K", "Cel"), 0.0, "273.15 K to Cel");
    assert_close(convert(32.0, "[degF]", "K"), 273.15, "32 [degF] to K");
    assert_close(convert(0.0, "Cel", "[degR]"), 491.67, "0 Cel to [degR]");
    assert_close(convert(1.0, "mK", "K"), 0.001, "mK to K");
}

#[test]
fn reported_factor_and_offset_give_the_value() {
    for (value, from, to) in [
        (0.0, "Cel", "[degF]"),
        (37.0, "Cel", "[degF]"),
        (98.6, "[degF]", "Cel"),
        (5.0, "km", "m"),
    ] {
        let result =
            convert_with_context(value, from, to, &AdvancedConversionContext::default()).unwrap();
        assert_close(
            value * result.factor + result.offset,
            result.value,
            &format!("{value} {from} to {to}"),
        );
    }
}

#[test]
fn units_without_offset_are_unchanged() {
    assert_eq!(convert(1.0, "km", "m"), 1000.0);
    assert_close(convert(100.0, "kPa", "mm[Hg]"), 750.0637554192107, "kPa");
}

#[cfg(feature = "fhir")]
mod fhir {
    use super::assert_close;
    use octofhir_ucum::fhir::{FhirQuantity, are_equivalent, convert_quantity};

    #[test]
    fn convert_quantity_applies_the_offsets() {
        // These used to be 0 K and 0 [degF]: offsets were ignored
        let zero = FhirQuantity::with_ucum_code(0.0, "Cel");
        assert_close(
            convert_quantity(&zero, "K").unwrap().value,
            273.15,
            "Cel to K",
        );
        assert_close(
            convert_quantity(&zero, "[degF]").unwrap().value,
            32.0,
            "Cel to [degF]",
        );
    }

    #[test]
    fn are_equivalent_applies_the_offsets() {
        let celsius = FhirQuantity::with_ucum_code(0.0, "Cel");
        let kelvin = FhirQuantity::with_ucum_code(273.15, "K");
        let zero_kelvin = FhirQuantity::with_ucum_code(0.0, "K");
        let fahrenheit = FhirQuantity::with_ucum_code(32.0, "[degF]");
        assert!(are_equivalent(&celsius, &kelvin).unwrap());
        assert!(are_equivalent(&kelvin, &celsius).unwrap());
        assert!(are_equivalent(&fahrenheit, &celsius).unwrap());
        // This used to be true
        assert!(!are_equivalent(&celsius, &zero_kelvin).unwrap());
    }
}
