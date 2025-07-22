use octofhir_ucum_core::*;

fn main() {
    println!("=== UCUM-RS Comprehensive API Demo ===\n");

    // Phase 1: Basic Validation and Analysis
    demo_validation_and_analysis();

    // Phase 2: Unit Arithmetic Operations
    demo_unit_arithmetic();

    // Phase 3: Search and Discovery
    demo_search_functionality();

    // Phase 4: Property-based Validation
    demo_property_validation();

    // Phase 5: Special Units and Temperature
    demo_special_units();
}

fn demo_validation_and_analysis() {
    println!("🔍 === Validation and Analysis ===");

    let units = vec!["m/s", "kg.m/s2", "invalid_unit", "Cel", "[in_i]"];

    for unit in units {
        print!("Unit '{}': ", unit);

        match validate(unit) {
            Ok(()) => {
                println!("✅ Valid");

                // Perform detailed analysis
                if let Ok(analysis) = analyse(unit) {
                    println!("  📊 Factor: {:.6}", analysis.factor);
                    println!("  📐 Dimension: {:?}", analysis.dimension);
                    if analysis.has_offset {
                        println!("  🌡️  Has temperature offset: {:.2}", analysis.offset);
                    }
                    if analysis.is_dimensionless {
                        println!("  ⚪ Dimensionless unit");
                    }
                }
            }
            Err(e) => println!("❌ Invalid: {}", e),
        }
        println!();
    }
}

fn demo_unit_arithmetic() {
    println!("🧮 === Unit Arithmetic Operations ===");

    // Multiplication examples
    let multiplications = vec![("m", "s"), ("kg", "m/s2"), ("V", "A")];

    println!("Multiplication:");
    for (unit1, unit2) in multiplications {
        match unit_multiply(unit1, unit2) {
            Ok(result) => {
                println!(
                    "  {} × {} = {} (factor: {:.3})",
                    unit1, unit2, result.expression, result.factor
                );
            }
            Err(e) => println!("  {} × {} = Error: {}", unit1, unit2, e),
        }
    }

    // Division examples
    let divisions = vec![("m", "s"), ("J", "s"), ("kg.m2", "s2")];

    println!("\nDivision:");
    for (numerator, denominator) in divisions {
        match unit_divide(numerator, denominator) {
            Ok(result) => {
                println!(
                    "  {} ÷ {} = {} (factor: {:.3})",
                    numerator, denominator, result.expression, result.factor
                );
            }
            Err(e) => println!("  {} ÷ {} = Error: {}", numerator, denominator, e),
        }
    }
    println!();
}

fn demo_search_functionality() {
    println!("🔎 === Search and Discovery ===");

    // Basic search
    println!("Search for 'meter':");
    let results = search_units("meter");
    for (i, unit) in results.iter().take(5).enumerate() {
        println!("  {}. {} - {}", i + 1, unit.code, unit.display_name);
    }

    // Property-based search
    println!("\nLength units:");
    let length_units = search_units_by_property("length");
    for (i, unit) in length_units.iter().take(5).enumerate() {
        println!("  {}. {} - {}", i + 1, unit.code, unit.display_name);
    }

    // Get defined forms
    println!("\nDefined forms of 'g' (gram):");
    let gram_forms = get_defined_forms("g");
    for (i, unit) in gram_forms.iter().take(5).enumerate() {
        println!("  {}. {} - {}", i + 1, unit.code, unit.display_name);
    }
    println!();
}

fn demo_property_validation() {
    println!("🏷️  === Property-based Validation ===");

    let validations = vec![
        ("m", "length"),
        ("kg", "mass"),
        ("s", "time"),
        ("m", "mass"), // This should fail
        ("kg.m/s2", "force"),
        ("J", "energy"),
    ];

    for (unit, property) in validations {
        match validate_in_property(unit, property) {
            Ok(is_valid) => {
                let status = if is_valid { "✅" } else { "❌" };
                println!(
                    "  {} '{}' for property '{}': {}",
                    status,
                    unit,
                    property,
                    if is_valid { "Valid" } else { "Invalid" }
                );
            }
            Err(e) => println!("  ❓ '{}' for property '{}': Error - {}", unit, property, e),
        }
    }
    println!();
}

fn demo_special_units() {
    println!("🌡️  === Special Units and Temperature ===");

    // Unit compatibility checking
    let comparisons = vec![
        ("m", "km"),
        ("kg", "g"),
        ("Cel", "K"),
        ("m", "kg"), // Should be incompatible
    ];

    println!("Unit compatibility:");
    for (unit1, unit2) in comparisons {
        match is_comparable(unit1, unit2) {
            Ok(compatible) => {
                let status = if compatible {
                    "✅ Compatible"
                } else {
                    "❌ Incompatible"
                };
                println!("  {} and {}: {}", unit1, unit2, status);
            }
            Err(e) => println!("  {} and {}: Error - {}", unit1, unit2, e),
        }
    }

    // Canonical units
    println!("\nCanonical unit forms:");
    let units_for_canonical = vec!["km", "mg", "kPa", "MHz"];
    for unit in units_for_canonical {
        match get_canonical_units(unit) {
            Ok(canonical) => {
                println!(
                    "  {} → {} (factor: {:.6})",
                    unit, canonical.unit, canonical.factor
                );
            }
            Err(e) => println!("  {}: Error - {}", unit, e),
        }
    }

    println!("\n🎉 Demo completed! UCUM-RS provides comprehensive unit handling capabilities.");
}
