use octofhir_ucum_core::{parse_expression, analyse, get_canonical_units, optimize_expression, canonicalize_expression, simplify_expression};

fn main() {
    println!("Testing basic functions...");
    
    // Test basic parsing and analysis
    let test_expr = "m";
    println!("Testing expression: {}", test_expr);
    
    match parse_expression(test_expr) {
        Ok(parsed) => println!("Parsed: {:?}", parsed),
        Err(e) => println!("Parse error: {}", e),
    }
    
    println!("Calling analyse...");
    match analyse(test_expr) {
        Ok(analysis) => {
            println!("Analysis successful!");
            println!("Dimension: {:?}", analysis.dimension);
            println!("Factor: {}", analysis.factor);
        },
        Err(e) => println!("Analysis error: {}", e),
    }
    
    println!("Testing get_canonical_units..."); 
    match get_canonical_units(test_expr) {
        Ok(canonical) => {
            println!("Canonical successful!");
            println!("Unit: {}", canonical.unit);
            println!("Factor: {}", canonical.factor);
        },
        Err(e) => println!("Canonical error: {}", e),
    }
    
    println!("Testing optimize_expression...");
    match optimize_expression(test_expr) {
        Ok(optimized) => println!("Optimized: {}", optimized),
        Err(e) => println!("Optimization error: {}", e),
    }
    
    println!("Testing canonicalize_expression...");
    match canonicalize_expression(test_expr) {
        Ok(canonical) => println!("Canonical: {}", canonical),
        Err(e) => println!("Canonicalization error: {}", e),
    }
    
    println!("Testing simplify_expression...");
    match simplify_expression(test_expr) {
        Ok(simplified) => println!("Simplified: {}", simplified),
        Err(e) => println!("Simplification error: {}", e),
    }
    
    println!("Done!");
}