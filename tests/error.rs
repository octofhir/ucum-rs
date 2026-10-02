use octofhir_ucum::{ErrorKind, OwnedUnitExpr, parse_expression, validate};

#[test]
fn multiple_slash_allowed() {
    // Multiple slashes should be allowed per UCUM §7.4 (left-to-right evaluation)
    let result = parse_expression("kg/m/s").unwrap();
    // Should parse as ((kg/m)/s)
    assert!(matches!(result, OwnedUnitExpr::Quotient(_, _)));
}

#[test]
fn percent_in_unit_codes() {
    // '%' may be part of a symbol (UCUM §3.2), so this is an unknown unit, not a syntax error
    let err = validate("kg%g").unwrap_err();
    assert!(matches!(err.kind, ErrorKind::UnitNotFound { .. }));
    // ...while these are units: gram percent, and percent with an annotation
    assert!(validate("g%").is_ok());
    assert!(validate("%{vol}").is_ok());
    assert!(validate("mg/%").is_ok());
}
