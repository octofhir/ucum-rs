//! Units common in FHIR Quantity resources validate and convert, and common non-UCUM
//! spellings are rejected.

use octofhir_ucum::{convert, find_unit, validate};

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-12 * a.abs().max(b.abs())
}

#[test]
fn turbidity_units_of_ucum_2_2_exist() {
    assert_eq!(
        find_unit("[NTU]").unwrap().display_name,
        "Nephelometric Turbidity Unit"
    );
    assert_eq!(
        find_unit("[FNU]").unwrap().display_name,
        "Formazin Nephelometric Unit"
    );
}

#[test]
fn fhir_units_are_valid() {
    for code in [
        "%",
        "mg/dL",
        "mmol/L",
        "meq/L",
        "U/L",
        "mm[Hg]",
        "Cel",
        "[degF]",
        "kg/m2",
        "g/dL",
        "mg",
        "mL",
        "cm",
        "mm",
        "kg",
        "[lb_av]",
        "[in_i]",
        "[ft_i]",
        "{beats}/min",
        "/min",
        "deg",
        "rad",
        "[IU]/L",
        "10*3/uL",
        "10*9/L",
        "fL",
        "pg",
        "mL/min/{1.73_m2}",
        "[pH]",
        "ng/mL",
        "umol/L",
        "[iU]/mL",
        "kcal",
        "h",
        "wk",
        "mo",
        "a",
    ] {
        assert!(validate(code).is_ok(), "{code}: {:?}", validate(code));
    }
}

#[test]
fn common_spellings_that_are_not_ucum_are_rejected() {
    // "lb" is not a unit ([lb_av]), "beats" neither ({beats}), µ is not ASCII (u), "mEq" is
    // "meq", and "degC" is "Cel"
    for code in [
        "lb",
        "beats/min",
        "µg",
        "mEq/L",
        "degC",
        "cc",
        "mcg",
        "hr",
        "sec",
    ] {
        assert!(validate(code).is_err(), "{code}");
    }
}

#[test]
fn fhir_units_convert() {
    for (value, from, to, expected) in [
        (100.0, "mg/dL", "g/L", 1.0),
        (1.0, "[lb_av]", "kg", 0.453_592_37),
        (6.0, "[ft_i]", "m", 1.8288),
        (98.6, "[degF]", "Cel", 37.0),
        // UCUM defines m[Hg] as 133.322 kPa
        (120.0, "mm[Hg]", "kPa", 15.998_64),
        (5.0, "10*9/L", "10*3/uL", 5.0),
        (1.0, "mL/min", "L/h", 0.06),
        (1.0, "kcal", "kJ", 4.184),
    ] {
        let actual = convert(value, from, to).unwrap();
        assert!(close(actual, expected), "{value} {from} to {to}: {actual}");
    }
}
