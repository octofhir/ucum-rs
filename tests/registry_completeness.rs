use octofhir_ucum::{find_unit, get_all_units};
use std::collections::HashSet;
use std::fs;

#[test]
fn test_all_units_from_xml_are_in_registry() {
    // Read and parse the ucum-essence.xml file
    let xml_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ucum-essence.xml");
    let xml_content = fs::read_to_string(&xml_path).expect("Failed to read ucum-essence.xml");

    // Extract all unit codes from XML
    let xml_units = extract_unit_codes_from_xml(&xml_content);
    println!("Found {} units in ucum-essence.xml", xml_units.len());

    // Get all units from registry
    let registry_units = get_all_units();
    let registry_codes: HashSet<String> = registry_units
        .iter()
        .map(|unit| unit.code.to_string())
        .collect();

    println!("Found {} units in registry", registry_codes.len());

    // Check for missing units
    let mut missing_units = Vec::new();
    let mut found_units = Vec::new();

    for xml_unit in &xml_units {
        if registry_codes.contains(xml_unit) {
            found_units.push(xml_unit.clone());
        } else {
            missing_units.push(xml_unit.clone());
        }
    }

    // Print results
    println!("Units found in registry: {}", found_units.len());
    if !missing_units.is_empty() {
        println!("Missing units from registry:");
        for unit in &missing_units {
            println!("  - {unit}");
        }
    }

    // Also check if we can lookup each unit
    let mut lookup_failures = Vec::new();
    for xml_unit in &xml_units {
        if find_unit(xml_unit).is_none() {
            lookup_failures.push(xml_unit.clone());
        }
    }

    if !lookup_failures.is_empty() {
        println!("Units that failed lookup:");
        for unit in &lookup_failures {
            println!("  - {unit}");
        }
    }

    // Print some sample units for verification
    println!("\nSample units from XML:");
    for (i, unit) in xml_units.iter().take(10).enumerate() {
        println!("  {}. {}", i + 1, unit);
    }

    println!("\nSample units from registry:");
    for (i, unit) in registry_units.iter().take(10).enumerate() {
        println!("  {}. {} ({})", i + 1, unit.code, unit.display_name);
    }

    // The test passes if all units from XML are found in registry
    assert!(
        missing_units.is_empty(),
        "Found {} missing units in registry: {:?}",
        missing_units.len(),
        missing_units
    );

    // Also verify lookup works for all units
    assert!(
        lookup_failures.is_empty(),
        "Found {} units that failed lookup: {:?}",
        lookup_failures.len(),
        lookup_failures
    );
}

fn extract_unit_codes_from_xml(xml_content: &str) -> Vec<String> {
    let mut unit_codes = Vec::new();

    let mut reader = quick_xml::Reader::from_str(xml_content);

    loop {
        use quick_xml::events::Event;
        match reader.read_event() {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"base-unit" | b"unit" => {
                        // Extract the Code attribute
                        if let Some(code_attr) = e
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"Code")
                        {
                            let code = String::from_utf8_lossy(&code_attr.value).to_string();
                            unit_codes.push(code);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error reading XML: {e:?}"),
            _ => {}
        }
    }

    // Remove duplicates and sort
    unit_codes.sort();
    unit_codes.dedup();

    unit_codes
}

#[test]
fn test_specific_units_exist() {
    // Test some specific units that should definitely exist
    let expected_units = vec![
        "m",     // meter (base unit)
        "kg",    // kilogram (base unit)
        "s",     // second (base unit)
        "A",     // ampere (base unit)
        "K",     // kelvin (base unit)
        "mol",   // mole (base unit)
        "cd",    // candela (base unit)
        "g",     // gram
        "L",     // liter
        "l",     // liter (lowercase)
        "min",   // minute
        "h",     // hour
        "d",     // day
        "Cel",   // Celsius
        "bar",   // bar
        "Pa",    // pascal
        "N",     // newton
        "J",     // joule
        "W",     // watt
        "V",     // volt
        "Ohm",   // ohm
        "Hz",    // hertz
        "wk",    // week
        "[NTU]", // Nephelometric Turbidity Unit
        "[FNU]", // Formazin Turbidity Unit
    ];

    for unit_code in expected_units {
        let unit = find_unit(unit_code);
        assert!(
            unit.is_some(),
            "Expected unit '{unit_code}' not found in registry"
        );

        if let Some(u) = unit {
            println!("✓ Found unit '{}': {}", unit_code, u.display_name);
        }
    }
}

#[test]
fn test_registry_statistics() {
    let all_units = get_all_units();

    println!("Registry Statistics:");
    println!("  Total units: {}", all_units.len());

    // Count by property
    let mut property_counts = std::collections::HashMap::new();
    for unit in all_units {
        *property_counts.entry(&unit.property).or_insert(0) += 1;
    }

    println!("  Units by property:");
    let mut properties: Vec<_> = property_counts.into_iter().collect();
    properties.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    for (property, count) in properties {
        println!("    {property}: {count}");
    }
}
