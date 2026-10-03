//! The expression returned by `unit_multiply` and `unit_divide` must be valid UCUM and
//! denote the product or quotient of the two operands.
//!
//! Special units (temperature, logarithmic) are rejected: UCUM §22.1 does not let them
//! take part in algebraic operations.

use octofhir_ucum::{
    Dimension, ErrorKind, UnitArithmeticResult, analyse, unit_divide, unit_multiply,
};

/// Operand pairs: simple units, products, quotients, a leading solidus, parentheses,
/// annotations and numbers.
const OPERANDS: &[(&str, &str)] = &[
    ("m", "s"),
    ("m", "s.kg"),
    ("m", "s/kg"),
    ("m", "/s"),
    ("m", "/s.kg"),
    ("/s", "/s"),
    ("m/s", "kg"),
    ("m/s", "kg/A"),
    ("kg.m", "s2"),
    ("m", "s-1"),
    ("m", "(s.kg)"),
    ("1", "s"),
    ("1", "/s"),
    ("1", "s.kg"),
    ("m", "1"),
    ("mL", "{a}"),
    ("mL{total}", "h"),
    ("10*3{RBC}", "uL"),
    ("1{cells}", "uL"),
    ("µg", "mL"),
    (" m ", " /s "),
    ("", "s"),
    ("m", ""),
];

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * a.abs().max(b.abs())
}

/// Check `result` against values computed independently from the two operands, and check
/// that its expression evaluates to the same unit.
fn assert_result(result: &UnitArithmeticResult, factor: f64, dimension: [i8; 7], what: &str) {
    let expression = &result.expression;
    assert_eq!(result.dimension, Dimension(dimension), "{what}");
    assert!(
        close(result.factor, factor),
        "{what}: factor {}",
        result.factor
    );

    let parsed = analyse(expression)
        .unwrap_or_else(|e| panic!("{what}: `{expression}` does not evaluate: {e}"));
    assert_eq!(
        parsed.dimension,
        Dimension(dimension),
        "{what}: `{expression}`"
    );
    assert!(
        close(parsed.factor, factor),
        "{what}: `{expression}` has factor {}, expected {factor}",
        parsed.factor
    );
}

#[test]
fn product_expression_denotes_the_product() {
    for (a, b) in OPERANDS {
        let (left, right) = (analyse(a).unwrap(), analyse(b).unwrap());
        let mut dimension = left.dimension.0;
        for (exp, other) in dimension.iter_mut().zip(right.dimension.0) {
            *exp += other;
        }
        let result = unit_multiply(a, b).unwrap();
        assert_result(
            &result,
            left.factor * right.factor,
            dimension,
            &format!("unit_multiply({a:?}, {b:?})"),
        );
    }
}

#[test]
fn quotient_expression_denotes_the_quotient() {
    for (a, b) in OPERANDS {
        let (left, right) = (analyse(a).unwrap(), analyse(b).unwrap());
        let mut dimension = left.dimension.0;
        for (exp, other) in dimension.iter_mut().zip(right.dimension.0) {
            *exp -= other;
        }
        let result = unit_divide(a, b).unwrap();
        assert_result(
            &result,
            left.factor / right.factor,
            dimension,
            &format!("unit_divide({a:?}, {b:?})"),
        );
    }
}

#[test]
fn compound_divisor_is_parenthesized() {
    // `m/s.kg` would be `(m/s).kg` (UCUM §7.4)
    assert_eq!(unit_divide("m", "s.kg").unwrap().expression, "m/(s.kg)");
    assert_eq!(unit_divide("m", "s/kg").unwrap().expression, "m/(s/kg)");
    assert_eq!(unit_divide("1", "s.kg").unwrap().expression, "/(s.kg)");
    // Not twice
    assert_eq!(unit_divide("m", "(s.kg)").unwrap().expression, "m/(s.kg)");
}

