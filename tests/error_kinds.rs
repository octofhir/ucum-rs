//! Each failure has a kind of its own, with the data a caller needs to report it, and a
//! stable code (`ErrorKind::code`), for instance to map errors to HTTP statuses.

use octofhir_ucum::{
    AdvancedConversionContext, Case, ErrorKind, UcumError, UnitExpr, convert, convert_with_context,
    display_name, divide_by, evaluate, get_canonical_units, multiply, unit_multiply, validate,
    validate_in_property,
};

#[test]
fn syntax_errors_give_the_position_where_parsing_stopped() {
    for (expression, position) in [
        ("m//s", 2),
        ("m s", 1),
        ("kg/(m", 5),
        ("m.", 2),
        ("0.m", 0),
        ("mé", 1),
    ] {
        let err = validate(expression).unwrap_err();
        match err.kind {
            ErrorKind::InvalidExpression {
                position: found,
                expression: parsed,
                ..
            } => {
                assert_eq!(found, Some(position), "{expression}");
                assert_eq!(parsed, expression);
            }
            other => panic!("{expression}: {other:?}"),
        }
        assert!(
            err.message.ends_with(&format!("at position {position}")),
            "{expression}: {}",
            err.message
        );
    }
    // Not tied to a position
    let err = validate("").unwrap_err();
    assert!(matches!(
        err.kind,
        ErrorKind::InvalidExpression { position: None, .. }
    ));
}

#[test]
fn the_position_refers_to_the_expression_it_was_found_in() {
    // Which operand of a two-operand function is wrong
    let operand = |err: UcumError| match err.kind {
        ErrorKind::InvalidExpression {
            expression,
            position,
            ..
        } => (expression, position),
        other => panic!("{other:?}"),
    };
    assert_eq!(
        operand(convert(1.0, "m", "kg//s").unwrap_err()),
        ("kg//s".to_string(), Some(3))
    );
    assert_eq!(
        operand(convert(1.0, "kg//s", "m").unwrap_err()),
        ("kg//s".to_string(), Some(3))
    );
    assert_eq!(
        operand(unit_multiply("m", "s..").unwrap_err()),
        ("s..".to_string(), Some(2))
    );
}

