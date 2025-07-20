use ucum_core::{UcumParser, UcumRegistry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("UCUM Core Library - Basic Usage Example");
    println!("=======================================\n");

    // Create a parser
    let parser = UcumParser::new();
    
    // Example UCUM expressions
    let expressions = vec![
        "mg",
        "kg/m2",
        "cm3",
        "mmol/L",
        "g/(kg.d)",
        "m{IOU}/L",
    ];
    
    for expr in expressions {
        println!("Parsing: {}", expr);
        match parser.parse(expr) {
            Ok(result) => println!("  Result: {:?}", result),
            Err(e) => println!("  Error: {}", e),
        }
        println!();
    }
    
    // Example with registry
    let registry = UcumRegistry::new().unwrap_or_default();
    println!("Registry contains {} units", registry.len());
    
    Ok(())
} 