use octofhir_ucum::{OwnedUnitExpr, parse_expression};

fn sym(s: &str) -> OwnedUnitExpr {
    OwnedUnitExpr::Symbol(s.to_string())
}

#[test]
fn unicode_micro_alias_simple() {
    assert_eq!(parse_expression("µg").unwrap(), sym("ug"));
}

#[test]
fn unicode_micro_alias_complex() {
    let expr = parse_expression("µmol/L").unwrap();
    let expected = OwnedUnitExpr::Quotient(Box::new(sym("umol")), Box::new(sym("L")));
    assert_eq!(expr, expected);
}
