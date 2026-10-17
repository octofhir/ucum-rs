//! Arbitrary units (UCUM §24-26): units whose meaning depends on the measurement procedure.
//! They are not commensurable with any other unit (§24.3), and a term with an arbitrary unit
//! is itself arbitrary (§25.1). The same arbitrary atoms with the same exponents stay
//! commensurable, so `[IU]/mL` converts to `[IU]/L`; `[IU]` is defined as 1 `[iU]` in the
//! tables and is the same unit.

use octofhir_ucum::{
    ErrorKind, SpecialKind, analyse, convert, find_unit, get_canonical_units, is_comparable,
    multiply, unit_divide, unit_multiply, validate, validate_in_property,
};

fn assert_no_conversion(from: &str, to: &str) {
    let err = convert(1.0, from, to).expect_err(&format!("{from} to {to}"));
    assert!(
        matches!(err.kind, ErrorKind::ConversionError { .. }),
        "{from} to {to}: {:?}",
        err.kind
    );
    assert!(!is_comparable(from, to).unwrap(), "{from} ~ {to}");
}

#[test]
fn arbitrary_units_are_not_commensurable_with_proper_units() {
    assert_no_conversion("[IU]", "1");
    assert_no_conversion("[IU]", "%");
    assert_no_conversion("1", "[IU]");
    assert_no_conversion("[arb'U]", "10*3");
    assert_no_conversion("[IU]/L", "/L");
    assert_no_conversion("[IU]/mL", "/mL");
}

#[test]
fn arbitrary_units_are_not_commensurable_with_each_other() {
    // UCUM §24 note: since version 1.7
    assert_no_conversion("[IU]", "[arb'U]");
    assert_no_conversion("[CFU]", "[PFU]");
    assert_no_conversion("[IU]/mL", "[arb'U]/mL");
    assert_no_conversion("[IU]", "[IU]2");
    assert_no_conversion("[hp_X]", "[hp'_X]");
}

#[test]
fn the_same_arbitrary_atoms_convert() {
    assert_eq!(convert(1.0, "[IU]/mL", "[IU]/L").unwrap(), 1000.0);
    assert_eq!(convert(1.0, "k[IU]", "[IU]").unwrap(), 1000.0);
    assert_eq!(convert(3.0, "[CFU]", "[CFU]").unwrap(), 3.0);
    assert_eq!(convert(1.0, "[IU]/(10*3.mL)", "[IU]/L").unwrap(), 1.0);
    assert!(is_comparable("[IU]/mL", "[IU]/L").unwrap());
}

#[test]
fn iu_is_defined_as_the_metric_iu() {
    // `[IU]` is 1 `[iU]` in the UCUM tables
    assert_eq!(convert(1.0, "[IU]", "[iU]").unwrap(), 1.0);
    assert_eq!(convert(1.0, "m[iU]", "[IU]").unwrap(), 0.001);
}

#[test]
fn arbitrary_units_take_prefixes_only_when_metric() {
    for code in ["k[IU]", "m[IU]/L", "u[iU]"] {
        assert!(validate(code).is_ok(), "{code}");
    }
    for code in ["k[arb'U]", "k[CFU]"] {
        assert!(validate(code).is_err(), "{code}");
    }
}

#[test]
fn unknown_bracket_units_are_rejected() {
    assert!(validate("[custom'U]").is_err());
    // Codes are case sensitive: the unit is `[beth'U]`
    assert!(validate("[BETH'U]").is_err());
    assert!(validate("[beth'U]").is_ok());
}

#[test]
fn analysis_lists_the_arbitrary_atoms() {
    let analysis = analyse("[IU]/mL").unwrap();
    assert_eq!(analysis.arbitrary, vec![("[iU]", 1)]);
    assert_eq!(analysis.special, None);
    assert!(!analysis.is_dimensionless);
    assert_eq!(analysis.dimension.to_string(), "L-3");
    assert_eq!(analyse("[arb'U]").unwrap().arbitrary, vec![("[arb'U]", 1)]);
    assert_eq!(analyse("5.[IU]").unwrap().factor, 5.0);
    assert_eq!(find_unit("[IU]").unwrap().special, SpecialKind::Arbitrary);
}

#[test]
fn the_canonical_unit_keeps_the_arbitrary_atoms() {
    let canonical = get_canonical_units("[IU]/mL").unwrap();
    assert_eq!(canonical.unit, "m-3.[iU]");
    assert_eq!(canonical.factor, 1e6);
    assert_eq!(get_canonical_units("[arb'U]").unwrap().unit, "[arb'U]");
}

#[test]
fn arithmetic_keeps_the_arbitrary_atoms() {
    let product = multiply(2.0, "[IU]", 3.0, "mL").unwrap();
    assert_eq!(product.unit, "m3.[iU]");
    assert!((product.value - 6e-6).abs() < 1e-20);
    let product = multiply(1.0, "[IU]", 1.0, "[arb'U]").unwrap();
    assert_eq!(product.unit, "[arb'U].[iU]");
    let quotient = unit_divide("[IU]", "L").unwrap();
    assert_eq!(quotient.expression, "[IU]/L");
    assert!(!quotient.is_dimensionless);
    let ratio = unit_divide("[IU]", "[IU]").unwrap();
    assert!(ratio.is_dimensionless);
    assert_eq!(unit_multiply("[IU]", "L").unwrap().expression, "[IU].L");
}

#[test]
fn an_arbitrary_unit_is_not_dimensionless() {
    assert!(!validate_in_property("[IU]", "dimensionless").unwrap());
    assert!(validate_in_property("[IU]", "arbitrary").unwrap());
}
