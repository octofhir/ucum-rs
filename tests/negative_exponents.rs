//! UCUM §9: exponents written right after a unit, with an optional sign
//! (`s-1`, `m.s-2`, `m+2`).

use octofhir_ucum::precision::{NumericOps, from_f64};
use octofhir_ucum::{
    Dimension, EvalResult, OwnedUnitExpr, OwnedUnitFactor, evaluate_owned, get_canonical_units,
    parse_expression, validate,
};

fn sym(s: &str) -> OwnedUnitExpr {
    OwnedUnitExpr::Symbol(s.to_string())
}

fn eval(expr: &str) -> EvalResult {
    let ast = parse_expression(expr).expect("parse ok");
    evaluate_owned(&ast).expect("eval ok")
}

#[test]
fn parses_single_factor() {
    assert_eq!(
        parse_expression("s-1").unwrap(),
        OwnedUnitExpr::Power(Box::new(sym("s")), -1)
    );
    assert_eq!(
        parse_expression("[in_i]-2").unwrap(),
        OwnedUnitExpr::Power(Box::new(sym("[in_i]")), -2)
    );
}

#[test]
fn parses_product() {
    assert_eq!(
        parse_expression("m3.kg-1.s-2").unwrap(),
        OwnedUnitExpr::Product(vec![
            OwnedUnitFactor {
                expr: sym("m"),
                exponent: 3
            },
            OwnedUnitFactor {
                expr: sym("kg"),
                exponent: -1
            },
            OwnedUnitFactor {
                expr: sym("s"),
                exponent: -2
            },
        ])
    );
}

#[test]
fn parses_before_annotation() {
    assert_eq!(
        parse_expression("m-2{annotation}").unwrap(),
        OwnedUnitExpr::Power(Box::new(sym("m")), -2)
    );
}

#[test]
fn does_not_affect_ten_power() {
    assert_eq!(
        parse_expression("10*-7").unwrap(),
        OwnedUnitExpr::Numeric(1e-7)
    );
}

#[test]
fn matches_equivalent_quotient() {
    for (with_exponent, with_quotient) in [
        ("s-1", "/s"),
        ("m.s-2", "m/s2"),
        ("s.m-1", "s/m"),
        ("m3.kg-1.s-2", "m3/kg/s2"),
        ("g.m.s-2.A-2", "g.m/s2/A2"),
    ] {
        let a = eval(with_exponent);
        let b = eval(with_quotient);
        assert_eq!(a.dim, b.dim, "{with_exponent} vs {with_quotient}");
        assert!(
            (a.factor.sub(b.factor)).abs() < from_f64(1e-12),
            "{with_exponent} vs {with_quotient}"
        );
    }
}

#[test]
fn applies_prefix() {
    // 1 mm-1 = 1000 m-1
    let per_mm = eval("mm-1");
    assert_eq!(per_mm.dim, Dimension([0, -1, 0, 0, 0, 0, 0]));
    assert!((per_mm.factor.sub(from_f64(1_000.0))).abs() < from_f64(1e-6));
}

#[test]
fn parses_plus_sign() {
    // UCUM §9: positive exponents may carry an optional plus sign
    assert_eq!(
        parse_expression("m+2").unwrap(),
        OwnedUnitExpr::Power(Box::new(sym("m")), 2)
    );
    assert_eq!(eval("m+2.s-1").dim, eval("m2/s").dim);
    assert!(validate("10*+3/uL").is_ok());
}

#[test]
fn plus_is_not_addition() {
    for expr in ["m+", "m+s", "10+3/ul"] {
        assert!(parse_expression(expr).is_err(), "{expr}");
    }
    // ...but annotations may contain it
    assert!(validate("mg{a+b}").is_ok());
}

#[test]
fn parses_after_micro_sign() {
    assert_eq!(
        parse_expression("µs-1").unwrap(),
        parse_expression("us-1").unwrap()
    );
    assert_eq!(
        parse_expression("µm2").unwrap(),
        OwnedUnitExpr::Power(Box::new(sym("um")), 2)
    );
}

#[test]
fn rejects_malformed() {
    for expr in ["s-", "s--1", "s-2-3", "(m.s)-1", "m{a}-2", "s -2"] {
        assert!(validate(expr).is_err(), "{expr}");
    }
}

#[test]
fn round_trips_canonical_units() {
    // Canonical units are written with negative exponents ("g.m.s-2")
    for unit in ["N", "Pa", "Hz", "J/kg"] {
        let canonical = get_canonical_units(unit).unwrap();
        assert_eq!(
            eval(&canonical.unit).dim,
            canonical.dimension,
            "{unit} -> {}",
            canonical.unit
        );
    }
}
