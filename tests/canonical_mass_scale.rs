//! The UCUM base unit of mass is the gram (§28), and the registry is gram-based. Canonical
//! unit strings must name the gram too, so the numbers returned next to them are on the
//! scale of the string.

use octofhir_ucum::{analyse, divide_by, get_canonical_units, multiply};

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * a.abs().max(b.abs())
}

#[test]
fn canonical_unit_of_mass_is_the_gram() {
    for (unit, canonical, factor) in [
        ("g", "g", 1.0),
        ("kg", "g", 1e3),
        ("mg", "g", 1e-3),
        ("t", "g", 1e6),
        ("N", "g.m.s-2", 1e3),
        ("J", "g.m2.s-2", 1e3),
        ("Pa", "g.m-1.s-2", 1e3),
        ("/kg", "g-1", 1e-3),
        ("g2", "g2", 1.0),
    ] {
        let result = get_canonical_units(unit).unwrap();
        assert_eq!(result.unit, canonical, "{unit}");
        assert!(
            close(result.factor, factor),
            "1 {unit} is {} {canonical}, expected {factor}",
            result.factor
        );
    }
}

#[test]
fn canonical_factor_is_the_analysis_factor() {
    for unit in ["kg", "g", "N", "kPa", "[lb_av]", "km", "m/s", "1"] {
        assert_eq!(
            get_canonical_units(unit).unwrap().factor,
            analyse(unit).unwrap().factor,
            "{unit}"
        );
    }
}

#[test]
fn units_without_mass_are_unchanged() {
    for (unit, canonical, factor) in [
        ("km", "m", 1e3),
        ("m/s", "m.s-1", 1.0),
        ("h", "s", 3600.0),
        ("1", "1", 1.0),
    ] {
        let result = get_canonical_units(unit).unwrap();
        assert_eq!(result.unit, canonical, "{unit}");
        assert!(close(result.factor, factor), "{unit}: {}", result.factor);
    }
}

#[test]
fn official_multiplication_cases_through_the_public_api() {
    // UcumFunctionalTests.xml, multiplication 4-101 and 4-102
    let result = multiply(1.5, "g", 2.0, "m").unwrap();
    assert_eq!((result.value, result.unit.as_str()), (3.0, "g.m"));
    let result = multiply(2.0, "m", 1.5, "g").unwrap();
    assert_eq!((result.value, result.unit.as_str()), (3.0, "g.m"));
}

#[test]
fn official_division_cases_through_the_public_api() {
    // UcumFunctionalTests.xml, division 4-101 and 4-102
    let result = divide_by(1.5, "g", 2.0, "m").unwrap();
    assert_eq!((result.value, result.unit.as_str()), (0.75, "g.m-1"));
    let result = divide_by(2.0, "m", 1.5, "g").unwrap();
    assert_eq!(result.unit, "g-1.m");
    assert!(close(result.value, 2.0 / 1.5), "{}", result.value);
}

#[test]
fn quantity_arithmetic_returns_a_value_in_the_unit_it_names() {
    // 1 kg x 1 m is 1000 g.m
    let result = multiply(1.0, "kg", 1.0, "m").unwrap();
    assert_eq!((result.value, result.unit.as_str()), (1000.0, "g.m"));

    // 2 N x 3 m = 6 J = 6000 g.m2.s-2
    let result = multiply(2.0, "N", 3.0, "m").unwrap();
    assert_eq!(result.unit, "g.m2.s-2");
    assert!(close(result.value, 6000.0), "{}", result.value);

    // The mass cancels: 1 J / 1 kg = 1 m2.s-2
    let result = divide_by(1.0, "J", 1.0, "kg").unwrap();
    assert_eq!(result.unit, "m2.s-2");
    assert!(close(result.value, 1.0), "{}", result.value);
}

#[test]
fn user_values_are_not_rounded() {
    let third = 1.0 / 3.0;
    assert_eq!(multiply(third, "g", 1.0, "m").unwrap().value, third);
    assert_eq!(multiply(third, "m", 1.0, "s").unwrap().value, third);
}
