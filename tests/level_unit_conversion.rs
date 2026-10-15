//! Conversions between level units (UCUM §22: `B`, `Np` and their prefixes) are linear: a
//! level in decibels is a tenth of the level in bels, and a level in nepers is the natural
//! logarithm of the ratio, `ln 10` times the level in bels. A level does not convert
//! linearly to a unit of another kind, so that conversion is an error.

use octofhir_ucum::{AdvancedConversionContext, ErrorKind, analyse, convert_with_context};

fn convert(value: f64, from: &str, to: &str) -> f64 {
    convert_with_context(value, from, to, &AdvancedConversionContext::default())
        .unwrap_or_else(|err| panic!("{value} {from} to {to}: {err}"))
        .value
}

fn assert_close(actual: f64, expected: f64, what: &str) {
    assert!(
        (actual - expected).abs() < 1e-12 * expected.abs().max(1.0),
        "{what}: {actual}, expected {expected}"
    );
}

#[test]
fn prefixes_scale_the_level() {
    // These used to be 1, 1, 1 and 1
    assert_eq!(convert(1.0, "dB", "B"), 0.1);
    assert_eq!(convert(1.0, "B", "dB"), 10.0);
    assert_eq!(convert(1.0, "cNp", "Np"), 0.01);
    assert_eq!(convert(1000.0, "mB", "B"), 1.0);
    assert_eq!(convert(35.0, "dB", "B"), 3.5);
}

#[test]
fn bels_and_nepers() {
    use core::f64::consts::{LN_10, LOG10_E};
    // A ratio of 10 is 1 B and ln 10 Np; these used to be 1 and 20
    assert_close(convert(1.0, "B", "Np"), LN_10, "1 B");
    assert_close(convert(1.0, "Np", "B"), LOG10_E, "1 Np");
    assert_close(convert(20.0, "dB", "Np"), 2.0 * LN_10, "20 dB");
    assert_close(convert(1.0, "cNp", "dB"), 0.1 * LOG10_E, "1 cNp");
}

#[test]
fn compound_levels_convert_too() {
    assert_eq!(convert(1.0, "dB/s", "B/s"), 0.1);
}

#[test]
fn factor_of_a_level_is_its_size_in_bels() {
    assert_eq!(analyse("dB").unwrap().factor, 0.1);
    assert_eq!(analyse("B").unwrap().factor, 1.0);
    // `<number>.<level>` is still the ratio of that level: 10 dB is a ratio of 10
    assert_close(analyse("10.dB").unwrap().factor, 10.0, "10.dB");
}

#[test]
fn level_to_another_kind_of_unit_is_an_error() {
    for (from, to) in [
        // These used to convert 1:1
        ("dB", "1"),
        ("1", "Np"),
        ("[p'diop]", "B"),
        ("B", "[p'diop]"),
        ("B", "B[SPL]"),
        ("dB/s", "/s"),
    ] {
        let err = convert_with_context(1.0, from, to, &AdvancedConversionContext::default())
            .expect_err(&format!("{from} to {to}"));
        assert!(
            matches!(err.kind, ErrorKind::ConversionError { .. }),
            "{from} to {to}: {:?}",
            err.kind
        );
    }
}

#[test]
fn other_conversions_are_unchanged() {
    assert_eq!(convert(1.0, "B[SPL]", "dB[SPL]"), 10.0);
    assert_eq!(convert(1.0, "[p'diop]", "[p'diop]"), 1.0);
    assert_eq!(convert(1.0, "km", "m"), 1000.0);
}

#[cfg(feature = "fhir")]
mod fhir {
    use octofhir_ucum::fhir::{FhirQuantity, are_equivalent, convert_quantity};

    #[test]
    fn convert_quantity_scales_levels() {
        let level = FhirQuantity::with_ucum_code(1.0, "dB");
        assert_eq!(convert_quantity(&level, "B").unwrap().value, 0.1);
        assert!(convert_quantity(&level, "1").is_err());
    }

    #[test]
    fn are_equivalent_scales_levels() {
        let decibels = FhirQuantity::with_ucum_code(10.0, "dB");
        let bel = FhirQuantity::with_ucum_code(1.0, "B");
        assert!(are_equivalent(&decibels, &bel).unwrap());
        let ratio = FhirQuantity::with_ucum_code(10.0, "1");
        assert!(are_equivalent(&decibels, &ratio).is_err());
    }
}