#[test]
fn units_of_different_dimensions_are_a_dimension_mismatch() {
    let err = convert(1.0, "m", "kg").unwrap_err();
    match &err.kind {
        ErrorKind::DimensionMismatch {
            expected,
            found,
            from,
            to,
            ..
        } => {
            // The target, then the source
            assert_eq!(
                (expected.to_string(), found.to_string()),
                ("M".into(), "L".into())
            );
            assert_eq!((from.as_deref(), to.as_deref()), (Some("m"), Some("kg")));
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(
        err.message,
        "Dimension mismatch in conversion from 'm' to 'kg': expected M, found L"
    );
}

#[test]
fn a_conversion_involving_an_arbitrary_unit_is_a_conversion_error() {
    // An arbitrary unit has no dimension (§24.3), nor has a term with one (§25.1), so this
    // is never a `DimensionMismatch`
    for (from, to) in [
        ("[IU]", "1"),
        ("[IU]", "[arb'U]"),
        ("[IU]/mL", "/mL"),
        ("[IU]", "m"),
        ("m", "[IU]"),
        ("[IU]/mL", "m"),
    ] {
        let err = convert(1.0, from, to).unwrap_err();
        assert!(
            matches!(err.kind, ErrorKind::ConversionError { .. }),
            "{from} to {to}: {:?}",
            err.kind
        );
    }
}

#[test]
fn every_entry_point_suggests_corrections_for_an_unknown_unit() {
    let suggestions = |err: UcumError| {
        assert_eq!(err.kind.code(), "unit_not_found");
        err.suggestions
    };
    let expected = suggestions(validate("metr").unwrap_err());
    assert!(!expected.is_empty());
    for err in [
        convert(1.0, "metr", "m").unwrap_err(),
        convert(1.0, "m", "metr").unwrap_err(),
        get_canonical_units("metr").unwrap_err(),
        multiply(1.0, "metr", 1.0, "s").unwrap_err(),
        unit_multiply("s", "metr").unwrap_err(),
        display_name("metr").unwrap_err(),
    ] {
        assert_eq!(suggestions(err), expected);
    }
    #[cfg(feature = "fhir")]
    {
        use octofhir_ucum::fhir::{FhirError, FhirQuantity, convert_quantity};
        match convert_quantity(&FhirQuantity::with_ucum_code(1.0, "metr"), "m").unwrap_err() {
            FhirError::UcumError(err) => assert_eq!(suggestions(err), expected),
            other => panic!("{other:?}"),
        }
    }
}

#[test]
fn a_case_insensitive_lookup_gets_no_suggestions() {
    // The suggestions are case sensitive codes: 'Pa' read case insensitively is PA, the
    // picoampere (§3.4)
    let context = AdvancedConversionContext {
        case: Case::Insensitive,
        ..Default::default()
    };
    for code in ["PASCAL", "METR", "GAUS"] {
        let err = convert_with_context(1.0, code, "1", &context).unwrap_err();
        assert_eq!(err.kind.code(), "unit_not_found", "{code}");
        assert!(err.suggestions.is_empty(), "{code}: {:?}", err.suggestions);
    }
}

#[test]
fn an_unknown_unit_gives_where_its_symbol_starts() {
    let found = |err: UcumError| match err.kind {
        ErrorKind::UnitNotFound {
            unit,
            expression,
            position,
            ..
        } => (unit, expression, position),
        other => panic!("{other:?}"),
    };
    for (expression, unit, position) in [
        ("qq", "qq", 0),
        ("m/qq", "qq", 2),
        ("mqq2", "mqq", 0),
        // Not inside an annotation, nor inside a longer symbol
        ("m{qq}.qq", "qq", 6),
        ("kg.[in_i]/[qq]", "[qq]", 10),
        ("4.qq-1", "qq", 2),
    ] {
        assert_eq!(
            found(validate(expression).unwrap_err()),
            (unit.to_string(), expression.to_string(), Some(position)),
            "{expression}"
        );
    }
    // The expression tells which operand it is
    assert_eq!(
        found(convert(1.0, "m", "s/qq").unwrap_err()),
        ("qq".to_string(), "s/qq".to_string(), Some(2))
    );
    assert_eq!(
        found(unit_multiply("s", "m.qq").unwrap_err()),
        ("qq".to_string(), "m.qq".to_string(), Some(2))
    );
    assert_eq!(found(display_name("m/qq").unwrap_err()).2, Some(2));
    // A case insensitive lookup, which reads the symbol as written
    let context = AdvancedConversionContext {
        case: Case::Insensitive,
        ..Default::default()
    };
    assert_eq!(
        found(convert_with_context(1.0, "M/QQ", "1", &context).unwrap_err()),
        ("QQ".to_string(), "M/QQ".to_string(), Some(2))
    );
    // A syntax tree built by hand has no expression
    let err = evaluate(&UnitExpr::Symbol("qq".into())).unwrap_err();
    assert_eq!(found(err), ("qq".to_string(), String::new(), None));
}

#[test]
fn a_tree_built_by_hand_too_deep_says_so_in_nodes() {
    let mut expr = UnitExpr::Symbol("m".into());
    for _ in 0..200 {
        expr = UnitExpr::Product(vec![expr]);
    }
    let err = evaluate(&expr).unwrap_err();
    assert_eq!(err.kind.code(), "nesting_too_deep");
    assert_eq!(err.message, "Invalid syntax tree: deeper than 131 nodes");
}

#[cfg(feature = "std")]
#[test]
fn an_invalid_search_pattern_is_not_an_expression_error() {
    let err = octofhir_ucum::search_units_regex("[invalid", false).unwrap_err();
    match &err.kind {
        ErrorKind::InvalidPattern { pattern, .. } => assert_eq!(pattern, "[invalid"),
        other => panic!("{other:?}"),
    }
    // The reason of the regex crate, without the pattern it repeats
    assert_eq!(
        err.message,
        "Invalid search pattern: unclosed character class"
    );
}

#[test]
fn every_kind_has_a_stable_code() {
    let cases: Vec<(UcumError, &str)> = vec![
        (validate("m//s").unwrap_err(), "invalid_expression"),
        (validate(&"(".repeat(200)).unwrap_err(), "nesting_too_deep"),
        (validate("xyz").unwrap_err(), "unit_not_found"),
        (
            validate_in_property("m", "colour").unwrap_err(),
            "invalid_property",
        ),
        (convert(1.0, "m", "kg").unwrap_err(), "dimension_mismatch"),
        (convert(1.0, "[IU]", "1").unwrap_err(), "conversion_error"),
        (
            convert(f64::NAN, "m", "km").unwrap_err(),
            "non_finite_value",
        ),
        (convert(0.0, "mol/L", "[pH]").unwrap_err(), "out_of_domain"),
        (convert(1e308, "km", "m").unwrap_err(), "precision_overflow"),
        (
            divide_by(1.0, "m", 0.0, "s").unwrap_err(),
            "division_by_zero",
        ),
        (
            multiply(1.0, "Cel", 1.0, "m").unwrap_err(),
            "special_unit_error",
        ),
        (
            get_canonical_units("[pH]").unwrap_err(),
            "special_unit_error",
        ),
        (unit_multiply("B", "m").unwrap_err(), "special_unit_error"),
    ];
    for (err, code) in cases {
        assert_eq!(err.kind.code(), code, "{}", err.message);
    }
    #[cfg(feature = "std")]
    assert_eq!(
        octofhir_ucum::search_units_regex("(", false)
            .unwrap_err()
            .kind
            .code(),
        "invalid_pattern"
    );
}
