use ucum_core::{FP_Quantity, UcumParser, UcumRegistry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("UCUM Core Library - FHIRPath Quantity Operations Example");
    println!("=======================================================\n");

    // Create a parser and registry
    let registry = UcumRegistry::new().unwrap_or_default();
    let parser = UcumParser::new();
    
    // Create quantities with UCUM parser
    let weight1 = FP_Quantity::new(70.0, "kg".to_string())
        .with_ucum_parser(parser.clone_grammar_parser());
    let weight2 = FP_Quantity::new(5.0, "kg".to_string())
        .with_ucum_parser(parser.clone_grammar_parser());
    let height = FP_Quantity::new(1.75, "m".to_string())
        .with_ucum_parser(parser.clone_grammar_parser());
    
    println!("Created quantities:");
    println!("  Weight 1: {}", weight1);
    println!("  Weight 2: {}", weight2);
    println!("  Height: {}", height);
    println!();
    
    // Validate units
    println!("Unit validation:");
    println!("  Weight 1 unit valid: {}", weight1.validate_unit().is_ok());
    println!("  Height unit valid: {}", height.validate_unit().is_ok());
    println!();
    
    // Quantity operations
    println!("Quantity operations:");
    
    // Addition
    match weight1.plus(&weight2) {
        Ok(result) => println!("  {} + {} = {}", weight1, weight2, result),
        Err(e) => println!("  Addition error: {}", e),
    }
    
    // Multiplication
    match weight1.multiply(&height) {
        Ok(result) => println!("  {} * {} = {}", weight1, height, result),
        Err(e) => println!("  Multiplication error: {}", e),
    }
    
    // Division
    match weight1.divide(&height) {
        Ok(result) => println!("  {} / {} = {}", weight1, height, result),
        Err(e) => println!("  Division error: {}", e),
    }
    println!();
    
    // Comparison
    println!("Quantity comparison:");
    match weight1.compare(&weight2) {
        Some(ordering) => {
            let comparison = match ordering {
                std::cmp::Ordering::Less => "less than",
                std::cmp::Ordering::Equal => "equal to",
                std::cmp::Ordering::Greater => "greater than",
            };
            println!("  {} is {} {}", weight1, comparison, weight2);
        }
        None => println!("  Cannot compare {} and {} (different units)", weight1, weight2),
    }
    println!();
    
    // Unit conversion
    println!("Unit conversion:");
    let temperature = FP_Quantity::new(25.0, "Cel".to_string())
        .with_ucum_parser(parser.clone_grammar_parser());
    
    match temperature.convert_to("Cel") {
        Ok(converted) => println!("  {} converted to Cel = {}", temperature, converted),
        Err(e) => println!("  Conversion error: {}", e),
    }
    println!();
    
    // Complex unit expressions
    println!("Complex unit expressions:");
    let complex_quantity = FP_Quantity::new(100.0, "kg/m2".to_string())
        .with_ucum_parser(parser.clone_grammar_parser());
    
    println!("  Complex quantity: {}", complex_quantity);
    println!("  Unit valid: {}", complex_quantity.validate_unit().is_ok());
    
    match complex_quantity.get_parsed_unit() {
        Ok(parsed) => println!("  Parsed unit structure: {:?}", parsed),
        Err(e) => println!("  Parse error: {}", e),
    }
    
    Ok(())
} 