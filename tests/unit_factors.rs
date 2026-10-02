//! Units defined through another unit ("%" = 1 10*-2, "mo" = 1 mo_j = 1 a_j/12, ...)
//! must apply that unit's factor once.

use octofhir_ucum::precision::to_f64;
use octofhir_ucum::{evaluate_owned, parse_expression};

fn convert(value: f64, from: &str, to: &str) -> f64 {
    let a = evaluate_owned(&parse_expression(from).unwrap()).unwrap();
    let b = evaluate_owned(&parse_expression(to).unwrap()).unwrap();
    assert_eq!(a.dim, b.dim, "{from} -> {to}");
    value * to_f64(a.factor) / to_f64(b.factor)
}

#[test]
fn referenced_factors_apply_once() {
    for (value, from, to, expected) in [
        (50.0, "%", "1", 0.5),
        (1.0, "[ppth]", "1", 1e-3),
        (1.0, "[ppm]", "1", 1e-6),
        (1.0, "[ppb]", "1", 1e-9),
        (180.0, "deg", "rad", core::f64::consts::PI),
        (60.0, "'", "deg", 1.0),
        (1.0, "a", "mo", 12.0),
        (1.0, "mo", "d", 30.4375),
        (1.0, "[ft_us]", "m", 1200.0 / 3937.0),
        (1.0, "[mi_us]", "[ft_us]", 5280.0),
        (1.0, "[oz_av]", "g", 28.349523125),
        (1.0, "[pt_br]", "L", 0.56826125),
    ] {
        let got = convert(value, from, to);
        assert!(
            (got - expected).abs() <= 1e-9 * expected.abs(),
            "{value} {from} -> {to}: expected {expected}, got {got}"
        );
    }
}

#[test]
fn definitions_resolve_to_base_units() {
    for (value, from, to, expected) in [
        (1.0, "L", "m3", 1e-3),
        (1.0, "[gal_us]", "L", 3.785411784),
        (1.0, "[qt_us]", "L", 0.946352946),
        (1.0, "[cup_us]", "mL", 236.5882365),
        (1.0, "[foz_us]", "mL", 29.5735295625),
        (1.0, "[psi]", "Pa", 6894.757293168361),
        (1.0, "[den]", "g/km", 1.0 / 9.0),
        (1.0, "P", "Pa.s", 0.1),
        (1.0, "eV", "J", 1.602176634e-19),
        (1.0, "[k]", "J/K", 1.380649e-23),
        (1.0, "kat", "mol/s", 1.0),
        (1.0, "b", "m2", 1e-28),
    ] {
        let got = convert(value, from, to);
        assert!(
            (got - expected).abs() <= 1e-9 * expected.abs(),
            "{value} {from} -> {to}: expected {expected}, got {got}"
        );
    }
}

#[test]
fn out_of_decimal_range_is_an_error() {
    // Factors are `Decimal`s: values beyond about 1e-28..7.9e28 are reported, not
    // silently turned into 0
    for expr in ["[h]", "10*30", "10*-30", "nm4"] {
        let result = evaluate_owned(&parse_expression(expr).unwrap());
        assert!(result.is_err(), "{expr}: {result:?}");
    }
}

#[test]
fn display_names_are_complete() {
    use octofhir_ucum::find_unit;
    // Character references used to cut the name short ("amp"), and units with
    // several names used to get the last one
    assert_eq!(find_unit("A").unwrap().display_name, "ampère");
    assert_eq!(find_unit("Ao").unwrap().display_name, "Ångström");
    assert_eq!(find_unit("[Ch]").unwrap().display_name, "Charrière");
    assert_eq!(find_unit("cd").unwrap().dim.0, [0, 0, 0, 0, 0, 0, 1]);
}
