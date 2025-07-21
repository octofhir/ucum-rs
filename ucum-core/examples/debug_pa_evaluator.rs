use octofhir_ucum_core::{parse_expression, evaluate, find_unit, find_prefix};

fn main() {
    println!("Debugging Pa evaluation in detail...");

    // Check what find_unit returns for Pa
    match find_unit("Pa") {
        Some(unit) => {
            println!("find_unit('Pa') = {:?}", unit);
        }
        None => {
            println!("find_unit('Pa') = None");
        }
    }

    // Check if Pa is being treated as a prefixed unit
    match find_prefix("P") {
        Some(prefix) => {
            println!("find_prefix('P') = {:?}", prefix);
        }
        None => {
            println!("find_prefix('P') = None");
        }
    }

    // Check if 'a' is a valid unit
    match find_unit("a") {
        Some(unit) => {
            println!("find_unit('a') = {:?}", unit);
        }
        None => {
            println!("find_unit('a') = None");
        }
    }

    // Parse and evaluate Pa step by step
    println!("\nParsing 'Pa'...");
    match parse_expression("Pa") {
        Ok(expr) => {
            println!("Parsed expression: {:?}", expr);

            println!("Evaluating...");
            match evaluate(&expr) {
                Ok(result) => {
                    println!("Evaluation result: factor={}, dim={:?}, offset={}",
                             result.factor, result.dim, result.offset);
                }
                Err(e) => {
                    println!("Evaluation error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }

    // Also test a simple unit that should work correctly
    println!("\nTesting 'g' for comparison...");
    match parse_expression("g") {
        Ok(expr) => {
            println!("Parsed 'g': {:?}", expr);
            match evaluate(&expr) {
                Ok(result) => {
                    println!("'g' result: factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => {
                    println!("'g' error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("'g' parse error: {}", e);
        }
    }
}
