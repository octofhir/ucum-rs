//! The syntax of UCUM expressions (UCUM §1-15): characters, prefixes, square brackets,
//! annotations, operators, numbers, exponents and parentheses.

use octofhir_ucum::{
    ErrorKind, Number, UnitExpr, evaluate, generate_display_name, get_common_display,
    get_defined_forms, parse_expression, validate,
};

fn symbol(s: &str) -> UnitExpr {
    UnitExpr::Symbol(s.to_string())
}

fn power(s: &str, exp: i32) -> UnitExpr {
    UnitExpr::Power(Box::new(symbol(s)), exp)
}

fn number(text: &str) -> UnitExpr {
    UnitExpr::Numeric(Number::parse(text).unwrap())
}

fn parse(input: &str) -> UnitExpr {
    parse_expression(input).unwrap_or_else(|e| panic!("{input:?}: {e}"))
}

fn factor(input: &str) -> Number {
    evaluate(&parse(input))
        .unwrap_or_else(|e| panic!("{input:?}: {e}"))
        .factor
}

fn assert_invalid_syntax(input: &str) {
    let err = parse_expression(input).expect_err(input);
    assert!(
        matches!(err.kind, ErrorKind::InvalidExpression { .. }),
        "{input:?}: {:?}",
        err.kind
    );
}

fn assert_same(a: &str, b: &str) {
    let (x, y) = (evaluate(&parse(a)).unwrap(), evaluate(&parse(b)).unwrap());
    assert_eq!(x.dim, y.dim, "{a} vs {b}");
    assert_eq!(x.factor, y.factor, "{a} vs {b}");
}

// §3 character set

#[test]
fn only_ascii_33_to_126_is_allowed() {
    for input in ["µg", "µ", "μg", "rad2{錠}", "{é}", "é", "m€", "m\u{0}"] {
        assert_invalid_syntax(input);
    }
}

#[test]
fn white_space_is_an_error() {
    for input in [
        " m", "m ", "m s", "kg m", "m\t", "m\n", "s -2", "{a b}", "m / s",
    ] {
        assert_invalid_syntax(input);
    }
}

#[test]
fn the_empty_expression_is_not_a_term() {
    // Official case 1-103: the unity is written "1"
    assert_invalid_syntax("");
    assert!(validate("").is_err());
    assert!(validate("1").is_ok());
}

#[test]
fn symbols_may_hold_other_ascii_characters() {
    // '%', '\'', '*', '^' are symbol characters (§3.2): known units or unknown symbols
    assert_eq!(factor("%"), Number::parse("0.01").unwrap());
    assert_eq!(parse("'"), symbol("'"));
    assert_eq!(parse("''"), symbol("''"));
    assert!(validate("g%").is_ok());
    assert!(validate("mg/%").is_ok());
    for input in ["kg%g", "m;s", "m^2", "m^", "a!b"] {
        let err = validate(input).expect_err(input);
        assert!(
            matches!(err.kind, ErrorKind::UnitNotFound { .. }),
            "{input}: {:?}",
            err.kind
        );
    }
}

#[test]
fn excluded_characters_end_a_symbol() {
    for input in ["m=s", "kg\"", "m]", "m}", "m+", "m+s", "10+3/ul"] {
        assert_invalid_syntax(input);
    }
}

// §4 prefixes

#[test]
fn prefixes_take_metric_atoms_only() {
    assert_same("dam", "10.m");
    assert_eq!(factor("mg"), Number::parse("0.001").unwrap());
    assert!(validate("k[IU]").is_ok());
    for input in ["k[in_i]", "kh", "k[pi]", "k[degF]", "kbit_s", "m[pH]", "k%"] {
        let err = validate(input).expect_err(input);
        assert!(
            matches!(err.kind, ErrorKind::UnitNotFound { .. }),
            "{input}"
        );
    }
}

#[test]
fn a_prefix_needs_an_atom() {
    for input in ["k", "da", "Ki", "kkm"] {
        assert!(validate(input).is_err(), "{input}");
    }
}

