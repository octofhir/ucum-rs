//! The expression returned by `unit_multiply` and `unit_divide` is valid UCUM and denotes
//! the product or quotient of the two operands; `multiply` and `divide_by` compute exact
//! decimal values in canonical units.
//!
//! Special units are rejected: UCUM §22.1 does not let them take part in algebraic
//! operations. Arbitrary units take part, as their own atoms (§25).

use octofhir_ucum::{
    ErrorKind, EvalResult, UnitArithmeticResult, divide_by, evaluate, multiply, parse_expression,
    unit_divide, unit_multiply,
};

/// Operand pairs: simple units, products, quotients, parentheses, annotations, numbers and
/// arbitrary units.
const OPERANDS: &[(&str, &str)] = &[
    ("m", "s"),
    ("m", "s.kg"),
    ("m", "s/kg"),
    ("m/s", "kg"),
    ("m/s", "kg/A"),
    ("kg.m", "s2"),
    ("m", "s-1"),
    ("m", "(s.kg)"),
    ("1", "s"),
    ("1", "s.kg"),
    ("m", "1"),
    ("mL", "{a}"),
    ("mL{total}", "h"),
    ("10*3{RBC}", "uL"),
    ("1{cells}", "uL"),
    ("ug", "mL"),
    ("[IU]", "mL"),
    ("k[IU]/L", "[arb'U]"),
    // Numbers of 21 digits or more
    ("1000000000000000000000", "m"),
    ("s", "1234567890123456789012345678.m"),
];

fn eval(expr: &str) -> EvalResult {
    evaluate(&parse_expression(expr).unwrap_or_else(|e| panic!("{expr}: {e}")))
        .unwrap_or_else(|e| panic!("{expr}: {e}"))
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-12 * a.abs().max(b.abs())
}

/// `result` and its expression have the factor, dimension and arbitrary atoms of
/// `a ^ 1 × b ^ exp`.
fn assert_result(result: &UnitArithmeticResult, a: &str, b: &str, exp: i32) {
    let what = format!("{a:?} {} {b:?}", if exp == 1 { '*' } else { '/' });
    let (left, right) = (eval(a), eval(b));
    let expected = if exp == 1 {
        left.factor.checked_mul(right.factor)
    } else {
        left.factor.checked_div(right.factor)
    }
    .unwrap();
    let mut dim = left.dim.0;
    for (d, r) in dim.iter_mut().zip(right.dim.0) {
        *d += r * exp as i8;
    }
    assert_eq!(result.dimension.0, dim, "{what}");
    assert!(
        close(result.factor, expected.to_f64()),
        "{what}: {}",
        result.factor
    );

    let parsed = eval(&result.expression);
    assert_eq!(parsed.dim.0, dim, "{what}: `{}`", result.expression);
    assert_eq!(parsed.factor, expected, "{what}: `{}`", result.expression);
    let mut atoms = left.arbitrary.clone();
    for (atom, e) in &right.arbitrary {
        atoms.push((atom, e * exp));
    }
    atoms.sort();
    atoms.retain(|(_, e)| *e != 0);
    assert_eq!(parsed.arbitrary, atoms, "{what}: `{}`", result.expression);
    assert_eq!(
        result.is_dimensionless,
        dim == [0; 7] && atoms.is_empty(),
        "{what}"
    );
}

#[test]
fn product_expression_denotes_the_product() {
    for (a, b) in OPERANDS {
        assert_result(&unit_multiply(a, b).unwrap(), a, b, 1);
    }
}

#[test]
fn quotient_expression_denotes_the_quotient() {
    for (a, b) in OPERANDS {
        assert_result(&unit_divide(a, b).unwrap(), a, b, -1);
    }
}

#[test]
fn leading_solidus_operands_give_valid_products() {
    for (a, b) in [("m", "/s"), ("/s", "/s"), ("1", "/s")] {
        assert_result(&unit_multiply(a, b).unwrap(), a, b, 1);
    }
}

#[test]
fn product_with_a_compound_leading_solidus_operand_is_valid() {
    assert_result(&unit_multiply("m", "/s.kg").unwrap(), "m", "/s.kg", 1);
}

#[test]
fn leading_solidus_divisors_give_valid_quotients() {
    for (a, b) in [("m", "/s"), ("m", "/s.kg"), ("/s", "/s"), ("1", "/s")] {
        assert_result(&unit_divide(a, b).unwrap(), a, b, -1);
    }
}

