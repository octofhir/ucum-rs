use octofhir_ucum_core::{OwnedUnitExpr, OwnedUnitFactor, parse_expression};

fn sym<S: Into<String>>(s: S) -> OwnedUnitExpr {
    OwnedUnitExpr::Symbol(s.into())
}

#[test]
fn simple_symbol() {
    let expr = parse_expression("m").unwrap();
    assert_eq!(expr, sym("m"));
}

#[test]
fn product_dot() {
    let expr = parse_expression("kg.m").unwrap();
    assert_eq!(
        expr,
        OwnedUnitExpr::Product(vec![
            OwnedUnitFactor {
                expr: sym("kg"),
                exponent: 1
            },
            OwnedUnitFactor {
                expr: sym("m"),
                exponent: 1
            },
        ])
    );
}

#[test]
fn product_whitespace() {
    let expr = parse_expression("kg m").unwrap();
    assert_eq!(
        expr,
        OwnedUnitExpr::Product(vec![
            OwnedUnitFactor {
                expr: sym("kg"),
                exponent: 1
            },
            OwnedUnitFactor {
                expr: sym("m"),
                exponent: 1
            },
        ])
    );
}

#[test]
fn quotient_and_power() {
    let expr = parse_expression("kg.m/s^2").unwrap();
    let expected_num = OwnedUnitExpr::Product(vec![
        OwnedUnitFactor {
            expr: sym("kg"),
            exponent: 1,
        },
        OwnedUnitFactor {
            expr: sym("m"),
            exponent: 1,
        },
    ]);
    let expected_den = OwnedUnitExpr::Power(Box::new(sym("s")), 2);
    assert_eq!(
        expr,
        OwnedUnitExpr::Quotient(Box::new(expected_num), Box::new(expected_den))
    );
}

#[test]
fn parentheses_and_power() {
    let expr = parse_expression("(m/s)^2").unwrap();
    let inner = OwnedUnitExpr::Quotient(Box::new(sym("m")), Box::new(sym("s")));
    assert_eq!(expr, OwnedUnitExpr::Power(Box::new(inner), 2));
}

#[test]
fn annotation_ignored() {
    let expr = parse_expression("m{steel}").unwrap();
    assert_eq!(expr, sym("m"));
}

#[test]
fn arbitrary_unit_brackets() {
    let expr = parse_expression("[arbU]").unwrap();
    assert_eq!(expr, sym("[arbU]"));
}

#[test]
fn semicolon_annotation() {
    let expr = parse_expression("m{len;NIST}").unwrap();
    assert_eq!(expr, sym("m"));
}

#[test]
fn numeric_multiplier() {
    let expr = parse_expression("10*3.m").unwrap();
    let expected = OwnedUnitExpr::Product(vec![
        OwnedUnitFactor {
            expr: OwnedUnitExpr::Numeric(1000.0),
            exponent: 1,
        },
        OwnedUnitFactor {
            expr: sym("m"),
            exponent: 1,
        },
    ]);
    assert_eq!(expr, expected);
}

#[test]
fn implicit_mult_parentheses() {
    let expr = parse_expression("kg(m/s^2)").unwrap();
    // Expect kg * (m/s^2)
    let inner = OwnedUnitExpr::Quotient(
        Box::new(sym("m")),
        Box::new(OwnedUnitExpr::Power(Box::new(sym("s")), 2)),
    );
    let expected = OwnedUnitExpr::Product(vec![
        OwnedUnitFactor {
            expr: sym("kg"),
            exponent: 1,
        },
        OwnedUnitFactor {
            expr: inner,
            exponent: 1,
        },
    ]);
    assert_eq!(expr, expected);
}

#[test]
fn leading_numeric_symbol() {
    let expr = parse_expression("2.5kPa").unwrap();
    let expected = OwnedUnitExpr::Product(vec![
        OwnedUnitFactor {
            expr: OwnedUnitExpr::Numeric(2.5),
            exponent: 1,
        },
        OwnedUnitFactor {
            expr: sym("kPa"),
            exponent: 1,
        },
    ]);
    assert_eq!(expr, expected);
}

#[test]
fn caret_numeric_multiplier() {
    // simple numeric only
    assert_eq!(parse_expression("10^3").unwrap(), OwnedUnitExpr::Numeric(1e3));
    assert_eq!(parse_expression("10^-2").unwrap(), OwnedUnitExpr::Numeric(1e-2));
    // numeric with unit symbol
    let expr = parse_expression("10^3.m").unwrap();
    let expected = OwnedUnitExpr::Product(vec![
        OwnedUnitFactor {
            expr: OwnedUnitExpr::Numeric(1e3),
            exponent: 1,
        },
        OwnedUnitFactor {
            expr: sym("m"),
            exponent: 1,
        },
    ]);
    assert_eq!(expr, expected);
}

#[test]
fn complex_expression_test() {
    let expr = parse_expression("4.[pi].10*-7.N/A2").unwrap();
    let numerator = OwnedUnitExpr::Product(vec![
        OwnedUnitFactor {
            expr: OwnedUnitExpr::Numeric(4.0),
            exponent: 1,
        },
        OwnedUnitFactor {
            expr: sym("[pi]"),
            exponent: 1,
        },
        OwnedUnitFactor {
            expr: OwnedUnitExpr::Numeric(1e-7),
            exponent: 1,
        },
        OwnedUnitFactor {
            expr: sym("N"),
            exponent: 1,
        },
    ]);
    let denominator = OwnedUnitExpr::Power(Box::new(sym("A")), 2);
    let expected = OwnedUnitExpr::Quotient(Box::new(numerator), Box::new(denominator));
    assert_eq!(expr, expected);
}
