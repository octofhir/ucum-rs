//! `10*` and `10^` are unit atoms of the specification ("the number ten for arbitrary
//! powers"). Without an exponent they are the number ten, i.e. the same as `10*1`.

#![allow(clippy::result_large_err)]

use octofhir_ucum::precision::NumericOps;
use octofhir_ucum::{OwnedUnitExpr, evaluate_owned, parse_expression, validate, validate_ucum};

fn factor(expr: &str) -> f64 {
    let ast = parse_expression(expr).expect("parse ok");
    evaluate_owned(&ast).expect("eval ok").factor.to_f64()
}

#[test]
fn ten_power_without_exponent_is_ten() {
    for code in ["10*", "10^"] {
        assert!(validate(code).is_ok(), "validate({code:?})");
        assert_eq!(factor(code), 10.0, "{code}");
        // Same expression as the explicit exponent 1
        assert_eq!(
            parse_expression(code).unwrap(),
            parse_expression(&format!("{code}1")).unwrap()
        );
    }
}

#[test]
fn ten_power_atom_combines_with_other_units() {
    assert_eq!(factor("10*.m"), 10.0);
    assert_eq!(factor("m/10*"), 0.1);
    assert_eq!(factor("(10*)"), 10.0);
    assert_eq!(factor("10*{a}"), 10.0);
    assert_eq!(factor("10*/10^"), 1.0);
}

#[test]
fn ten_power_atom_behaves_like_the_number_ten() {
    // Next to a logarithmic unit the number is the argument: all three are 10 B
    assert_eq!(factor("10*.B"), factor("10.B"));
    assert_eq!(factor("10*.B"), factor("10*1.B"));
    assert_eq!(factor("10*.Np"), factor("10.Np"));
}

#[test]
fn display_parses_back_to_the_same_expression() {
    for input in ["10*", "10^", "10*.m", "m/10*", "10*.B", "(10*)^2"] {
        let ast = parse_expression(input).unwrap();
        let reparsed = parse_expression(&ast.to_string()).unwrap();
        assert_eq!(reparsed, ast, "{input} displays as {ast}");
    }
}

#[test]
fn ten_power_with_exponent_is_unchanged() {
    assert_eq!(
        parse_expression("10*3").unwrap(),
        OwnedUnitExpr::Numeric(1000.0)
    );
    assert_eq!(factor("10^3"), 1000.0);
    assert_eq!(factor("10*+3"), 1000.0);
    assert_eq!(factor("10*-2.m"), factor("cm"));
}

#[test]
fn sign_without_digits_is_still_rejected() {
    for input in ["10*-", "10^-", "10*+", "10*-.m"] {
        assert!(validate(input).is_err(), "validate({input:?})");
    }
}

#[test]
fn self_check_accepts_the_ten_power_atoms() {
    let issues = validate_ucum();
    assert!(
        !issues.iter().any(|issue| issue.starts_with("Unit 10")),
        "{issues:?}"
    );
}
