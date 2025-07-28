use octofhir_ucum_core::{Dimension, EvalResult, evaluate_owned, parse_expression};
use octofhir_ucum_core::precision::{NumericOps, from_f64};

fn eval(expr: &str) -> EvalResult {
    let ast = parse_expression(expr).expect("parse ok");
    evaluate_owned(&ast).expect("eval ok")
}

#[test]
fn prefix_unit_factor() {
    let pa = eval("Pa");
    let kpa = eval("kPa");
    assert_eq!(pa.dim, kpa.dim);
    let ratio = kpa.factor.div(pa.factor);
    assert!((ratio.sub(from_f64(1_000.0))).abs() < from_f64(1e-6));
}

#[test]
fn square_bracket_unit_dimensionless() {
    let iu = eval("[IU]");
    assert_eq!(iu.dim, Dimension([0; 7]));
    assert!((iu.factor.sub(from_f64(1.0))).abs() < from_f64(1e-12));
}

#[test]
fn celsius_offset() {
    let cel = eval("Cel");
    let k = eval("K");
    assert!((cel.factor.sub(k.factor)).abs() < from_f64(1e-12));
    assert!((cel.offset.sub(from_f64(273.15))).abs() < from_f64(1e-2));
}

#[test]
fn product_and_power() {
    let m2 = eval("m2"); // implicit exponent 2
    let m_pow = eval("m^2");
    assert_eq!(m2.dim, m_pow.dim);
    assert!((m2.factor.sub(m_pow.factor)).abs() < from_f64(1e-12));
}
