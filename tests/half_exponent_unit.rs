//! `[m/s2/Hz^(1/2)]` (amplitude spectral density, UCUM §45) is `sqrt(1 m2/s4/Hz)`, of
//! dimension L.T-3/2. The dimension vector holds integers only, so the unit is isolated: it
//! is a valid code, but it converts and compares only to the same power of itself, and takes
//! no part in unit arithmetic.

use octofhir_ucum::{
    AdvancedConversionContext, ErrorKind, analyse, convert_with_context, get_canonical_units,
    is_comparable, multiply, unit_divide, unit_multiply, validate, validate_in_property,
};

const ASD: &str = "[m/s2/Hz^(1/2)]";

fn convert(value: f64, from: &str, to: &str) -> f64 {
    convert_with_context(value, from, to, &AdvancedConversionContext::default())
        .unwrap_or_else(|err| panic!("{value} {from} to {to}: {err}"))
        .value
}

fn assert_no_conversion(from: &str, to: &str) {
    let err = convert_with_context(1.0, from, to, &AdvancedConversionContext::default())
        .expect_err(&format!("{from} to {to}"));
    assert!(
        matches!(err.kind, ErrorKind::ConversionError { .. }),
        "{from} to {to}: {:?}",
        err.kind
    );
}

#[test]
fn the_unit_is_a_valid_code() {
    assert!(validate(ASD).is_ok());
    assert!(validate("[m/s2/Hz^(1/2)]2").is_ok());
    assert!(validate("/[m/s2/Hz^(1/2)]").is_ok());
    assert!(!analyse(ASD).unwrap().is_dimensionless);
}

#[test]
fn the_unit_converts_to_itself() {
    assert_eq!(convert(3.0, ASD, ASD), 3.0);
    assert_eq!(convert(3.0, "[m/s2/Hz^(1/2)].s", "s.[m/s2/Hz^(1/2)]"), 3.0);
    assert_eq!(
        convert(3.0, "[m/s2/Hz^(1/2)]/min", "[m/s2/Hz^(1/2)]/s"),
        0.05
    );
    assert_eq!(convert(2.0, "[m/s2/Hz^(1/2)]/[m/s2/Hz^(1/2)]", "%"), 200.0);
}

#[test]
fn the_unit_does_not_convert_to_a_dimensionless_unit() {
    // These used to convert 1:1, or 1:100 for %
    assert_no_conversion(ASD, "1");
    assert_no_conversion(ASD, "%");
    assert_no_conversion("1", ASD);
    assert_no_conversion(ASD, "[IU]");
    assert_no_conversion(ASD, "[m/s2/Hz^(1/2)]2");
    assert_no_conversion("[m/s2/Hz^(1/2)]2", "1");
    assert_no_conversion(ASD, "/[m/s2/Hz^(1/2)]");
}

#[test]
fn the_unit_is_commensurable_only_with_the_same_power_of_itself() {
    assert!(is_comparable(ASD, ASD).unwrap());
    assert!(is_comparable("[m/s2/Hz^(1/2)].m", "m.[m/s2/Hz^(1/2)]").unwrap());
    // This used to be true
    assert!(!is_comparable(ASD, "1").unwrap());
    assert!(!is_comparable(ASD, "[m/s2/Hz^(1/2)]2").unwrap());
    assert!(!is_comparable("[m/s2/Hz^(1/2)].m", "m").unwrap());
}

#[test]
fn the_unit_takes_no_part_in_unit_arithmetic() {
    assert!(unit_multiply(ASD, "s").is_err());
    assert!(unit_divide("m", ASD).is_err());
    assert!(multiply(1.0, ASD, 2.0, "1").is_err());
    let err = get_canonical_units(ASD).expect_err(ASD);
    assert!(
        matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
        "{:?}",
        err.kind
    );
    assert!(validate_in_property(ASD, "dimensionless").is_err());
}
