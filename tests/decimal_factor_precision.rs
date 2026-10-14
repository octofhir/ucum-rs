//! Unit factors are stored as `f64`. Turned into a `Decimal`, they must give the decimal
//! number they stand for (`0.001` for `m`), not the binary expansion of the `f64`
//! (`0.0010000000000000000208...`), so that conversions between decimal units are exact.

use octofhir_ucum::precision::{from_f64, to_f64};
use octofhir_ucum::{
    AdvancedConversionContext, convert_with_context, evaluate_owned, parse_expression,
};

fn convert(value: f64, from: &str, to: &str) -> f64 {
    convert_with_context(value, from, to, &AdvancedConversionContext::default())
        .unwrap_or_else(|err| panic!("{value} {from} to {to}: {err}"))
        .value
}

fn factor(expression: &str) -> String {
    let parsed = parse_expression(expression).unwrap();
    evaluate_owned(&parsed)
        .unwrap()
        .factor
        .normalize()
        .to_string()
}

const PREFIXES: &[(&str, i32)] = &[
    ("Y", 24),
    ("Z", 21),
    ("E", 18),
    ("P", 15),
    ("T", 12),
    ("G", 9),
    ("M", 6),
    ("k", 3),
    ("h", 2),
    ("da", 1),
    ("d", -1),
    ("c", -2),
    ("m", -3),
    ("u", -6),
    ("n", -9),
    ("p", -12),
    ("f", -15),
    ("a", -18),
    ("z", -21),
    ("y", -24),
];

#[test]
fn prefixed_factors_are_exact_decimals() {
    // These used to be 0.0010000000000000000208166817 and 0.0000000010000000000000000001
    assert_eq!(factor("mg"), "0.001");
    assert_eq!(factor("ns"), "0.000000001");
    assert_eq!(factor("YL"), "1000000000000000000000");
    assert_eq!(factor("%"), "0.01");
}

#[test]
fn every_prefix_converts_exactly() {
    for base in ["m", "g", "s", "L", "mol", "Pa", "Hz"] {
        for &(prefix, exponent) in PREFIXES {
            let prefixed = format!("{prefix}{base}");
            // Parsing gives the f64 nearest to 10^exponent (10f64.powi does not, from 10^23)
            let power = |exp: i32| format!("1e{exp}").parse::<f64>().unwrap();
            assert_eq!(convert(1.0, &prefixed, base), power(exponent), "{prefixed}");
            assert_eq!(
                convert(1.0, base, &prefixed),
                power(-exponent),
                "{prefixed}"
            );
        }
    }
}

#[test]
fn common_conversions_are_exact() {
    // These used to be 0.0010000000000000002, 0.0009999999999999998, 999999999.9999999
    // and 453.59237000000013
    assert_eq!(convert(1.0, "mg", "g"), 0.001);
    assert_eq!(convert(1.0, "uL", "mL"), 0.001);
    assert_eq!(convert(1.0, "s", "ns"), 1e9);
    assert_eq!(convert(1.0, "[lb_av]", "g"), 453.59237);
    assert_eq!(convert(1.0, "[oz_tr]", "g"), 31.1034768);
    assert_eq!(convert(250.0, "mg", "g"), 0.25);
    assert_eq!(convert(1.0, "[in_i]", "cm"), 2.54);
    assert_eq!(convert(1.0, "GiBy", "By"), 1073741824.0);
}

#[test]
fn temperatures_are_exact() {
    assert_eq!(convert(0.0, "Cel", "K"), 273.15);
    assert_eq!(convert(25.0, "Cel", "K"), 298.15);
    // 5/9 has no exact decimal: [degF] stays within an f64 rounding error
    let fahrenheit = convert(100.0, "Cel", "[degF]");
    assert!((fahrenheit - 212.0).abs() < 1e-12, "{fahrenheit}");
}

#[test]
fn from_f64_is_the_shortest_decimal() {
    assert_eq!(from_f64(0.1).to_string(), "0.1");
    assert_eq!(from_f64(1e24).to_string(), "1000000000000000000000000");
    assert_eq!(from_f64(1.0 / 3.0).to_string(), "0.3333333333333333");
    assert_eq!(
        from_f64(core::f64::consts::PI).to_string(),
        "3.141592653589793"
    );
    assert_eq!(from_f64(-2.5).to_string(), "-2.5");
    assert_eq!(from_f64(0.0).to_string(), "0");
    assert_eq!(from_f64(f64::NAN).to_string(), "0");
}

#[test]
fn to_f64_is_the_nearest_f64() {
    // This used to be 1.0000000000000001e-21
    assert_eq!(to_f64(from_f64(1e-21)), 1e-21);
    for value in [
        0.1,
        2.54,
        1e-9,
        1e24,
        453.59237,
        core::f64::consts::PI,
        -273.15,
    ] {
        assert_eq!(to_f64(from_f64(value)), value, "{value}");
    }
}
