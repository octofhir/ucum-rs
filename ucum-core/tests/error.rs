use octofhir_ucum_core::{UcumError, UnitExpr, parse_expression};

#[test]
fn multiple_slash_allowed() {
    // Multiple slashes should be allowed per UCUM §7.4 (left-to-right evaluation)
    let result = parse_expression("kg/m/s").unwrap();
    // Should parse as ((kg/m)/s)
    assert!(matches!(result, UnitExpr::Quotient(_, _)));
}

#[test]
fn invalid_percent_error() {
    let err = parse_expression("kg%g").unwrap_err();
    assert!(matches!(err, UcumError::InvalidPercentPlacement));
}
