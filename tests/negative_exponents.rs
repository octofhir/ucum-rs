//! UCUM §9: negative exponents are written with a leading minus sign (`s-1`, `m.s-2`).

use octofhir_ucum::precision::{NumericOps, from_f64};
use octofhir_ucum::{
    Dimension, EvalResult, OwnedUnitExpr, OwnedUnitFactor, evaluate_owned, parse_expression,
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
