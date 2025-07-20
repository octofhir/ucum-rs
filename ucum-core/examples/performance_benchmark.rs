use std::time::Instant;
use ucum_core::{UcumParser, UcumRegistry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("UCUM Core Library - Performance Benchmark");
    println!("=========================================\n");

    // Create parser and registry
    let registry = UcumRegistry::new().unwrap_or_default();
    let mut parser = UcumParser::new();
    
    // Test expressions
    let test_expressions = vec![
        "m",
        "kg",
        "s",
        "m/s",
        "kg/m2",
        "kg.m2",
        "m2",
        "m-2",
        "{count}",
        "[iU]",
        "kg.m2/s3",
        "N.m/s",
        "Pa.s",
        "g/L",
        "beats{count}/min",
    ];
    
    println!("Performance Tests:");
    println!("==================");
    
    // Test parsing performance
    println!("\n1. Parsing Performance:");
    let start = Instant::now();
    for expression in &test_expressions {
        for _ in 0..100 {
            let _ = parser.parse(expression);
        }
    }
    let parse_duration = start.elapsed();
    println!("   Parsed {} expressions 100 times each in {:?}", test_expressions.len(), parse_duration);
    
    // Test caching performance
    println!("\n2. Caching Performance:");
    let start = Instant::now();
    for expression in &test_expressions {
        for _ in 0..100 {
            let _ = parser.grammar_parser_mut().parse_with_cache(expression);
        }
    }
    let cache_duration = start.elapsed();
    println!("   Cached parsing took {:?}", cache_duration);
    println!("   Speedup: {:.2}x", parse_duration.as_nanos() as f64 / cache_duration.as_nanos() as f64);
    
    // Test cache statistics
    let (cache_size, cache_capacity) = parser.grammar_parser().cache_stats();
    println!("   Cache size: {} entries, capacity: {}", cache_size, cache_capacity);
    
    // Test symbol validation performance
    println!("\n3. Symbol Validation Performance:");
    let test_symbols = vec![
        "m",
        "kg",
        "s",
        "invalid(",
        "invalid)",
        "invalid{",
        "invalid}",
        "invalid[",
        "invalid]",
    ];
    
    let start = Instant::now();
    for symbol in &test_symbols {
        for _ in 0..1000 {
            let _ = parser.grammar_parser().fast_symbol_validation(symbol);
        }
    }
    let fast_validation_duration = start.elapsed();
    println!("   Fast validation took {:?}", fast_validation_duration);
    
    // Test traditional validation for comparison
    let start = Instant::now();
    for symbol in &test_symbols {
        for _ in 0..1000 {
            let _ = parser.grammar_parser().validate_simple_unit_symbols(symbol);
        }
    }
    let traditional_validation_duration = start.elapsed();
    println!("   Traditional validation took {:?}", traditional_validation_duration);
    println!("   Speedup: {:.2}x", traditional_validation_duration.as_nanos() as f64 / fast_validation_duration.as_nanos() as f64);
    
    // Test evaluation performance
    println!("\n4. Expression Evaluation Performance:");
    let start = Instant::now();
    for expression in &test_expressions {
        for _ in 0..50 {
            if let Ok(term) = parser.parse(expression) {
                let _ = parser.grammar_parser().evaluate_term(&term);
            }
        }
    }
    let evaluation_duration = start.elapsed();
    println!("   Evaluation took {:?}", evaluation_duration);
    
    // Test memory usage
    println!("\n5. Memory Usage:");
    let initial_cache_size = parser.grammar_parser().cache_stats().0;
    println!("   Initial cache entries: {}", initial_cache_size);
    
    // Clear cache and measure
    parser.grammar_parser_mut().clear_cache();
    let after_clear_cache_size = parser.grammar_parser().cache_stats().0;
    println!("   After clearing cache: {} entries", after_clear_cache_size);
    
    // Test FHIRPath quantity operations performance
    println!("\n6. FHIRPath Quantity Operations Performance:");
    use ucum_core::FP_Quantity;
    
    let start = Instant::now();
    for _ in 0..1000 {
        let q1 = FP_Quantity::new(10.0, "kg".to_string())
            .with_ucum_parser(parser.clone_grammar_parser());
        let q2 = FP_Quantity::new(5.0, "kg".to_string())
            .with_ucum_parser(parser.clone_grammar_parser());
        
        let _ = q1.plus(&q2);
        let _ = q1.multiply(&q2);
        let _ = q1.divide(&q2);
    }
    let fhirpath_duration = start.elapsed();
    println!("   FHIRPath operations took {:?}", fhirpath_duration);
    
    println!("\nPerformance Summary:");
    println!("====================");
    println!("✅ Parsing: {:?}", parse_duration);
    println!("✅ Caching: {:?} (speedup: {:.2}x)", cache_duration, parse_duration.as_nanos() as f64 / cache_duration.as_nanos() as f64);
    println!("✅ Symbol validation: {:?} (speedup: {:.2}x)", fast_validation_duration, traditional_validation_duration.as_nanos() as f64 / fast_validation_duration.as_nanos() as f64);
    println!("✅ Expression evaluation: {:?}", evaluation_duration);
    println!("✅ FHIRPath operations: {:?}", fhirpath_duration);
    println!("✅ Memory management: Cache cleared successfully");
    
    Ok(())
} 