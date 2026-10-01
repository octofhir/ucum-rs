//! Non-ASCII input must be rejected with an error (or `None`), never a panic.

use octofhir_ucum::{
    ErrorKind, UnitExpr, evaluate, find_unit, find_unit_optimized, get_common_display,
    get_defined_forms, validate,
};

/// Multi-byte characters at the start, in the middle and at the end of a code.
const NON_ASCII_CODES: &[&str] = &["é", "éa", "aé", "mé", "kéé", "€", "{é}"];

#[test]
fn find_unit_returns_none_for_non_ascii_codes() {
    for code in NON_ASCII_CODES {
        assert!(find_unit(code).is_none(), "find_unit({code:?})");
        assert!(
            find_unit_optimized(code).is_none(),
            "find_unit_optimized({code:?})"
        );
    }
}

#[test]
fn evaluate_reports_unit_not_found_for_non_ascii_symbols() {
    for code in NON_ASCII_CODES {
        let err = evaluate(&UnitExpr::Symbol(code)).unwrap_err();
        assert!(
            matches!(err.kind, ErrorKind::UnitNotFound { .. }),
            "evaluate(Symbol({code:?})) returned {:?}",
            err.kind
        );
    }
}

#[test]
fn validate_does_not_panic_on_non_ascii_annotation() {
    // The parser accepts any character inside an annotation, so this reaches
    // the unit lookup with a multi-byte symbol.
    assert!(validate("{é}").is_err());
    assert!(validate("{é}/m").is_err());
}

#[test]
fn display_helpers_do_not_panic_on_non_ascii_codes() {
    for code in NON_ASCII_CODES {
        assert_eq!(get_common_display(code), *code);
        assert!(get_defined_forms(code).is_empty());
    }
}
