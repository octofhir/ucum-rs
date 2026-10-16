//! `[pH]` and `bit_s` are levels like `B` and `Np` (UCUM §21-22), defined by other
//! functions: `bit_s` is the binary logarithm of a ratio (`ld(1 1)`), so 1 bit_s is lg 2 B,
//! and `[pH]` is minus the decimal logarithm of a concentration in mol/l (`pH(1 mol/l)`),
//! a level with a reference like `B[V]`.

use octofhir_ucum::{
    AdvancedConversionContext, ErrorKind, analyse, convert_with_context, divide_by,
    get_canonical_units, is_comparable, multiply, unit_multiply,
};

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
fn a_bit_is_a_binary_level() {
    // These used to be 1, 1, 1 and 8: bit_s was a plain dimensionless unit
    assert_close(
        convert(1.0, "bit_s", "B"),
        core::f64::consts::LOG10_2,
        "1 bit_s",
    );
    assert_close(
        convert(1.0, "B", "bit_s"),
        core::f64::consts::LOG2_10,
        "1 B",
    );
    assert_close(
        convert(1.0, "bit_s", "Np"),
        core::f64::consts::LN_2,
        "1 bit_s",
    );
    // A ratio of 2^8 is 24.08 dB
    assert_close(
        convert(8.0, "bit_s", "dB"),
        80.0 * core::f64::consts::LOG10_2,
        "8 bit_s",
    );
    assert_eq!(convert(3.0, "bit_s", "bit_s"), 3.0);
    assert!(is_comparable("bit_s", "B").unwrap());
}

#[test]
fn a_bit_level_is_a_ratio_after_a_number() {
    // `8.bit_s` is the ratio 2^8, like `20.dB` is 10^2; it used to be 8
    assert_close(analyse("8.bit_s").unwrap().factor, 256.0, "8.bit_s");
    assert_close(
        analyse("bit_s").unwrap().factor,
        core::f64::consts::LOG10_2,
        "size of bit_s in bels",
    );
}

#[test]
fn a_bit_level_does_not_convert_to_a_proper_unit() {
    // These used to convert 1:1, or 1:100 for %
    assert_no_conversion("bit_s", "bit");
    assert_no_conversion("bit", "bit_s");
    assert_no_conversion("bit_s", "1");
    assert_no_conversion("bit_s", "%");
    assert_no_conversion("By", "bit_s");
}

#[test]
fn a_bit_level_takes_no_part_in_unit_arithmetic() {
    // UCUM §22.1; these used to return `bit_s.m`, 0.602 m and 2.408 s-1
    assert!(unit_multiply("bit_s", "m").is_err());
    for result in [
        multiply(1.0, "bit_s", 2.0, "m"),
        multiply(2.0, "m", 1.0, "bit_s"),
        divide_by(8.0, "bit_s", 1.0, "s"),
        // The same for the other levels and the prism diopter
        multiply(1.0, "B", 2.0, "m"),
        multiply(1.0, "dB", 2.0, "1"),
        divide_by(1.0, "m", 2.0, "Np"),
        multiply(1.0, "[p'diop]", 2.0, "m"),
    ] {
        let err = result.expect_err("a level in a product or a quotient");
        assert!(
            matches!(err.kind, ErrorKind::ConversionError { .. }),
            "{:?}",
            err.kind
        );
    }
    // A plain level still goes into terms
    assert!(analyse("bit_s/s").is_ok());
}

#[test]
fn a_level_has_no_canonical_unit() {
    // These used to return a factor as if the unit were linear: a canonical value
    // `value * factor + offset` cannot express a logarithm or a tangent
    for code in ["bit_s", "B", "dB", "[p'diop]"] {
        let err = get_canonical_units(code).expect_err(code);
        assert!(
            matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
            "{code}: {:?}",
            err.kind
        );
    }
}

#[test]
fn ph_is_a_level_of_a_concentration() {
    // UCUM §21.3; [pH] used to be dimensionless
    assert!(is_comparable("[pH]", "mol/l").unwrap());
    assert!(is_comparable("[pH]", "mmol/L").unwrap());
    assert!(!is_comparable("[pH]", "1").unwrap());
    assert!(!is_comparable("[pH]", "B").unwrap());
    assert_eq!(convert(7.4, "[pH]", "[pH]"), 7.4);
    // One pH unit is minus one bel: a tenth of the concentration
    assert_eq!(analyse("[pH]").unwrap().factor, -1.0);
    assert!(analyse("[pH]").unwrap().has_offset);
}

#[test]
fn ph_does_not_convert_to_other_units() {
    // These used to convert 1:1, or 1:100 for %
    assert_no_conversion("[pH]", "1");
    assert_no_conversion("[pH]", "%");
    assert_no_conversion("[pH]", "B");
    assert_no_conversion("[pH]", "bit_s");
    // Commensurable, but not linearly
    assert_no_conversion("[pH]", "mol/l");
    assert_no_conversion("mmol/L", "[pH]");
}

#[test]
fn ph_is_not_part_of_a_term() {
    // These used to evaluate as dimensionless, and `7.[pH]` as 7
    for expression in ["[pH].m", "[pH]/s", "m/[pH]", "[pH]2", "7.[pH]"] {
        let err = analyse(expression).expect_err(expression);
        assert!(
            matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
            "{expression}: {:?}",
            err.kind
        );
    }
    assert!(unit_multiply("[pH]", "m").is_err());
    assert!(multiply(7.0, "[pH]", 2.0, "m").is_err());
    assert!(divide_by(7.0, "[pH]", 2.0, "s").is_err());
    let err = get_canonical_units("[pH]").expect_err("[pH]");
    assert!(
        matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
        "{:?}",
        err.kind
    );
}
