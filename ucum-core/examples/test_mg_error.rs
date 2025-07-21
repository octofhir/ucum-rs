use octofhir_ucum_core::{parse_expression, evaluate, find_unit, find_prefix};

fn main() {
    println!("Testing 'mg' unit recognition...");

    match parse_expression("mg") {
        Ok(expr) => {
            println!("✓ Successfully parsed 'mg': {:?}", expr);

            // Try to evaluate it
            match evaluate(&expr) {
                Ok(result) => {
                    println!("✓ Successfully evaluated 'mg': factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => {
                    println!("✗ Failed to evaluate 'mg': {}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to parse 'mg': {}", e);
        }
    }

    // Also test if we can find the unit directly
    match find_unit("mg") {
        Some(unit) => {
            println!("✓ Found 'mg' unit directly: {:?}", unit);
        }
        None => {
            println!("✗ Could not find 'mg' unit directly");
        }
    }

    // Test if we can find the base unit 'g'
    match find_unit("g") {
        Some(unit) => {
            println!("✓ Found 'g' unit: {:?}", unit);
        }
        None => {
            println!("✗ Could not find 'g' unit");
        }
    }

    // Test if we can find the prefix 'm'
    match find_prefix("m") {
        Some(prefix) => {
            println!("✓ Found 'm' prefix: {:?}", prefix);
        }
        None => {
            println!("✗ Could not find 'm' prefix");
        }
    }
}
