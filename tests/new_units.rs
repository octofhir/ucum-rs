use octofhir_ucum::{find_unit, parse_expression};

#[test]
fn test_new_turbidity_units() {
    // Test the new turbidity units added in UCUM v2.2
    let ntu = find_unit("[NTU]");
    let fnu = find_unit("[FNU]");

    assert!(
        ntu.is_some(),
        "Nephelometric Turbidity Unit [NTU] not found"
    );
    assert!(fnu.is_some(), "Formazin Turbidity Unit [FNU] not found");

    if let Some(ntu_unit) = ntu {
        println!("✓ Found NTU: {}", ntu_unit.display_name);
    }

    if let Some(fnu_unit) = fnu {
        println!("✓ Found FNU: {}", fnu_unit.display_name);
    }
}

#[test]
fn test_fhir_relevant_units() {
    // Test units commonly used in FHIR contexts
    let fhir_units = vec![
        "%",      // percent
        "mg/dL",  // milligrams per deciliter
        "mmol/L", // millimoles per liter
        "mEq/L",  // milliequivalents per liter
        "U/L",    // units per liter
        "mm[Hg]", // millimeters of mercury
        "Cel",    // Celsius
        "[degF]", // Fahrenheit
        "kg/m2",  // BMI unit
        "g/dL",   // grams per deciliter
    ];

    for unit_code in fhir_units {
        // Try to parse each unit expression
        let result = parse_expression(unit_code);
        assert!(
            result.is_ok(),
            "Failed to parse FHIR-relevant unit '{}': {:?}",
            unit_code,
            result.err()
        );

        if let Ok(expr) = result {
            println!("✓ Parsed FHIR unit '{}': {:?}", unit_code, expr);
        }
    }
}

#[test]
fn test_fhir_quantity_units() {
    // Test additional units commonly found in FHIR Quantity resources
    let quantity_units = vec![
        "mg",        // milligram
        "mL",        // milliliter
        "cm",        // centimeter
        "mm",        // millimeter
        "kg",        // kilogram
        "lb",        // pound
        "[in_i]",    // inch
        "[ft_i]",    // foot
        "beats/min", // beats per minute
        "/min",      // per minute
        "deg",       // degree
        "rad",       // radian
    ];

    for unit_code in quantity_units {
        let result = parse_expression(unit_code);
        assert!(
            result.is_ok(),
            "Failed to parse quantity unit '{}': {:?}",
            unit_code,
            result.err()
        );

        if let Ok(expr) = result {
            println!("✓ Parsed quantity unit '{}': {:?}", unit_code, expr);
        }
    }
}
