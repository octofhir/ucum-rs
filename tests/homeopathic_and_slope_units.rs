//! The retired homeopathic potencies (`[hp'_X]`, `[hp'_C]`, `[hp'_M]`, `[hp'_Q]`, UCUM §44)
//! are levels of a dilution: `n X` is a dilution of 10^-n, `n C` of 100^-n, `n M` of
//! 1000^-n and `n Q` of 50000^-n, so they convert linearly between each other and to `B`.
//! The percent of slope (`%[slope]`) is `100tan(1 rad)` like the prism diopter, so the two
//! convert 1:1 and to nothing else.

use octofhir_ucum::{
    AdvancedConversionContext, ErrorKind, analyse, convert_with_context, get_canonical_units,
    multiply, unit_multiply,
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
fn potencies_of_the_series_convert_by_their_basis() {
    // These used to be 1: a dilution of 100^-1 is 10^-2
    assert_close(convert(1.0, "[hp'_C]", "[hp'_X]"), 2.0, "1 C");
    assert_close(convert(1.0, "[hp'_M]", "[hp'_X]"), 3.0, "1 M");
    assert_close(convert(1.0, "[hp'_Q]", "[hp'_X]"), 50000f64.log10(), "1 Q");
    assert_close(convert(6.0, "[hp'_X]", "[hp'_C]"), 3.0, "6 X");
    assert_close(convert(2.0, "[hp'_M]", "[hp'_C]"), 3.0, "2 M");
    // A dilution of 10^-1 is a level of -1 B
    assert_close(convert(1.0, "[hp'_X]", "B"), -1.0, "1 X");
    assert_close(convert(-20.0, "dB", "[hp'_X]"), 2.0, "-20 dB");
}

#[test]
fn a_potency_is_a_dilution_after_a_number() {
    // `3.[hp'_C]` is the dilution 100^-3, like `20.dB` is the ratio 10^2; it used to be 3
    assert_close(
        analyse("3.[hp'_C]").unwrap().factor * 1e6,
        1.0,
        "3.[hp'_C] in 1e-6",
    );
    assert_close(
        analyse("[hp'_X]").unwrap().factor,
        -1.0,
        "size of X in bels",
    );
}

#[test]
fn other_levels_stay_exact_powers_of_their_base() {
    // Only a decimal level goes through its size in bels: 2^3 through 10^(3 lg 2) would be
    // 7.999999999999999
    assert_eq!(analyse("3.bit_s").unwrap().factor, 8.0);
    assert_eq!(analyse("10.bit_s").unwrap().factor, 1024.0);
    assert_eq!(analyse("2.B").unwrap().factor, 100.0);
    assert_eq!(analyse("1.Np").unwrap().factor, core::f64::consts::E);
}

#[test]
fn a_potency_does_not_convert_to_a_proper_or_arbitrary_unit() {
    // These used to convert 1:1, or 1:100 for %
    assert_no_conversion("[hp'_X]", "1");
    assert_no_conversion("[hp'_C]", "%");
    assert_no_conversion("1", "[hp'_M]");
    // The current Hahnemann potencies are arbitrary units, not dilutions
    assert_no_conversion("[hp'_X]", "[hp_X]");
    assert_no_conversion("[hp_C]", "[hp'_C]");
}

#[test]
fn slope_and_prism_diopter_are_the_same_scale() {
    // Both are 100 times the tangent of an angle (UCUM §44)
    assert_eq!(convert(1.0, "%[slope]", "[p'diop]"), 1.0);
    assert_eq!(convert(12.5, "[p'diop]", "%[slope]"), 12.5);
    assert_eq!(convert(10.0, "%[slope]", "%[slope]"), 10.0);
}

#[test]
fn slope_does_not_convert_to_a_linear_unit() {
    // These used to convert 1:1, as if a percent of slope were a percent
    assert_no_conversion("%[slope]", "%");
    assert_no_conversion("%", "%[slope]");
    assert_no_conversion("%[slope]", "1");
    assert_no_conversion("%[slope]", "rad");
    assert_no_conversion("%[slope]", "deg");
    assert_no_conversion("%[slope]", "B");
}

#[test]
fn potencies_and_slope_take_no_part_in_unit_arithmetic() {
    // UCUM §22.1, like `B` and `[p'diop]`; these used to succeed
    for code in ["[hp'_X]", "[hp'_Q]", "%[slope]"] {
        assert!(unit_multiply(code, "m").is_err(), "{code}");
        assert!(multiply(1.0, code, 2.0, "m").is_err(), "{code}");
        let err = get_canonical_units(code).expect_err(code);
        assert!(
            matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
            "{code}: {:?}",
            err.kind
        );
    }
}
