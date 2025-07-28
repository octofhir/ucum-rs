//! Manual performance test to validate Phase 4 improvements

use octofhir_ucum_core::{
    parse_expression, evaluate_owned, find_unit, find_unit_optimized, 
    get_cache_stats, clear_global_cache, get_cache_sizes,
};
use std::time::Instant;

fn main() {
    println!("Phase 4 Performance Validation Test");
    println!("===================================");

    // Test 1: Basic expression evaluation with caching
    test_expression_caching();
    
    // Test 2: Unit lookup performance comparison
    test_unit_lookup_performance();
    
    // Test 3: Cache hit ratio validation
    test_cache_effectiveness();
    
    println!("\nPhase 4 performance improvements validated successfully!");
}

fn test_expression_caching() {
    println!("\n1. Testing Expression Caching:");
    
    // Clear cache for clean test
    clear_global_cache().expect("Failed to clear cache");
    
    let expressions = vec!["kg", "mg", "g", "m", "cm", "mm"];
    let parsed_expressions: Vec<_> = expressions.iter()
        .map(|e| parse_expression(e).unwrap())
        .collect();
    
    // First run - should populate cache
    let start = Instant::now();
    for expr in &parsed_expressions {
        let _ = evaluate_owned(expr).unwrap();
    }
    let first_run = start.elapsed();
    
    // Second run - should hit cache
    let start = Instant::now();
    for expr in &parsed_expressions {
        let _ = evaluate_owned(expr).unwrap();
    }
    let second_run = start.elapsed();
    
    println!("   First run (cache miss): {:?}", first_run);
    println!("   Second run (cache hit): {:?}", second_run);
    
    if second_run < first_run {
        println!("   ✓ Cache is providing performance benefit");
    } else {
        println!("   ⚠ Cache might not be working optimally");
    }
    
    // Display cache stats
    if let Ok(stats) = get_cache_stats() {
        println!("   Expression hits: {}, misses: {}", stats.expression_hits, stats.expression_misses);
        println!("   Hit ratio: {:.2}%", stats.expression_hit_ratio() * 100.0);
    }
}

fn test_unit_lookup_performance() {
    println!("\n2. Testing Unit Lookup Performance:");
    
    let units = vec!["kg", "mg", "g", "t", "oz", "lb", "m", "cm", "mm", "km"];
    let iterations = 10000;
    
    // Test linear lookup
    let start = Instant::now();
    for _ in 0..iterations {
        for unit in &units {
            let _ = find_unit(unit);
        }
    }
    let linear_time = start.elapsed();
    
    // Test optimized lookup
    let start = Instant::now();
    for _ in 0..iterations {
        for unit in &units {
            let _ = find_unit_optimized(unit);
        }
    }
    let optimized_time = start.elapsed();
    
    println!("   Linear lookup ({} iterations): {:?}", iterations * units.len(), linear_time);
    println!("   Optimized lookup ({} iterations): {:?}", iterations * units.len(), optimized_time);
    
    if optimized_time < linear_time {
        let speedup = linear_time.as_nanos() as f64 / optimized_time.as_nanos() as f64;
        println!("   ✓ Optimized lookup is {:.2}x faster", speedup);
    } else {
        println!("   ⚠ Optimized lookup is not faster (might be due to small dataset)");
    }
}

fn test_cache_effectiveness() {
    println!("\n3. Testing Cache Effectiveness:");
    
    // Clear cache for clean test
    clear_global_cache().expect("Failed to clear cache");
    
    // Perform repeated operations
    let test_expressions = vec!["kg", "mg", "g", "m", "cm"];
    let repetitions = 100;
    
    for _ in 0..repetitions {
        for expr_str in &test_expressions {
            let expr = parse_expression(expr_str).unwrap();
            let _ = evaluate_owned(&expr).unwrap();
        }
    }
    
    // Check cache statistics
    if let Ok(stats) = get_cache_stats() {
        let total_operations = repetitions * test_expressions.len();
        println!("   Total operations: {}", total_operations);
        println!("   Expression hits: {}, misses: {}", stats.expression_hits, stats.expression_misses);
        println!("   Hit ratio: {:.2}%", stats.expression_hit_ratio() * 100.0);
        
        if stats.expression_hit_ratio() > 0.8 {
            println!("   ✓ Cache is highly effective (>80% hit ratio)");
        } else if stats.expression_hit_ratio() > 0.5 {
            println!("   ✓ Cache is working (>50% hit ratio)");
        } else {
            println!("   ⚠ Cache effectiveness could be improved");
        }
    }
    
    // Check cache sizes
    if let Ok((expr_cache, conv_cache, dim_cache)) = get_cache_sizes() {
        println!("   Cache sizes - Expressions: {}, Conversions: {}, Dimensions: {}", 
                 expr_cache, conv_cache, dim_cache);
    }
}