#[test]
fn the_longest_prefix_that_leaves_a_metric_atom_wins() {
    // §4.4: "dar" is deci-are (no "da" + "r"); "cd", "Pa" and "ph" are atoms because
    // "d" (day), "a" (year) and "h" (hour) are not metric
    assert_same("dar", "10.m2");
    assert_eq!(factor("cd"), Number::ONE);
    assert_same("Pa", "N/m2");
    // The table defines the phot as 1e-4 lx (not pico-hour)
    assert_same("ph", "10*-4.lx");
    assert_same("kPa", "1000.Pa");
}

// §5 square brackets

#[test]
fn square_brackets_hold_any_character() {
    assert!(validate("dB[10.nV]").is_ok());
    assert_eq!(parse("[H2O]"), symbol("[H2O]"));
    assert_eq!(parse("m[H2O]2"), power("m[H2O]", 2));
    let err = validate("[abc+ef]").unwrap_err();
    assert!(matches!(err.kind, ErrorKind::UnitNotFound { .. }));
}

#[test]
fn square_brackets_are_matched_and_not_nested() {
    for input in ["[[ft_i]]", "[a", "[a[b]]", "[ft_i"] {
        assert_invalid_syntax(input);
    }
}

// §6 annotations

#[test]
fn annotations_have_no_meaning() {
    assert_same("{rbc}", "1");
    assert_same("/{tot}", "1");
    assert_same("mL/{hb}.m2", "mL.m2");
    assert_same("m{steel}", "m");
    assert_same("mg{a+b}", "mg");
    assert_same("1{c}", "1");
    assert_eq!(parse("{rbc}"), UnitExpr::Annotation("rbc".into()));
    assert_eq!(
        parse("m-2{a}"),
        UnitExpr::Annotated(Box::new(power("m", -2)), "a".into())
    );
    assert_eq!(
        parse("1{c}"),
        UnitExpr::Annotated(Box::new(number("1")), "c".into())
    );
}

#[test]
fn an_annotation_ends_a_symbol() {
    // Official cases 1-115, 1-116, 1-118
    assert!(validate("rad2{a}").is_ok());
    assert!(validate("{a}.rad2{b}").is_ok());
    for input in ["{a}rad2{b}", "{|}1", "k{a}g", "m{a}-2"] {
        assert!(validate(input).is_err(), "{input}");
    }
}

#[test]
fn an_annotation_has_no_escape_and_no_nesting() {
    // A backslash is an ordinary character: "{a\}" is the annotation "a\"
    assert_eq!(parse("{a\\}"), UnitExpr::Annotation("a\\".into()));
    assert!(validate("{a\\}").is_ok());
    for input in ["{a\\}b}", "{a{b}", "{a{b}}", "{}", "{a", "m{abc"] {
        assert_invalid_syntax(input);
    }
}

#[test]
fn a_component_takes_one_annotation() {
    for input in ["m{a}{b}", "{a}{b}", "(m){a}", "(m.s){a}"] {
        assert_invalid_syntax(input);
    }
}

// §7 operators

#[test]
fn period_and_solidus_are_read_left_to_right() {
    // Official case 3-111a: s/m.mg is (s/m).mg
    assert_eq!(
        parse("m/s.kg"),
        UnitExpr::Product(vec![
            UnitExpr::Quotient(Box::new(symbol("m")), Box::new(symbol("s"))),
            symbol("kg"),
        ])
    );
    assert_eq!(
        parse("kg/m/s"),
        UnitExpr::Quotient(
            Box::new(UnitExpr::Quotient(
                Box::new(symbol("kg")),
                Box::new(symbol("m"))
            )),
            Box::new(symbol("s"))
        )
    );
    assert_same("s/m.mg", "s.m-1.mg");
    assert_same("kg/m.s", "kg.s/m");
    assert_same("s/m/mg", "s.m-1.mg-1");
    assert_same("umol/2.h", "umol.h/2");
    assert_same("kg/(m.s)", "kg.m-1.s-1");
}

#[test]
fn a_leading_solidus_inverts_the_component_after_it() {
    // §7.3
    assert_eq!(parse("/min"), UnitExpr::Reciprocal(Box::new(symbol("min"))));
    assert_same("/s.m", "m/s");
    assert_same("/s", "s-1");
}

