//! `optimize_expression` and `simplify_expression` return an expression equal to their
//! input, with the same magnitude and dimension; `canonicalize_expression` returns the
//! canonical unit, without the magnitude.

use octofhir_ucum::{
    EvalResult, canonicalize_expression, evaluate, optimize_expression, parse_expression,
    simplify_expression,
};

fn eval(expr: &str) -> EvalResult {
    evaluate(&parse_expression(expr).unwrap_or_else(|e| panic!("{expr}: {e}")))
        .unwrap_or_else(|e| panic!("{expr}: {e}"))
}

/// `rewritten` is valid UCUM and equal to `original`, to the 28 digits of a factor.
fn assert_equal(original: &str, rewritten: &str) {
    let (a, b) = (eval(original), eval(rewritten));
    let what = format!("{original} rewritten as {rewritten}");
    assert_eq!(
        (a.dim, &a.arbitrary, a.special),
        (b.dim, &b.arbitrary, b.special),
        "{what}"
    );
    let ratio = a.factor.checked_div(b.factor).unwrap().to_f64();
    assert!(
        (ratio - 1.0).abs() < 1e-26,
        "{what}: {} vs {}",
        a.factor,
        b.factor
    );
}

const EXPRESSIONS: &[&str] = &[
    "m",
    "g",
    "km",
    "kg.m/s2",
    "kg.m2/s2",
    "g.m2.s-2",
    "kg.m2/s3",
    "kg/(m.s2)",
    "/s",
    "m2/s2",
    "dyn",
    "erg",
    "cal",
    "bar",
    "kV",
    "%",
    "[in_i]",
    "[lb_av]",
    "km.s/s",
    "ms",
    "m.s/s",
    "g.m/m",
    "2.m/4",
    "4.[pi].10*-7.N/A2",
    "mg/dL",
    "[IU]/mL",
    "k[IU]/L",
    "10*3{RBC}/uL",
    "{rbc}",
    "1",
    "eV",
    "[ly]",
    "sr",
    "deg",
];

#[test]
fn optimized_expressions_are_equal_to_their_input() {
    for expr in EXPRESSIONS {
        assert_equal(expr, &optimize_expression(expr).unwrap());
    }
}

#[test]
fn optimization_uses_derived_units_with_a_prefix() {
    for (expr, optimized) in [
        ("kg.m/s2", "N"),
        ("kg.m2/s2", "J"),
        ("kg.m2/s3", "W"),
        ("kg/(m.s2)", "Pa"),
        ("/s", "Hz"),
        ("g.m2.s-2", "mJ"),
        ("g.m/s2", "mN"),
        ("kV", "kV"),
        ("C/s", "A"),
        ("dm3", "L"),
    ] {
        assert_eq!(optimize_expression(expr).unwrap(), optimized, "{expr}");
    }
}

#[test]
fn optimization_keeps_the_magnitude() {
    // These used to drop the magnitude: dyn was N, [in_i] was m, % was 1
    for (expr, optimized) in [
        ("m", "m"),
        ("g", "g"),
        ("km", "km"),
        ("g.m/s2", "mN"),
        ("m2/s2", "m2.s-2"),
        ("dyn", "10*-2.g.m.s-2"),
        ("[in_i]", "254.10*-4.m"),
        ("%", "10*-2"),
        ("bar", "10*8.g.m-1.s-2"),
        ("[IU]/mL", "10*6.m-3.[iU]"),
    ] {
        assert_eq!(optimize_expression(expr).unwrap(), optimized, "{expr}");
    }
}

#[test]
fn optimized_factor_round_trips_exactly() {
    let optimized = optimize_expression("4.[pi].10*-7.N/A2").unwrap();
    assert_eq!(eval("4.[pi].10*-7.N/A2"), eval(&optimized), "{optimized}");
}

#[test]
fn optimized_large_integer_factors_are_valid_ucum() {
    // "6.02214076e23.m-3" is not UCUM: it should be "602214076.10*15.m-3"
    for expr in ["mol", "mmol/L"] {
        let optimized = optimize_expression(expr).unwrap();
        assert_equal(expr, &optimized);
    }
}

#[test]
fn special_units_are_not_optimized() {
    for expr in ["Cel", "[degF]", "dB", "B[V]", "[pH]", "2.Cel"] {
        assert_eq!(optimize_expression(expr).unwrap(), expr, "{expr}");
    }
    assert!(optimize_expression("invalid_unit").is_err());
}

#[test]
fn simplified_expressions_are_equal_to_their_input() {
    for expr in EXPRESSIONS {
        assert_equal(expr, &simplify_expression(expr).unwrap());
    }
}

#[test]
fn simplification_combines_exponents_and_numbers() {
    for (expr, simplified) in [
        ("m.s/s", "m"),
        ("g.m/m", "g"),
        // These used to change the magnitude: km.s/s was m, ms was s
        ("km.s/s", "km"),
        ("ms", "ms"),
        ("[lb_av]", "[lb_av]"),
        ("km.km", "km2"),
        ("m2/s2", "m2.s-2"),
        ("2.m/4", "5.10*-1.m"),
        ("4.m.250", "1000.m"),
        ("m/m", "1"),
        ("mg{total}/dL", "mg.dL-1"),
        ("{rbc}", "1"),
        ("/min", "min-1"),
    ] {
        assert_eq!(simplify_expression(expr).unwrap(), simplified, "{expr}");
    }
}

#[test]
fn special_units_are_not_simplified() {
    for expr in ["Cel", "dB", "2.Cel", "[pH]"] {
        assert_eq!(simplify_expression(expr).unwrap(), expr, "{expr}");
    }
    assert!(simplify_expression("invalid_unit").is_err());
}

#[test]
fn canonical_expressions_are_the_canonical_units() {
    for (expr, canonical) in [
        ("N", "g.m.s-2"),
        ("J", "g.m2.s-2"),
        ("W", "g.m2.s-3"),
        ("Pa", "g.m-1.s-2"),
        ("Hz", "s-1"),
        ("km", "m"),
        ("mg", "g"),
        ("kg", "g"),
        ("m", "m"),
        ("1", "1"),
        ("A", "s-1.C"),
        ("V", "g.m2.s-2.C-1"),
        ("sr", "rad2"),
        ("mol", "1"),
        ("[IU]/mL", "m-3.[iU]"),
        ("Cel", "K"),
    ] {
        assert_eq!(canonicalize_expression(expr).unwrap(), canonical, "{expr}");
    }
    assert!(canonicalize_expression("invalid_unit").is_err());
    // A level has no linear canonical unit
    assert!(canonicalize_expression("B").is_err());
}