#[test]
fn solidus_always_has_a_left_operand_inside_a_term() {
    // The grammar allows a leading solidus at the start of the whole term only
    // (`mainTerm : '/' term | term`)
    assert_eq!(unit_multiply("m", "/s").unwrap().expression, "m/s");
    assert_eq!(unit_multiply("m", "/s.kg").unwrap().expression, "m/s.kg");
    assert_eq!(unit_divide("m", "/s").unwrap().expression, "m/(1/s)");
    assert_eq!(unit_divide("1", "/s").unwrap().expression, "/(1/s)");
    assert_eq!(unit_multiply("1", "/s").unwrap().expression, "/s");

    for (a, b) in OPERANDS {
        for expression in [
            unit_multiply(a, b).unwrap().expression,
            unit_divide(a, b).unwrap().expression,
        ] {
            let inner = expression.strip_prefix('/').unwrap_or(&expression);
            assert!(
                !inner.contains("(/") && !inner.contains("./") && !inner.contains("//"),
                "{a:?}, {b:?}: `{expression}` has a solidus without a left operand"
            );
        }
    }
}

#[test]
fn simple_results_are_unchanged() {
    assert_eq!(unit_multiply("m", "s").unwrap().expression, "m.s");
    assert_eq!(unit_divide("m", "s").unwrap().expression, "m/s");
    assert_eq!(unit_multiply("kg", "m/s2").unwrap().expression, "kg.m/s2");
    assert_eq!(unit_divide("kg.m", "s2").unwrap().expression, "kg.m/s2");
    assert_eq!(unit_multiply("1", "s").unwrap().expression, "s");
    assert_eq!(unit_divide("m", "1").unwrap().expression, "m");
    assert_eq!(unit_divide("1", "s").unwrap().expression, "/s");
    // Operands keep their spelling, annotations included
    assert_eq!(
        unit_divide("mL{total}", "h").unwrap().expression,
        "mL{total}/h"
    );
    assert_eq!(
        unit_divide("1{cells}", "uL").unwrap().expression,
        "1{cells}/uL"
    );
    assert_eq!(
        unit_divide("10*3{RBC}", "uL").unwrap().expression,
        "10*3{RBC}/uL"
    );
}

#[test]
fn small_factors_are_not_rounded_further() {
    // The factor is the ratio of the two operand factors: no extra `Decimal` round trip,
    // which would turn 1.6605e-27 into 1.7e-27
    let result = unit_divide("u", "kg").unwrap();
    let expected = analyse("u").unwrap().factor / analyse("kg").unwrap().factor;
    assert_eq!(result.factor, expected);
    assert!(close(result.factor, 1.6605e-27), "{}", result.factor);
}

#[test]
fn whitespace_and_empty_operands_give_a_valid_expression() {
    assert_eq!(unit_multiply("m", " /s").unwrap().expression, "m/s");
    assert_eq!(unit_multiply(" m ", " s ").unwrap().expression, "m.s");
    // The empty string is the unity, like for the parser
    assert_eq!(unit_multiply("m", "").unwrap().expression, "m");
    assert_eq!(unit_multiply("", "s").unwrap().expression, "s");
    assert_eq!(unit_divide("m", "").unwrap().expression, "m");
    assert_eq!(unit_divide("", "s").unwrap().expression, "/s");
    assert_eq!(unit_multiply("", "").unwrap().expression, "1");
}

#[test]
fn special_units_are_rejected() {
    // UCUM §22.1: special units cannot take part in algebraic operations. Pasting them
    // would also change their value: `2.B` is 100, but `2.B.m` is `2 x B x m`.
    for (a, b) in [
        ("2.B", "m"),
        ("m", "20.dB"),
        ("2", "B"),
        ("dB", "/s"),
        ("m", "mNp"),
        ("100", "[p'diop]"),
        ("Cel", "m"),
        ("m", "[degF]"),
    ] {
        for result in [unit_multiply(a, b), unit_divide(a, b)] {
            let err = result.expect_err(&format!("{a:?}, {b:?}"));
            assert!(
                matches!(err.kind, ErrorKind::ConversionError { .. }),
                "{a:?}, {b:?}: unexpected error {:?}",
                err.kind
            );
        }
    }
}
