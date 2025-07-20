use std::fs;
use std::path::Path;
use ucum_core::UcumParser;

#[test]
fn test_all_ucum_expressions_from_fixtures() {
    let parser = UcumParser::new();
    let fixture_path = Path::new("tests/fixtures/cases.csv");
    
    // Read the CSV file
    let content = fs::read_to_string(fixture_path)
        .expect("Failed to read cases.csv fixture file");
    
    let mut total_expressions = 0;
    let mut successful_parses = 0;
    let mut failed_parses = 0;
    let mut failed_expressions = Vec::new();
    
    // Parse each line (skip header)
    for (line_num, line) in content.lines().enumerate() {
        if line_num == 0 {
            continue; // Skip header
        }
        
        // Split by tab and get the UCUM_CODE column (index 1)
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 2 {
            let ucum_code = parts[1].trim();
            
            // Skip empty lines or invalid entries
            if ucum_code.is_empty() || ucum_code == "UCUM_CODE" {
                continue;
            }
            
            total_expressions += 1;
            
            // Try to parse the UCUM expression
            match parser.parse(ucum_code) {
                Ok(_) => {
                    successful_parses += 1;
                }
                Err(e) => {
                    failed_parses += 1;
                    failed_expressions.push((ucum_code.to_string(), e.to_string()));
                }
            }
        }
    }
    
    // Print summary
    println!("\n=== UCUM Fixture Test Results ===");
    println!("Total expressions tested: {}", total_expressions);
    println!("Successful parses: {}", successful_parses);
    println!("Failed parses: {}", failed_parses);
    println!("Success rate: {:.2}%", (successful_parses as f64 / total_expressions as f64) * 100.0);
    
    if !failed_expressions.is_empty() {
        println!("\nFailed expressions:");
        for (expr, error) in failed_expressions.iter().take(20) { // Show first 20 failures
            println!("  '{}' -> {}", expr, error);
        }
        if failed_expressions.len() > 20 {
            println!("  ... and {} more failures", failed_expressions.len() - 20);
        }
    }
    
    // Assert that we have a reasonable success rate (at least 80% should parse)
    let success_rate = successful_parses as f64 / total_expressions as f64;
    assert!(
        success_rate >= 0.8,
        "Success rate {}% is below 80%. {} out of {} expressions failed to parse.",
        success_rate * 100.0,
        failed_parses,
        total_expressions
    );
    
    println!("\n✅ Fixture test completed successfully!");
}

#[test]
fn test_specific_ucum_expressions() {
    let parser = UcumParser::new();
    
    // Test a variety of expression types that should definitely work
    let test_cases = vec![
        // Simple units
        "m", "kg", "s", "L", "g", "mol", "K", "A", "V", "W", "J", "N", "Pa", "Hz",
        
        // Division
        "m/s", "kg/m2", "L/min", "g/L", "mol/L", "A/m", "V/m", "W/m2", "J/kg", "N/m",
        
        // Concatenation
        "kg.m", "N.m", "J.s", "W.s", "Pa.s", "kg.m2", "m.s",
        
        // Exponents
        "m2", "m3", "kg2", "s2", "L2", "m-2", "s-1", "kg-1",
        
        // Annotations
        "{count}", "{count}/min", "{beats}/min", "{cells}", "{cells}/uL",
        
        // Square brackets
        "[IU]", "[iU]", "[AU]", "[CFU]", "[IU]/mL", "[iU]/L",
        
        // Complex combinations
        "kg.m2/s3", "N.m/s", "Pa.s", "J/(kg.K)", "W/(m2.K)", "mol/(L.s)",
        
        // With annotations
        "U{37Cel}/L", "U{25Cel}/L", "g{creat}", "g{Hb}", "ug{FEU}/mL",
        
        // With square brackets
        "cm[Hg]", "cm[H2O]", "m[Hg]", "in[H2O]",
        
        // Complex medical units
        "U/(10.g){feces}", "U/g{creat}", "U/g{Hb}", "U/g{protein}",
        "ug/g{creat}", "ug/g{Hb}", "ug/g{tissue}", "umol/g{creat}",
        
        // Time-based units
        "L/(24.h)", "g/(12.h)", "U/(2.h)", "ug/(8.h)", "umol/(24.h)",
        
        // Per area/volume
        "L/min/m2", "g/m2", "ug/m2", "U/m2", "cells/m2",
        
        // Per weight
        "g/kg", "ug/kg", "U/kg", "umol/kg", "mg/kg",
        
        // Per concentration
        "U/L", "ug/L", "umol/L", "g/L", "mg/L",
    ];
    
    let mut failed_cases = Vec::new();
    
    for case in &test_cases {
        match parser.parse(case) {
            Ok(_) => {
                // Success
            }
            Err(e) => {
                failed_cases.push((case.to_string(), e.to_string()));
            }
        }
    }
    
    if !failed_cases.is_empty() {
        println!("\nFailed specific test cases:");
        for (expr, error) in &failed_cases {
            println!("  '{}' -> {}", expr, error);
        }
    }
    
    // Assert that all basic cases should parse
    assert!(
        failed_cases.is_empty(),
        "{} out of {} basic UCUM expressions failed to parse: {:?}",
        failed_cases.len(),
        test_cases.len(),
        failed_cases
    );
    
    println!("✅ All specific UCUM expressions parsed successfully!");
}

#[test]
fn test_ucum_expression_evaluation() {
    let parser = UcumParser::new();
    
    // Test expressions that should both parse and evaluate
    let test_cases = vec![
        ("kg", "kilogram"),
        ("m", "meter"),
        ("s", "second"),
        ("L", "liter"),
        ("g", "gram"),
        ("mol", "mole"),
        ("K", "degree Kelvin"),
        ("A", "ampere"),
        ("V", "volt"),
        ("W", "watt"),
        ("J", "joule"),
        ("N", "newton"),
        ("Pa", "pascal"),
        ("Hz", "Hertz"),
    ];
    
    for (expr, expected_name) in test_cases {
        match parser.parse_and_evaluate(expr) {
            Ok(unit) => {
                assert_eq!(
                    unit.name, expected_name,
                    "Expected unit name '{}' for '{}', got '{}'",
                    expected_name, expr, unit.name
                );
            }
            Err(e) => {
                panic!("Failed to parse and evaluate '{}': {}", expr, e);
            }
        }
    }
    
    println!("✅ All UCUM expression evaluations successful!");
} 