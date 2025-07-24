use octofhir_ucum_core::{Dimension, find_prefix, find_unit};
use octofhir_ucum_core::precision::{NumericOps, from_f64};

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
    use octofhir_ucum_core::{evaluate, parse_expression};
    let expr = parse_expression("mg").expect("should parse mg");
    let result = evaluate(&expr).expect("should evaluate mg");

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
fn special_units_lookup() {
    println!("Testing special units lookup:");

    let test_units = ["[pi]", "[in_i]", "[mu_0]", "[ly]"];

    for unit in &test_units {
        match find_unit(unit) {
            Some(u) => println!("  {} -> Found: factor={}, dim={:?}", unit, u.factor, u.dim),
            None => println!("  {} -> NOT FOUND", unit),
        }
    }

    // Test specific units that should exist
    let pi_unit = find_unit("[pi]");
    if pi_unit.is_some() {
        let pi = pi_unit.unwrap();
        println!(
            "  [pi] details: factor={}, dim={:?}, code={}",
            pi.factor, pi.dim, pi.code
        );
    }

    let in_i_unit = find_unit("[in_i]");
    if in_i_unit.is_some() {
        let in_i = in_i_unit.unwrap();
        println!(
            "  [in_i] details: factor={}, dim={:?}, code={}",
            in_i.factor, in_i.dim, in_i.code
        );
    }
}
