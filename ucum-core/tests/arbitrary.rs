use octofhir_ucum_core::{evaluate, parse_expression, Dimension, EvalResult, UcumError};

fn eval(expr: &str) -> Result<EvalResult, UcumError> {
    let ast = parse_expression(expr).expect("parse ok");
    println!("AST for {}: {:?}", expr, ast);
    let result = evaluate(&ast);
    println!("Result for {}: {:?}", expr, result);
    result
}

#[test]
fn arbitrary_unit_dimensionless() {
    // Test that arbitrary units are dimensionless
    let iu = eval("[IU]").unwrap();
    assert_eq!(iu.dim, Dimension([0; 7]));
    assert!((iu.factor - 1.0).abs() < 1e-12);
}

#[test]
fn arbitrary_unit_custom() {
    // Test that custom arbitrary units work
    let custom = eval("[custom'U]").unwrap();
    assert_eq!(custom.dim, Dimension([0; 7]));
    assert!((custom.factor - 1.0).abs() < 1e-12);
}

#[test]
fn arbitrary_unit_with_prefix() {
    // Test that prefixed arbitrary units work
    let kiu = eval("k[IU]").unwrap();
    assert_eq!(kiu.dim, Dimension([0; 7]));
    assert!((kiu.factor - 1000.0).abs() < 1e-12);
}

#[test]
fn arbitrary_unit_multiplication() {
    // First, check what dimension mL has
    let ml = eval("mL").unwrap();
    println!("mL dimension: {:?}", ml.dim);

    // Check what dimension L has
    let l = eval("L").unwrap();
    println!("L dimension: {:?}", l.dim);

    // Check what dimension m has (meter)
    let m = eval("m").unwrap();
    println!("m dimension: {:?}", m.dim);

    // Test multiplication of arbitrary units with other units
    let iu_per_ml = eval("[IU]/mL").unwrap();
    // Should have dimension of 1/volume (L^-3)
    assert_eq!(iu_per_ml.dim, Dimension([0, -3, 0, 0, 0, 0, 0]));
    // 1 / (0.001 L) = 1000
    assert!((iu_per_ml.factor - 1000.0).abs() < 1e-12);
}

#[test]
fn arbitrary_unit_conversion() {
    // Test conversion between the same arbitrary units
    let iu1 = eval("[IU]").unwrap();
    let iu2 = eval("[IU]").unwrap();

    // Same arbitrary units should be commensurable
    assert_eq!(iu1.dim, iu2.dim);

    // Different arbitrary units should not be commensurable
    let iu = eval("[IU]").unwrap();
    let arbu = eval("[arb'U]").unwrap();

    // Both are dimensionless but should be treated as different units
    assert_eq!(iu.dim, Dimension([0; 7]));
    assert_eq!(arbu.dim, Dimension([0; 7]));

    // Conversion factor should be 1.0 for the same unit
    assert!((iu1.factor / iu2.factor - 1.0).abs() < 1e-12);
}

#[test]
fn arbitrary_unit_with_numeric() {
    // Test numeric value with arbitrary unit
    let expr = parse_expression("5[IU]").unwrap();
    let result = evaluate(&expr).unwrap();

    // Should have factor of 5.0 and dimensionless
    assert!((result.factor - 5.0).abs() < 1e-12);
    assert_eq!(result.dim, Dimension([0; 7]));
}

#[test]
fn arbitrary_unit_in_complex_expression() {
    // Test arbitrary unit in a complex expression
    let expr = parse_expression("10[IU]/(m2.s)").unwrap();
    let result = evaluate(&expr).unwrap();

    // Should have dimension of 1/(L^2*T)
    assert_eq!(result.dim, Dimension([0, -2, -1, 0, 0, 0, 0]));

    // Factor should be 10.0 / 1.0
    assert!((result.factor - 10.0).abs() < 1e-12);
}
