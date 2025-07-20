use octofhir_ucum_core::{UnitExpr, UnitFactor, parse_expression};

fn sym<S: Into<String>>(s: S) -> UnitExpr {
    UnitExpr::Symbol(s.into())
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
        UnitExpr::Product(vec![
            UnitFactor {
                expr: sym("kg"),
                exponent: 1
            },
            UnitFactor {
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
        UnitExpr::Product(vec![
            UnitFactor {
                expr: sym("kg"),
                exponent: 1
            },
            UnitFactor {
                expr: sym("m"),
                exponent: 1
            },
        ])
    );
}

#[test]
fn quotient_and_power() {
    let expr = parse_expression("kg.m/s^2").unwrap();
    let expected_num = UnitExpr::Product(vec![
        UnitFactor {
            expr: sym("kg"),
            exponent: 1,
        },
        UnitFactor {
            expr: sym("m"),
            exponent: 1,
        },
    ]);
    let expected_den = UnitExpr::Power(Box::new(sym("s")), 2);
    assert_eq!(
        expr,
        UnitExpr::Quotient(Box::new(expected_num), Box::new(expected_den))
    );
}

#[test]
fn parentheses_and_power() {
    let expr = parse_expression("(m/s)^2").unwrap();
    let inner = UnitExpr::Quotient(Box::new(sym("m")), Box::new(sym("s")));
    assert_eq!(expr, UnitExpr::Power(Box::new(inner), 2));
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
fn fractional_decimal_literals() {
    assert_eq!(parse_expression("1.23").unwrap(), UnitExpr::Numeric(1.23));
    assert_eq!(parse_expression(".5").unwrap(), UnitExpr::Numeric(0.5));
    assert_eq!(parse_expression("4.").unwrap(), UnitExpr::Numeric(4.0));
    assert_eq!(parse_expression("-0.1").unwrap(), UnitExpr::Numeric(-0.1));
    assert_eq!(parse_expression("+3").unwrap(), UnitExpr::Numeric(3.0));
}

#[test]
fn numeric_multiplier() {
    let expr = parse_expression("10*3.m").unwrap();
    let expected = UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(1000.0),
            exponent: 1,
        },
        UnitFactor {
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
    let inner = UnitExpr::Quotient(
        Box::new(sym("m")),
        Box::new(UnitExpr::Power(Box::new(sym("s")), 2)),
    );
    let expected = UnitExpr::Product(vec![
        UnitFactor {
            expr: sym("kg"),
            exponent: 1,
        },
        UnitFactor {
            expr: inner,
            exponent: 1,
        },
    ]);
    assert_eq!(expr, expected);
}

#[test]
fn leading_numeric_symbol() {
    let expr = parse_expression("2.5kPa").unwrap();
    let expected = UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(2.5),
            exponent: 1,
        },
        UnitFactor {
            expr: sym("kPa"),
            exponent: 1,
        },
    ]);
    assert_eq!(expr, expected);
}

#[test]
fn caret_numeric_multiplier() {
    // simple numeric only
    assert_eq!(parse_expression("10^3").unwrap(), UnitExpr::Numeric(1e3));
    assert_eq!(parse_expression("10^-2").unwrap(), UnitExpr::Numeric(1e-2));
    // numeric with unit symbol
    let expr = parse_expression("10^3.m").unwrap();
    let expected = UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(1e3),
            exponent: 1,
        },
        UnitFactor {
            expr: sym("m"),
            exponent: 1,
        },
    ]);
    assert_eq!(expr, expected);
}

#[test]
fn complex_expression_test() {
    let expr = parse_expression("4.[pi].10*-7.N/A2").unwrap();
    let numerator = UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(4.0),
            exponent: 1,
        },
        UnitFactor {
            expr: sym("[pi]"),
            exponent: 1,
        },
        UnitFactor {
            expr: UnitExpr::Numeric(1e-7),
            exponent: 1,
        },
        UnitFactor {
            expr: sym("N"),
            exponent: 1,
        },
    ]);
    let denominator = UnitExpr::Power(Box::new(sym("A")), 2);
    let expected = UnitExpr::Quotient(Box::new(numerator), Box::new(denominator));
    assert_eq!(expr, expected);
}
