use octofhir_ucum_core::{parse_expression, evaluate, find_unit, find_prefix};

fn main() {
    println!("Testing Pa and kPa unit evaluation...");

    // Test Pa
    match parse_expression("Pa") {
        Ok(expr) => {
            println!("✓ Successfully parsed 'Pa': {:?}", expr);
            match evaluate(&expr) {
                Ok(result) => {
                    println!("✓ Successfully evaluated 'Pa': factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => {
                    println!("✗ Failed to evaluate 'Pa': {}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to parse 'Pa': {}", e);
        }
    }

    // Test kPa
    match parse_expression("kPa") {
        Ok(expr) => {
            println!("✓ Successfully parsed 'kPa': {:?}", expr);
            match evaluate(&expr) {
                Ok(result) => {
                    println!("✓ Successfully evaluated 'kPa': factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => {
                    println!("✗ Failed to evaluate 'kPa': {}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to parse 'kPa': {}", e);
        }
    }

    // Test if we can find Pa unit directly
    match find_unit("Pa") {
        Some(unit) => {
            println!("✓ Found 'Pa' unit directly: {:?}", unit);
        }
        None => {
            println!("✗ Could not find 'Pa' unit directly");
        }
    }

    // Test if we can find kPa unit directly
    match find_unit("kPa") {
        Some(unit) => {
            println!("✓ Found 'kPa' unit directly: {:?}", unit);
        }
        None => {
            println!("✗ Could not find 'kPa' unit directly");
        }
    }

    // Test if we can find the prefix 'k'
    match find_prefix("k") {
        Some(prefix) => {
            println!("✓ Found 'k' prefix: {:?}", prefix);
        }
        None => {
            println!("✗ Could not find 'k' prefix");
        }
    }
}
