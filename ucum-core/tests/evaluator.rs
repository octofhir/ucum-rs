use ucum_core::{evaluate, parse_expression};

fn eval(expr: &str) -> ucum_core::EvalResult {
    let ast = parse_expression(expr).expect("parse ok");
    evaluate(&ast).expect("eval ok")
}

#[test]
fn prefix_unit_factor() {
    let pa = eval("Pa");
    let kpa = eval("kPa");
    assert_eq!(pa.dim, kpa.dim);
    let ratio = kpa.factor / pa.factor;
    assert!((ratio - 1_000.0).abs() < 1e-6);
}

#[test]
fn square_bracket_unit_dimensionless() {
    let iu = eval("[IU]");
    assert_eq!(iu.dim, ucum_core::Dimension([0; 7]));
    assert!((iu.factor - 1.0).abs() < 1e-12);
}

#[test]
fn celsius_offset() {
    let cel = eval("Cel");
    let k = eval("K");
    assert!((cel.factor - k.factor).abs() < 1e-12);
    assert!((cel.offset - 273.15).abs() < 1e-2);
}

#[test]
fn product_and_power() {
    let m2 = eval("m2"); // implicit exponent 2
    let m_pow = eval("m^2");
    assert_eq!(m2.dim, m_pow.dim);
    assert!((m2.factor - m_pow.factor).abs() < 1e-12);
}