#[test]
fn a_leading_solidus_is_only_at_the_start_of_the_expression() {
    for input in ["(/m)", "m.(/s)", "m/(/s)", "m//s"] {
        assert_invalid_syntax(input);
    }
}

#[test]
fn operators_are_binary_and_mandatory() {
    for input in [
        "m/", "m.", "/", ".", "m./s", "m..s", "()", "(m", "m)", "ug(8.h)", ".m",
    ] {
        assert_invalid_syntax(input);
    }
}

// §8 numbers

#[test]
fn numbers_are_positive_integers() {
    assert_eq!(
        parse("4.[pi]"),
        UnitExpr::Product(vec![number("4"), symbol("[pi]")])
    );
    // "2.5" is 2 × 5
    assert_eq!(factor("2.5"), Number::from(10));
    for input in ["0", "00", "0.m", "0/m", "m/0"] {
        assert_invalid_syntax(input);
    }
}

#[test]
fn numbers_keep_all_their_digits() {
    let digits = "1234567890123456789012345678";
    assert_eq!(parse(digits), number(digits));
    assert_eq!(
        factor(digits).to_integer_and_exponent(),
        (1_234_567_890_123_456_789_012_345_678, 0)
    );
}

#[test]
fn digits_followed_by_a_symbol_character_are_a_symbol() {
    // §8.2: "12a" is a symbol, "10*" is the atom ten
    assert!(matches!(
        validate("12a").unwrap_err().kind,
        ErrorKind::UnitNotFound { .. }
    ));
    assert_eq!(parse("10*"), symbol("10*"));
    assert_eq!(factor("10*"), Number::from(10));
}

// §9 exponents

#[test]
fn exponents_follow_a_simple_unit() {
    assert_eq!(parse("m2"), power("m", 2));
    assert_eq!(parse("s-1"), power("s", -1));
    assert_eq!(parse("m+2"), power("m", 2));
    assert_eq!(parse("[in_i]-2"), power("[in_i]", -2));
    assert_eq!(
        parse("m3.kg-1.s-2"),
        UnitExpr::Product(vec![power("m", 3), power("kg", -1), power("s", -2)])
    );
}

#[test]
fn an_exponent_raises_the_prefix_too() {
    // §9.2: 1 cm3 is 1e-6 m3
    assert_eq!(factor("cm3"), Number::parse("1e-6").unwrap());
    assert_eq!(factor("mm-1"), Number::from(1000));
}

#[test]
fn ten_powers_are_the_atom_ten_with_an_exponent() {
    assert_eq!(parse("10*3"), power("10*", 3));
    assert_eq!(parse("10*-3"), power("10*", -3));
    assert_eq!(parse("10*+3"), power("10*", 3));
    assert_eq!(parse("10^"), symbol("10^"));
    assert_eq!(factor("10^3"), Number::from(1000));
    assert_eq!(factor("10*-7"), Number::parse("1e-7").unwrap());
    assert_same("10*-2.m", "cm");
    assert_same("10*.m", "10.m");
    assert_same("10*/10^", "1");
    for input in ["10*-", "10^-", "10*+", "10*-.m"] {
        assert!(validate(input).is_err(), "{input}");
    }
}

#[test]
fn caret_is_not_an_operator() {
    // `^` is a symbol character (as in `10^`), so "m^2" is the unknown unit "m^" squared
    assert_eq!(parse("m^2"), power("m^", 2));
    for input in ["m^2", "s^-1", "[pi]^2", "{a}^2", "(m.s)^2", "10*3^2"] {
        assert!(validate(input).is_err(), "{input}");
    }
}

#[test]
fn malformed_exponents_are_rejected() {
    for input in ["s-", "s--1", "s-2-3", "m2-1", "m2+1", "m/-2", "10-3"] {
        assert_invalid_syntax(input);
    }
    // An exponent out of the i32 range
    assert_invalid_syntax("s99999999999");
    assert_invalid_syntax("s-99999999999");
}

