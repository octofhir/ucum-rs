use octofhir_ucum_core::{UcumError, parse_expression};

#[test]
fn multiple_slash_error() {
    let err = parse_expression("kg/m/s").unwrap_err();
    assert!(matches!(err, UcumError::MultipleSlash));
}

#[test]
fn invalid_percent_error() {
    let err = parse_expression("kg%g").unwrap_err();
    assert!(matches!(err, UcumError::InvalidPercentPlacement));
}
