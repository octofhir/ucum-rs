use octofhir_ucum_core::{parse_expression, evaluate};

fn main() {
    println!("Debugging kg->g conversion issue (simplified)...");

    // Test the specific kg->g conversion
    let src_unit = "1.25 kg";
    let dst_unit = "g";
    let expected = 1250.0;

    println!("\n=== Testing conversion: {} -> {} (expected: {}) ===", src_unit, dst_unit, expected);

    // Parse and evaluate source unit
    match parse_expression(src_unit) {
        Ok(src_expr) => {
            println!("Source parsed successfully");

            match evaluate(&src_expr) {
                Ok(src_result) => {
                    println!("Source evaluated: factor={}, dim={:?}", src_result.factor, src_result.dim);

                    // Parse and evaluate target unit
                    match parse_expression(dst_unit) {
                        Ok(dst_expr) => {
                            println!("Destination parsed successfully");
                            match evaluate(&dst_expr) {
                                Ok(dst_result) => {
                                    println!("Destination evaluated: factor={}, dim={:?}", dst_result.factor, dst_result.dim);

                                    // Check commensurability
                                    if src_result.dim == dst_result.dim {
                                        // Calculate conversion
                                        let conversion_factor = src_result.factor / dst_result.factor;
                                        println!("✓ Commensurable! Conversion factor: {}", conversion_factor);

                                        // Compare with expected result
                                        let diff = (conversion_factor - expected).abs();
                                        let rel_diff = diff / expected.abs().max(1e-10);
                                        println!("Expected: {}, Got: {}, Absolute diff: {}, Relative diff: {}",
                                                 expected, conversion_factor, diff, rel_diff);
                                    } else {
                                        println!("✗ Not commensurable: {:?} vs {:?}", src_result.dim, dst_result.dim);
                                    }
                                }
                                Err(e) => println!("Destination evaluation error: {:?}", e),
                            }
                        }
                        Err(e) => println!("Destination parse error: {:?}", e),
                    }
                }
                Err(e) => println!("Source evaluation error: {:?}", e),
            }
        }
        Err(e) => println!("Source parse error: {:?}", e),
    }

    // Also test individual components
    println!("\n=== Testing individual components ===");

    // Test "kg" by itself
    let kg_unit = "kg";
    println!("\nTesting unit: {}", kg_unit);
    match parse_expression(kg_unit) {
        Ok(expr) => {
            println!("Parsed successfully");
            match evaluate(&expr) {
                Ok(result) => {
                    println!("Evaluated: factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => println!("Evaluation error: {:?}", e),
            }
        }
        Err(e) => println!("Parse error: {:?}", e),
    }

    // Test "g" by itself
    let g_unit = "g";
    println!("\nTesting unit: {}", g_unit);
    match parse_expression(g_unit) {
        Ok(expr) => {
            println!("Parsed successfully");
            match evaluate(&expr) {
                Ok(result) => {
                    println!("Evaluated: factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => println!("Evaluation error: {:?}", e),
            }
        }
        Err(e) => println!("Parse error: {:?}", e),
    }

    // Test "k" prefix by itself (should fail, but let's see)
    let k_prefix = "k";
    println!("\nTesting prefix: {}", k_prefix);
    match parse_expression(k_prefix) {
        Ok(expr) => {
            println!("Parsed successfully");
            match evaluate(&expr) {
                Ok(result) => {
                    println!("Evaluated: factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => println!("Evaluation error: {:?}", e),
            }
        }
        Err(e) => println!("Parse error: {:?}", e),
    }

    // Test numeric value by itself
    let numeric = "1.25";
    println!("\nTesting numeric: {}", numeric);
    match parse_expression(numeric) {
        Ok(expr) => {
            println!("Parsed successfully");
            match evaluate(&expr) {
                Ok(result) => {
                    println!("Evaluated: factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => println!("Evaluation error: {:?}", e),
            }
        }
        Err(e) => println!("Parse error: {:?}", e),
    }

    // Test "1 kg" (without decimal)
    let one_kg = "1 kg";
    println!("\nTesting: {}", one_kg);
    match parse_expression(one_kg) {
        Ok(expr) => {
            println!("Parsed successfully");
            match evaluate(&expr) {
                Ok(result) => {
                    println!("Evaluated: factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => println!("Evaluation error: {:?}", e),
            }
        }
        Err(e) => println!("Parse error: {:?}", e),
    }

    // Test "1000 g" (equivalent to 1 kg)
    let thousand_g = "1000 g";
    println!("\nTesting: {}", thousand_g);
    match parse_expression(thousand_g) {
        Ok(expr) => {
            println!("Parsed successfully");
            match evaluate(&expr) {
                Ok(result) => {
                    println!("Evaluated: factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => println!("Evaluation error: {:?}", e),
            }
        }
        Err(e) => println!("Parse error: {:?}", e),
    }
}
