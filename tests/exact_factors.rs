//! The magnitudes of units are exact `Number`s of any magnitude, like the factors the
//! library computes (README, "Exact decimal arithmetic"): a valid unit such as `10*1000`
//! has a factor of exactly 10^1000, which an `f64` would turn into infinity.

use octofhir_ucum::{
    AdvancedConversionContext, ErrorKind, Number, analyse, convert, convert_with_context,
    get_canonical_units, unit_divide, unit_multiply,
};

fn ten_to(exp: i32) -> Number {
    Number::power_of_ten(exp)
}

#[test]
fn analysis_and_canonical_factors_are_exact_beyond_the_f64_range() {
    for (unit, factor, canonical) in [
        ("10*1000", ten_to(1000), "1"),
        ("10*-1000", ten_to(-1000), "1"),
        ("%1000", ten_to(-2000), "1"),
        ("Ym20", ten_to(480), "m20"),
        ("ym20", ten_to(-480), "m20"),
    ] {
        // These were inf or 0
        assert_eq!(analyse(unit).unwrap().factor, factor, "{unit}");
        let result = get_canonical_units(unit).unwrap();
        assert_eq!(
            (result.factor, result.unit.as_str()),
            (factor, canonical),
            "{unit}"
        );
    }
    // And exact within it
    assert_eq!(
        analyse("[in_i]").unwrap().factor,
        Number::parse("0.0254").unwrap()
    );
    assert_eq!(
        get_canonical_units("[m_e]").unwrap().factor,
        Number::parse("9.1093837139e-28").unwrap()
    );
}

#[test]
fn unit_arithmetic_is_exact_beyond_the_f64_range() {
    // These were a PrecisionOverflow
    let product = unit_multiply("10*1000", "10*1000").unwrap();
    assert_eq!(product.expression, "10*1000.10*1000");
    assert_eq!(product.factor, ten_to(2000));
    assert_eq!(
        unit_divide("10*-1000", "Ym20").unwrap().factor,
        ten_to(-1480)
    );
    // The expression evaluates to the same factor
    assert_eq!(analyse(&product.expression).unwrap().factor, product.factor);
}

#[test]
fn a_power_of_ten_beyond_a_number_is_an_overflow() {
    // The exponent of a `Number` is an i32
    let big = "10*2000000000";
    assert_eq!(analyse(big).unwrap().factor, ten_to(2_000_000_000));
    for result in [unit_multiply(big, big), unit_divide("10*-2000000000", big)] {
        let err = result.unwrap_err();
        assert!(
            matches!(err.kind, ErrorKind::PrecisionOverflow { .. }),
            "{:?}",
            err.kind
        );
    }
    let err = get_canonical_units(&format!("{big}.{big}")).unwrap_err();
    assert!(
        matches!(err.kind, ErrorKind::PrecisionOverflow { .. }),
        "{:?}",
        err.kind
    );
}

#[test]
fn temperatures_have_their_offset_in_base_units() {
    // A prefix scales the value, not the offset (§22.3)
    for unit in ["Cel", "mCel", "kCel", "2.Cel"] {
        assert_eq!(
            analyse(unit).unwrap().offset,
            Number::parse("273.15").unwrap(),
            "{unit}"
        );
        assert_eq!(
            get_canonical_units(unit).unwrap().offset,
            Number::parse("273.15").unwrap()
        );
    }
    // 459.67 × 5/9 K
    let offset = analyse("[degF]").unwrap().offset;
    assert_eq!(offset, get_canonical_units("[degF]").unwrap().offset);
    assert!((offset.to_f64() - 255.372_222_222_222_2).abs() < 1e-12);
    assert_eq!(analyse("m").unwrap().offset, Number::ZERO);
}

#[test]
fn linear_conversions_have_exact_coefficients() {
    let context = AdvancedConversionContext::default();
    // 0 converts to 0, with a factor of 10^800
    let result = convert_with_context(0.0, "10*400", "10*-400", &context).unwrap();
    assert_eq!(result.value, 0.0);
    assert_eq!(result.factor, Some(ten_to(800)));
    let result = convert_with_context(0.0, "Cel", "[degF]", &context).unwrap();
    assert_eq!(result.value, 32.0);
    assert_eq!(result.factor, Some(Number::parse("1.8").unwrap()));
    assert_eq!(result.offset, Some(Number::from(32)));
}

#[test]
fn values_of_quantities_stay_f64() {
    // A value that does not fit in an f64 is still an error
    assert!(convert(1.0, "10*1000", "1").is_err());
    assert_eq!(convert(1.0, "10*1000", "10*999").unwrap(), 10.0);
}
