use octofhir_ucum::precision::{NumericOps, from_f64};
use octofhir_ucum::{Dimension, find_prefix, find_unit};

#[test]
fn prefix_lookup() {
    let k = find_prefix("k").expect("kilo prefix");
    assert_eq!(k.factor, 1e3);
    let m = find_prefix("m").expect("milli prefix");
    assert_eq!(m.factor, 1e-3);
}

#[test]
fn base_unit_lookup() {
    let meter = find_unit("m").expect("meter");
    assert_eq!(meter.factor, 1.0);
    assert_eq!(meter.dim, Dimension([0, 1, 0, 0, 0, 0, 0]));
}

#[test]
fn gram_unit_lookup() {
    let gram = find_unit("g").expect("gram");
    assert_eq!(gram.factor, 1.0);
    assert_eq!(gram.dim, Dimension([1, 0, 0, 0, 0, 0, 0])); // Mass dimension
}

#[test]
fn milligram_unit_test() {
    // Test if "mg" can be found directly (it should now be found as a prefixed unit)
    let mg_direct = find_unit("mg");
    assert!(mg_direct.is_some(), "mg should be found as a prefixed unit");

    // The returned unit should be the base gram unit
    let mg_unit = mg_direct.unwrap();
    assert_eq!(mg_unit.code, "g", "mg should resolve to base unit 'g'");
    assert_eq!(
        mg_unit.dim,
        Dimension([1, 0, 0, 0, 0, 0, 0]),
        "mg should have mass dimension"
    );

    // Test if we can parse and evaluate "mg" as a prefixed unit
    use octofhir_ucum::{evaluate_owned, parse_expression};
    let expr = parse_expression("mg").expect("should parse mg");
    let result = evaluate_owned(&expr).expect("should evaluate mg");

    // mg should have factor 1e-3 (milli) and mass dimension
    assert!(
        (result.factor.sub(from_f64(1e-3))).abs() < from_f64(1e-12),
        "mg factor should be 1e-3, got {}",
        result.factor
    );
    assert_eq!(
        result.dim,
        Dimension([1, 0, 0, 0, 0, 0, 0]),
        "mg should have mass dimension"
    );
}

#[test]
fn bracketed_units_lookup() {
    for code in ["[pi]", "[in_i]", "[mu_0]", "[ly]"] {
        let unit = find_unit(code).unwrap_or_else(|| panic!("{code}"));
        assert_eq!(unit.code, code);
    }
    assert_eq!(find_unit("[pi]").unwrap().factor, core::f64::consts::PI);
    assert!((find_unit("[in_i]").unwrap().factor - 0.0254).abs() < 1e-15);
}
