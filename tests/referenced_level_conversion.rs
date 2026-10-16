//! Levels with a reference (UCUM §21-22: `B[SPL]`, `B[V]`, `B[mV]`, `B[uV]`, `B[10.nV]`,
//! `B[W]`, `B[kW]`) are special units commensurable with the unit of their reference, not
//! dimensionless ratios. Two levels of the same quantity differ by the level of one
//! reference in the other (1 V is 6 B[mV]), and a level of one quantity does not convert to
//! a level of another quantity, to a plain level (`B`) or to a proper unit.

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

#[test]
fn levels_of_the_same_quantity_shift_by_their_references() {
    // These used to be 1, 0, 60 and 20: the references were ignored
    // 1 B[V] is a voltage of 10^0.5 V, 3162 mV, which is 7 B[mV]
    assert_close(convert(1.0, "B[V]", "B[mV]"), 7.0, "1 B[V]");
    // 1 W is -3 B[kW]
    assert_close(convert(0.0, "B[W]", "B[kW]"), -3.0, "0 B[W]");
    // 60 dB re 1 uV is 1 mV
    assert_close(convert(60.0, "dB[uV]", "dB[mV]"), 0.0, "60 dB[uV]");
    // 20 dB re 10 nV is 100 nV, -20 dB re 1 uV
    assert_close(convert(20.0, "dB[10.nV]", "dB[uV]"), -20.0, "20 dB[10.nV]");
}

#[test]
fn prefixes_still_scale_the_level() {
    assert_eq!(convert(1.0, "B[SPL]", "dB[SPL]"), 10.0);
    assert_eq!(convert(30.0, "dB[W]", "B[W]"), 3.0);
}

#[test]
fn levels_of_other_quantities_do_not_convert() {
    for (from, to) in [
        // These used to convert 1:1, or 1:100 for %
        ("B[SPL]", "B[V]"),
        ("B[SPL]", "B[W]"),
        ("B[V]", "B[W]"),
        ("B[SPL]", "B"),
        ("dB[V]", "dB"),
        ("B[SPL]", "1"),
        ("B[SPL]", "%"),
        // Commensurable, but not linearly
        ("B[SPL]", "Pa"),
        ("V", "B[V]"),
        ("B[kW]", "W"),
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
fn a_level_is_commensurable_with_its_reference() {
    // UCUM §21.3; these used to be false, false and true
    assert!(is_comparable("B[V]", "mV").unwrap());
    assert!(is_comparable("dB[SPL]", "Pa").unwrap());
    assert!(!is_comparable("B[SPL]", "B[W]").unwrap());
}

#[test]
fn a_level_takes_no_part_in_unit_arithmetic() {
    // UCUM §22.1, like `B` and `Cel`; this used to return `B[V].m`
    assert!(unit_multiply("B[V]", "m").is_err());
    // `B[mV]` has a zero offset (1 mV is 1 g.m2.s-3.A-1) but is a level all the same
    assert!(unit_multiply("B[mV]", "m").is_err());
    assert!(multiply(1.0, "B[mV]", 2.0, "m").is_err());
    assert!(multiply(1.0, "m", 2.0, "B[V]").is_err());
    assert!(divide_by(1.0, "B[mV]", 2.0, "s").is_err());
}

#[test]
fn a_level_with_a_reference_is_not_part_of_a_term() {
    // A level with a reference is the logarithm of a quantity, not a unit to combine or
    // scale: these used to evaluate as a voltage, or as a ratio of 10^2 for `2.B[V]`
    for expression in [
        "B[mV].m",
        "m.B[V]",
        "B[mV]/s",
        "m/dB[SPL]",
        "B[mV]2",
        "2.B[mV]",
        "2.B[V]",
        "B[W].B[W]",
    ] {
        let err = analyse(expression).expect_err(expression);
        assert!(
            matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
            "{expression}: {:?}",
            err.kind
        );
    }
    // A plain level still goes into terms as before
    assert!(analyse("dB/s").is_ok());
    assert!(analyse("10.dB").is_ok());
}

#[test]
fn a_level_with_a_reference_always_has_an_offset() {
    // Its level in the base units, in bels, which happens to be 0 for `B[mV]`
    for code in ["B[V]", "B[mV]", "dB[mV]", "B[SPL]", "B[kW]"] {
        assert!(analyse(code).unwrap().has_offset, "{code}");
    }
    assert!(!analyse("B").unwrap().has_offset);
    let context = AdvancedConversionContext::default();
    for (from, to) in [("B[mV]", "dB[mV]"), ("B[V]", "dB[V]")] {
        let result = convert_with_context(1.0, from, to, &context).unwrap();
        assert!(result.used_special_units, "{from} to {to}");
    }
}

#[test]
fn a_level_with_a_reference_has_no_canonical_unit() {
    // No factor and offset turn a logarithm into a quantity: this used to return
    // `g.m2.s-3.A-1` with factor 1 and offset 6, so 1 B[V] was 7 instead of 3162
    for code in ["B[V]", "dB[mV]", "B[SPL]"] {
        let err = get_canonical_units(code).expect_err(code);
        assert!(
            matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
            "{code}: {:?}",
            err.kind
        );
    }
    assert!(get_canonical_units("Cel").is_ok());
}