#[test]
fn dimension_exponents_must_fit() {
    assert!(validate("m127").is_ok());
    assert!(validate("m-128").is_ok());
    for input in ["m128", "m-127.m-2", "m100.m100", "s2147483647"] {
        let err = validate(input).expect_err(input);
        assert!(
            matches!(err.kind, ErrorKind::PrecisionOverflow { .. }),
            "{input}: {:?}",
            err.kind
        );
    }
}

// §10 parentheses

#[test]
fn parentheses_nest_terms() {
    assert_same("kg/(m.s)", "kg/m/s");
    assert_same("(10*)", "10");
    assert_same("((m))", "m");
}

#[test]
fn nothing_follows_a_closing_parenthesis() {
    // The exponent on parentheses was struck from §10
    for input in ["(m.s)2", "(m.s)-1", "(s)-2", "(m/s)2"] {
        assert_invalid_syntax(input);
    }
}

#[test]
fn a_prefix_cannot_precede_parentheses() {
    assert!(validate("k(m)").is_err());
}

// Display

#[test]
fn expressions_display_as_written() {
    for input in [
        "kg.m/s2",
        "/min",
        "/s.m",
        "m/s.kg",
        "kg/(m.s)",
        "10*-3",
        "10*-3.L",
        "4.[pi].10*-7.N/A2",
        "mg{total}",
        "{rbc}",
        "1{c}",
        "m[H2O]2",
        "mL/{hb}.m2",
    ] {
        assert_eq!(parse(input).to_string(), input);
    }
    // A plus sign is not kept
    assert_eq!(parse("m+2").to_string(), "m2");
}

#[test]
fn large_numbers_display_as_integers() {
    // A number is written with its digits (§8.1), never as "1e21"
    for (input, number) in [
        ("100000000000000000000", "1e20"),
        ("1000000000000000000000", "1e21"),
        (
            "1234567890123456789012345678.m",
            "1.234567890123456789012345678e27",
        ),
        ("m/10000000000000000000000000000000", "1e-31"),
    ] {
        let displayed = parse(input).to_string();
        assert_eq!(displayed, input);
        assert_eq!(
            factor(&displayed),
            Number::parse(number).unwrap(),
            "{input}"
        );
    }
    assert_eq!(
        generate_display_name(&parse("1000000000000000000000.m")),
        "1000000000000000000000 * (meter)"
    );
}

#[test]
fn integers_built_by_hand_display_as_ucum() {
    let numeric = |text: &str| UnitExpr::Numeric(Number::parse(text).unwrap());
    let over = |expr: UnitExpr| UnitExpr::Quotient(Box::new(symbol("m")), Box::new(expr));
    // Past 100 zeros, an integer is a power of ten, not a string of zeros
    for (expr, shown) in [
        (numeric("1e100"), format!("1{}", "0".repeat(100))),
        (numeric("1e101"), "10*101".to_string()),
        (over(numeric("1e101")), "m/10*101".to_string()),
        (over(numeric("1.23e200")), "m/(123.10*198)".to_string()),
        (numeric("1e2000000000"), "10*2000000000".to_string()),
    ] {
        assert_eq!(expr.to_string(), shown);
        let parsed = parse(&shown);
        if let Ok(expected) = evaluate(&expr) {
            assert_eq!(
                evaluate(&parsed).unwrap().factor,
                expected.factor,
                "{shown}"
            );
        }
    }
    // Other numbers have no UCUM form: they are written in decimal and do not parse back
    for (text, shown) in [("0.5", "0.5"), ("-2", "-2"), ("0", "0")] {
        assert_eq!(numeric(text).to_string(), shown);
        assert!(parse_expression(shown).is_err(), "{shown}");
    }
}

// Robustness

#[test]
fn non_ascii_input_is_an_error_not_a_panic() {
    for code in ["é", "éa", "aé", "mé", "kéé", "€", "{é}"] {
        assert!(validate(code).is_err(), "{code}");
        let err = evaluate(&symbol(code)).unwrap_err();
        assert!(matches!(err.kind, ErrorKind::UnitNotFound { .. }), "{code}");
        assert_eq!(get_common_display(code), code);
        assert!(get_defined_forms(code).is_empty());
    }
}
