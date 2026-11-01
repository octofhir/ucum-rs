//! With the `serde` feature, an error serializes as a struct a service can return as the
//! body of an HTTP error: its code, message, suggestions and context, and the fields of
//! its kind in `details`.
#![cfg(feature = "serde")]

use octofhir_ucum::{
    ErrorKind, UcumError, convert, divide_by, get_canonical_units, multiply, unit_multiply,
    validate, validate_in_property,
};
use serde_json::{Value, json};

fn body(err: &UcumError) -> Value {
    serde_json::to_value(err).unwrap()
}

#[test]
fn an_error_serializes_as_its_code_message_and_details() {
    let err = validate("m//s").unwrap_err();
    let ErrorKind::InvalidExpression { reason, .. } = &err.kind else {
        panic!("{:?}", err.kind)
    };
    assert_eq!(
        body(&err),
        json!({
            "code": "invalid_expression",
            "message": err.message,
            "suggestions": err.suggestions,
            "context": ["While parsing UCUM expression: 'm//s'"],
            "details": {"reason": reason, "expression": "m//s", "position": 2},
        })
    );
    let err = convert(1.0, "m", "kg").unwrap_err();
    assert_eq!(
        body(&err),
        json!({
            "code": "dimension_mismatch",
            "message": "Dimension mismatch in conversion from 'm' to 'kg': expected M, found L",
            "suggestions": [],
            "context": [],
            "details": {
                "expected": [0, 0, 1, 0, 0, 0, 0],
                "found": [1, 0, 0, 0, 0, 0, 0],
                "operation": "conversion from 'm' to 'kg'",
                "from": "m",
                "to": "kg",
            },
        })
    );
    let err = validate("Pascal").unwrap_err();
    let value = body(&err);
    assert_eq!(value["code"], "unit_not_found");
    assert_eq!(value["details"], json!({"unit": "Pascal"}));
    assert_eq!(value["suggestions"], json!(err.suggestions));
    assert!(!err.suggestions.is_empty());
}

#[test]
fn every_kind_serializes_with_its_code() {
    let mut errors = vec![
        validate("m//s").unwrap_err(),
        validate(&"(".repeat(200)).unwrap_err(),
        validate("xyz").unwrap_err(),
        validate_in_property("m", "colour").unwrap_err(),
        convert(1.0, "m", "kg").unwrap_err(),
        convert(1.0, "[IU]", "1").unwrap_err(),
        convert(f64::NAN, "m", "km").unwrap_err(),
        convert(0.0, "mol/L", "[pH]").unwrap_err(),
        convert(1e308, "km", "m").unwrap_err(),
        divide_by(1.0, "m", 0.0, "s").unwrap_err(),
        multiply(1.0, "Cel", 1.0, "m").unwrap_err(),
        get_canonical_units("[pH]").unwrap_err(),
        unit_multiply("B", "m").unwrap_err(),
    ];
    #[cfg(feature = "std")]
    errors.push(octofhir_ucum::search_units_regex("(", false).unwrap_err());
    let mut codes: Vec<&str> = Vec::new();
    for err in &errors {
        let value = body(err);
        assert_eq!(value["code"], err.kind.code(), "{}", err.message);
        assert_eq!(value["message"], err.message.as_str());
        assert!(value["details"].is_object(), "{}", err.message);
        codes.push(err.kind.code());
    }
    // One error of each of the 12 kinds: a kind added later fails here until it is listed
    codes.sort();
    codes.dedup();
    #[cfg(feature = "std")]
    assert_eq!(codes.len(), 12, "{codes:?}");
}
