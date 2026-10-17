//! The public `Number` type: exact decimals of any magnitude.

use octofhir_ucum::Number;

fn n(text: &str) -> Number {
    Number::parse(text).unwrap_or_else(|| panic!("{text}"))
}

#[test]
fn parses_and_displays_decimals() {
    for (text, shown) in [
        ("2.54", "2.54"),
        ("0.0063", "0.0063"),
        ("6300000", "6300000"),
        ("-273.15", "-273.15"),
        ("+5", "5"),
        ("1e-7", "0.0000001"),
        ("1e-8", "1e-8"),
        ("6.02214076e23", "6.02214076e23"),
        ("000.000", "0"),
        (".5", "0.5"),
        ("5.", "5"),
    ] {
        assert_eq!(n(text).to_string(), shown, "{text}");
    }
    for text in ["", ".", "-", "1.2.3", "1e", "e5", "1x", "1 2"] {
        assert!(Number::parse(text).is_none(), "{text:?}");
    }
}

#[test]
fn rounds_to_28_significant_digits() {
    assert_eq!(
        n("3.1415926535897932384626433832795028841971693993751").to_string(),
        "3.141592653589793238462643383"
    );
    assert_eq!(n("9999999999999999999999999999.5").to_string(), "1e28");
}

#[test]
fn shows_scientific_notation() {
    assert_eq!(format!("{:e}", n("2.54")), "2.54e0");
    assert_eq!(format!("{:e}", n("1e-7")), "1e-7");
}

#[test]
fn converts_from_and_to_f64() {
    assert_eq!(Number::from_f64(0.1).unwrap(), n("0.1"));
    assert_eq!(Number::from_f64(1e24).unwrap(), n("1e24"));
    assert_eq!(Number::from_f64(-2.5).unwrap(), n("-2.5"));
    assert_eq!(Number::from_f64(0.0).unwrap(), Number::ZERO);
    assert!(Number::from_f64(f64::NAN).is_none());
    assert!(Number::from_f64(f64::INFINITY).is_none());
    assert_eq!(n("1e-21").to_f64(), 1e-21);
    for value in [
        0.1,
        2.54,
        1e-9,
        1e24,
        453.59237,
        core::f64::consts::PI,
        -273.15,
    ] {
        assert_eq!(Number::from_f64(value).unwrap().to_f64(), value, "{value}");
    }
    assert_eq!(n("1e400").to_f64(), f64::INFINITY);
}

#[test]
fn arithmetic_is_exact_and_unbounded() {
    assert_eq!(n("6.3").checked_mul(n("0.001")).unwrap(), n("0.0063"));
    assert_eq!(n("0.1").checked_add(n("0.2")).unwrap(), n("0.3"));
    assert_eq!(n("373.15").checked_sub(n("273.15")).unwrap(), n("100"));
    assert_eq!(n("2").checked_powi(-2).unwrap(), n("0.25"));
    assert_eq!(n("1e200").checked_mul(n("1e200")).unwrap(), n("1e400"));
    assert_eq!(Number::power_of_ten(-30), n("1e-30"));
    assert!(n("1").checked_div(Number::ZERO).is_none());
    assert!(Number::ZERO.checked_powi(-1).is_none());
    assert_eq!(-n("2"), n("-2"));
    assert_eq!(n("-2").abs(), n("2"));
    assert!(n("-2").is_negative());
    assert!(Number::ZERO.is_zero());
    assert_eq!(Number::default(), Number::ZERO);
    assert_eq!(Number::from(42), n("42"));
}

#[test]
fn orders_by_value() {
    let mut numbers = [n("10"), n("-2"), n("0.5"), Number::ZERO, n("-1"), n("9")];
    numbers.sort();
    let shown: Vec<String> = numbers.iter().map(Number::to_string).collect();
    assert_eq!(shown, ["-2", "-1", "0", "0.5", "9", "10"]);
}

#[test]
fn splits_into_integer_and_exponent() {
    assert_eq!(n("2.54").to_integer_and_exponent(), (254, -2));
    assert_eq!(n("6300000").to_integer_and_exponent(), (63, 5));
    assert_eq!(n("1").to_integer_and_exponent(), (1, 0));
    assert_eq!(n("-0.05").to_integer_and_exponent(), (-5, -2));
}