#[test]
fn expressions_read_as_written() {
    assert_eq!(unit_multiply("m", "s").unwrap().expression, "m.s");
    assert_eq!(unit_divide("m", "s").unwrap().expression, "m/s");
    assert_eq!(unit_divide("kg.m", "s2").unwrap().expression, "kg.m/s2");
    // Terms are read left to right (UCUM §7.4): a product is the operands one after the
    // other, a compound divisor is grouped
    assert_eq!(unit_multiply("kg", "m/s2").unwrap().expression, "kg.m/s2");
    assert_eq!(unit_multiply("m", "/s").unwrap().expression, "m/s");
    assert_eq!(unit_divide("m", "s.kg").unwrap().expression, "m/(s.kg)");
    assert_eq!(unit_divide("m", "s/kg").unwrap().expression, "m/(s/kg)");
    assert_eq!(unit_divide("m", "(s.kg)").unwrap().expression, "m/(s.kg)");
    assert_eq!(unit_divide("1", "s.kg").unwrap().expression, "/(s.kg)");
    // The unity is left out
    assert_eq!(unit_multiply("1", "s").unwrap().expression, "s");
    assert_eq!(unit_multiply("1", "/s").unwrap().expression, "/s");
    assert_eq!(unit_divide("m", "1").unwrap().expression, "m");
    assert_eq!(unit_divide("1", "s").unwrap().expression, "/s");
    // Operands keep their annotations
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
fn factors_are_exact_ratios() {
    let result = unit_divide("u", "kg").unwrap();
    assert_eq!(
        result.factor,
        eval("u")
            .factor
            .checked_div(eval("kg").factor)
            .unwrap()
            .to_f64()
    );
    assert!(
        close(result.factor, 1.660_539_066_6e-27),
        "{}",
        result.factor
    );
    assert_eq!(unit_multiply("km", "km").unwrap().factor, 1e6);
    assert_eq!(unit_divide("km", "s").unwrap().factor, 1e3);
}

#[test]
fn invalid_operands_are_rejected() {
    // The empty string and white space are not UCUM (UCUM §3.1)
    for (a, b) in [("", "s"), ("m", ""), (" m", "s"), ("m", "s "), ("µg", "mL")] {
        let err = unit_multiply(a, b).expect_err(&format!("{a:?}, {b:?}"));
        assert!(
            matches!(err.kind, ErrorKind::InvalidExpression { .. }),
            "{a:?}, {b:?}: {:?}",
            err.kind
        );
        assert!(unit_divide(a, b).is_err());
    }
}

#[test]
fn special_units_are_rejected() {
    // UCUM §22.1: special units take part in no algebraic operation
    for (a, b) in [
        ("2.B", "m"),
        ("m", "20.dB"),
        ("2", "B"),
        ("dB", "s"),
        ("m", "mNp"),
        ("100.[p'diop]", "1"),
        ("Cel", "m"),
        ("m", "[degF]"),
        ("[pH]", "L"),
        ("[m/s2/Hz^(1/2)]", "s"),
    ] {
        for result in [unit_multiply(a, b), unit_divide(a, b)] {
            let err = result.expect_err(&format!("{a:?}, {b:?}"));
            assert!(
                matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
                "{a:?}, {b:?}: unexpected error {:?}",
                err.kind
            );
        }
        let err = multiply(1.0, a, 2.0, b).unwrap_err();
        assert!(
            matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
            "{a} {b}"
        );
        let err = divide_by(1.0, a, 2.0, b).unwrap_err();
        assert!(
            matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
            "{a} {b}"
        );
    }
}

#[test]
fn quantity_arithmetic_is_exact_decimal() {
    let result = multiply(1.1, "m", 3.0, "m").unwrap();
    assert_eq!((result.value, result.unit.as_str()), (3.3, "m2"));
    let result = divide_by(0.3, "m", 0.1, "s").unwrap();
    assert_eq!((result.value, result.unit.as_str()), (3.0, "m.s-1"));
    let result = multiply(6.3, "mm", 1.0, "1").unwrap();
    assert_eq!((result.value, result.unit.as_str()), (0.0063, "m"));
    // Factors beyond the range of a Decimal: the electron mass is 9.1093837139e-28 g
    let result = divide_by(1.0, "[m_p]", 1.0, "[m_e]").unwrap();
    assert!(
        close(result.value, 1.672_621_925_95e-24 / 9.109_383_713_9e-28),
        "{}",
        result.value
    );
    assert_eq!(result.unit, "1");
}

#[test]
fn arbitrary_units_are_kept_in_the_result() {
    let result = multiply(2.0, "[IU]", 3.0, "mL").unwrap();
    assert_eq!(result.unit, "m3.[iU]");
    assert!(close(result.value, 6e-6), "{}", result.value);
    let result = divide_by(10.0, "k[IU]", 2.0, "L").unwrap();
    assert_eq!(result.unit, "m-3.[iU]");
    assert_eq!(result.value, 5e6);
    let result = divide_by(1.0, "[IU]", 1.0, "[arb'U]").unwrap();
    assert_eq!(result.unit, "[arb'U]-1.[iU]");
    // The same arbitrary atom cancels
    let result = divide_by(4.0, "[IU]", 2.0, "[iU]").unwrap();
    assert_eq!((result.value, result.unit.as_str()), (2.0, "1"));
    assert!(unit_divide("[IU]", "[IU]").unwrap().is_dimensionless);
    assert!(!unit_multiply("[IU]", "1").unwrap().is_dimensionless);
}
