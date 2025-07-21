use octofhir_ucum_core::{parse_expression, evaluate, find_unit, find_prefix, UnitExpr, UnitFactor};

fn main() {
    println!("Debugging dNp (decineper) evaluation...");

    // Check what find_unit returns for Np
    match find_unit("Np") {
        Some(unit) => {
            println!("find_unit('Np') = {:?}", unit);
        }
        None => {
            println!("find_unit('Np') = None");
        }
    }

    // Check what find_unit returns for dNp
    match find_unit("dNp") {
        Some(unit) => {
            println!("find_unit('dNp') = {:?}", unit);
        }
        None => {
            println!("find_unit('dNp') = None");
        }
    }

    // Check if 'd' is a valid prefix
    match find_prefix("d") {
        Some(prefix) => {
            println!("find_prefix('d') = {:?}", prefix);
        }
        None => {
            println!("find_prefix('d') = None");
        }
    }

    // Test evaluating just "Np"
    println!("\nTesting 'Np':");
    match parse_expression("Np") {
        Ok(expr) => {
            println!("Parsed 'Np': {:?}", expr);
            match evaluate(&expr) {
                Ok(result) => {
                    println!("'Np' result: factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => {
                    println!("'Np' error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("'Np' parse error: {}", e);
        }
    }

    // Test evaluating "dNp"
    println!("\nTesting 'dNp':");
    match parse_expression("dNp") {
        Ok(expr) => {
            println!("Parsed 'dNp': {:?}", expr);
            match evaluate(&expr) {
                Ok(result) => {
                    println!("'dNp' result: factor={}, dim={:?}", result.factor, result.dim);
                }
                Err(e) => {
                    println!("'dNp' error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("'dNp' parse error: {}", e);
        }
    }

    // Test the actual failing case: 10 dNp
    println!("\nTesting '10 dNp' (the failing test case):");
    let expr = UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(10.0),
            exponent: 1,
        },
        UnitFactor {
            expr: UnitExpr::Symbol("dNp".into()),
            exponent: 1,
        },
    ]);

    println!("Expression: {:?}", expr);
    match evaluate(&expr) {
        Ok(result) => {
            println!("'10 dNp' result: factor={}", result.factor);
            println!("Expected: {} (e^1)", std::f64::consts::E);
            println!("Ratio: {}", result.factor / std::f64::consts::E);
        }
        Err(e) => {
            println!("'10 dNp' error: {}", e);
        }
    }
}
