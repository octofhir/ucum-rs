//! UCUM §7: '.' and '/' have the same precedence and are evaluated left to right,
//! the whole expression must be consumed, and evaluation reports overflow instead of
//! panicking.

use octofhir_ucum::precision::{NumericOps, from_f64};
use octofhir_ucum::{EvalResult, OwnedUnitExpr, evaluate_owned, parse_expression, validate};

fn eval(expr: &str) -> EvalResult {
    let ast = parse_expression(expr).expect("parse ok");
    evaluate_owned(&ast).expect("eval ok")
}

fn assert_same(a: &str, b: &str) {
    let (x, y) = (eval(a), eval(b));
    assert_eq!(x.dim, y.dim, "{a} vs {b}");
    assert!(
        (x.factor.sub(y.factor)).abs() <= from_f64(1e-12).mul(y.factor.abs()),
        "{a} vs {b}: {} != {}",
        x.factor,
        y.factor
    );
}

#[test]
fn period_and_solidus_are_left_associative() {
    // Official case 3-111a: s/m.mg is (s/m).mg
    assert_same("s/m.mg", "s.m-1.mg");
    assert_same("kg/m.s", "kg.s/m");
    assert_same("s/m/mg", "s.m-1.mg-1");
    assert_same("umol/2.h", "umol.h/2");
    assert_same("kg/(m.s)", "kg.m-1.s-1");
}

#[test]
fn leading_solidus_inverts_next_factor() {
    assert_same("/s.m", "m/s");
    assert_same("/s", "s-1");
}

#[test]
fn rejects_dangling_operators() {
    // Official case 1-102: "m/" is not followed by a term
    for expr in ["m/", "m.", "/", ".", "m./s", "m//s", "()", "(m", "m)"] {
        assert!(parse_expression(expr).is_err(), "{expr}");
    }
}

#[test]
fn rejects_trailing_input() {
    for expr in [
        "(m.s)-1", "(s)-2", "m{a}-2", "s -2", "10-3", "m/-2", "m{abc",
    ] {
        assert!(parse_expression(expr).is_err(), "{expr}");
    }
}

#[test]
fn empty_expression_is_not_a_valid_term() {
    // Official case 1-103: the unity is written "1"...
    assert!(validate("").is_err());
    assert!(validate("  ").is_err());
    assert!(validate("1").is_ok());
    // ...but an empty unit, e.g. a canonical unit, still reads as the unity (2-101, 4-103)
    assert_eq!(parse_expression("").unwrap(), OwnedUnitExpr::Numeric(1.0));
}

#[test]
fn parses_symbol_only_atoms() {
    // '%' used to be dropped by the tokenizer and evaluated as 1
    assert_eq!(eval("%").factor, from_f64(0.01));
    assert_eq!(
        parse_expression("'").unwrap(),
        OwnedUnitExpr::Symbol("'".to_string())
    );
    assert_eq!(
        parse_expression("''").unwrap(),
        OwnedUnitExpr::Symbol("''".to_string())
    );
}

#[test]
fn reports_overflow_instead_of_panicking() {
    for expr in [
        // Factor does not fit in a Decimal
        "km10",
        "Gm4",
        "m.km10",
        "/km10",
        "[pi]100",
        // Dimension exponent does not fit in i8
        "s-2147483647",
        "s2147483647",
        "m-127.m-2",
        "m100.m100",
        // Exponent does not fit in i32
        "s99999999999",
        "s-99999999999",
        // Number does not fit in a Decimal
        "10*30.m",
    ] {
        assert!(validate(expr).is_err(), "{expr}");
    }
}

#[test]
fn large_valid_exponents() {
    assert!(validate("m127").is_ok());
    assert!(validate("m-128").is_ok());
    assert!(validate("km9").is_ok());
}

#[test]
fn numbers_scale_prefixed_units() {
    // A number times a unit used to drop the prefix ("2.km" was 2 m) or misread the
    // unit ("2.Pa" as peta-years, "2.min" as unknown)
    assert_same("2.km", "2000.m");
    assert_same("2.h", "7200.s");
    assert_same("2.min", "120.s");
    assert_same("2.mol", "2000.mmol");
    assert_same("2.Pa", "2000.mPa");
    assert_same("2.k[IU]", "2000.[IU]");
}

#[test]
fn prefixes_need_a_metric_unit() {
    assert_same("dam", "10.m");
    assert!(validate("k[in_i]").is_err());
    assert!(validate("kh").is_err());
    assert!(validate("k[IU]").is_ok());
}

#[test]
fn square_brackets_hold_any_character() {
    // UCUM §5.2: "B[10.nV]" is one atom, '.' included
    assert!(validate("dB[10.nV]").is_ok());
    // ...and '+' inside brackets is not addition, just an unknown unit here
    let err = validate("[abc+ef]").unwrap_err();
    assert!(matches!(
        err.kind,
        octofhir_ucum::ErrorKind::UnitNotFound { .. }
    ));
}

#[test]
fn annotations_are_the_unity() {
    assert_same("{rbc}", "1");
    assert_same("/{tot}", "1");
    assert_same("mL/{hb}.m2", "mL.m2");
    assert_same("g.m/{hb}", "g.m");
    // Characters 33-126 only (UCUM §6.1)
    assert!(validate("rad2{錠}").is_err());
    assert!(validate("{a b}").is_err());
    // An annotation does not start a symbol
    assert!(validate("{a}rad2{b}").is_err());
    assert!(validate("{a}.rad2{b}").is_ok());
}

#[test]
fn display_parses_back() {
    for expr in [
        "kg/(m.s)",
        "s/m.mg",
        "kg.m.s-2",
        "/s",
        "4.[pi].10*-7.N/A2",
        "m+2",
        "(m/s)^2",
        "10*-3.L",
        "mL/{hb}.m2",
    ] {
        let printed = parse_expression(expr).unwrap().to_string();
        assert_same(&printed, expr);
    }
}
