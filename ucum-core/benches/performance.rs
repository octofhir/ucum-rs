//! Performance benchmarks for Phase 4: Performance and Scalability
//!
//! These benchmarks test the performance improvements from Phase 4:
//! - Evaluation cache effectiveness
//! - Optimized registry access
//! - Prefix trie performance
//! - Overall conversion performance

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use octofhir_ucum_core::{
    parse_expression, evaluate, find_unit, find_unit_optimized, find_prefix_optimized,
    find_prefixes_with_trie, get_cache_stats, clear_global_cache, convert_with_context,
    AdvancedConversionContext, DecimalPrecision, RoundingMode, TemperatureScale,
};

/// Benchmark expression parsing performance
fn bench_parsing(c: &mut Criterion) {
    let expressions = vec![
        "kg",
        "mg/dL", 
        "kg.m/s2",
        "kg.m2/s3",
        "mm[Hg]",
        "10*3.mol/L",
        "cal_IT/g",
        "[degF]",
        "Cel",
        "N.m",
    ];

    c.bench_function("parse_simple_expressions", |b| {
        b.iter(|| {
            for expr in &expressions {
                let _ = black_box(parse_expression(expr));
            }
        })
    });

    c.bench_function("parse_complex_expression", |b| {
        b.iter(|| {
            let _ = black_box(parse_expression("4.[pi].10*-7.N/A2"));
        })
    });
}

/// Benchmark evaluation performance with and without caching
fn bench_evaluation(c: &mut Criterion) {
    let expressions: Vec<_> = vec![
        "kg", "mg", "g", "t", "oz",
        "m", "cm", "mm", "km", "ft",
        "s", "min", "h", "d", "a",
        "A", "mA", "uA", "kA",
        "K", "Cel", "[degF]",
    ].into_iter()
    .map(|expr| parse_expression(expr).unwrap())
    .collect();

    // Clear cache before benchmarking
    let _ = clear_global_cache();

    c.bench_function("evaluate_without_cache", |b| {
        let _ = clear_global_cache(); // Ensure clean state
        b.iter(|| {
            for expr in &expressions {
                let _ = black_box(evaluate(expr));
            }
        })
    });

    c.bench_function("evaluate_with_cache", |b| {
        // Warm up the cache
        for expr in &expressions {
            let _ = evaluate(expr);
        }
        
        b.iter(|| {
            for expr in &expressions {
                let _ = black_box(evaluate(expr));
            }
        })
    });
}

/// Benchmark unit lookup performance: linear vs optimized
fn bench_unit_lookup(c: &mut Criterion) {
    let units = vec![
        "kg", "mg", "g", "t", "oz", "lb",
        "m", "cm", "mm", "km", "ft", "in",
        "s", "min", "h", "d", "a", "mo",
        "A", "mA", "uA", "kA", "C", "F",
        "K", "Cel", "[degF]", "mol", "L",
    ];

    c.bench_function("unit_lookup_linear", |b| {
        b.iter(|| {
            for unit in &units {
                let _ = black_box(find_unit(unit));
            }
        })
    });

    c.bench_function("unit_lookup_optimized", |b| {
        b.iter(|| {
            for unit in &units {
                let _ = black_box(find_unit_optimized(unit));
            }
        })
    });
}

/// Benchmark prefix lookup and trie performance
fn bench_prefix_operations(c: &mut Criterion) {
    let prefixed_units = vec![
        "kg", "mg", "ug", "ng", "pg",
        "km", "cm", "mm", "um", "nm",
        "kA", "mA", "uA", "nA", "pA",
        "GHz", "MHz", "kHz", "mHz",
        "GPa", "MPa", "kPa", "mPa",
    ];

    c.bench_function("prefix_lookup_optimized", |b| {
        b.iter(|| {
            for unit in &prefixed_units {
                // Extract potential prefix
                for len in 1..=3 {
                    if len < unit.len() {
                        let prefix_part = &unit[..len];
                        let _ = black_box(find_prefix_optimized(prefix_part));
                    }
                }
            }
        })
    });

    c.bench_function("prefix_trie_lookup", |b| {
        b.iter(|| {
            for unit in &prefixed_units {
                let _ = black_box(find_prefixes_with_trie(unit));
            }
        })
    });
}

/// Benchmark conversion performance with caching
fn bench_conversions(c: &mut Criterion) {
    let conversions = vec![
        ("kg", "g"),
        ("mg", "ug"),
        ("km", "m"),
        ("cm", "mm"),
        ("h", "min"),
        ("Cel", "K"),
        ("[degF]", "Cel"),
        ("L", "mL"),
        ("kPa", "mmHg"),
        ("J", "cal"),
    ];

    let context = AdvancedConversionContext::default();

    c.bench_function("convert_without_cache", |b| {
        let _ = clear_global_cache(); // Clear cache before each run
        b.iter(|| {
            for (from, to) in &conversions {
                let _ = black_box(convert_with_context(1.0, from, to, &context));
            }
        })
    });

    c.bench_function("convert_with_cache", |b| {
        // Warm up cache
        for (from, to) in &conversions {
            let _ = convert_with_context(1.0, from, to, &context);
        }
        
        b.iter(|| {
            for (from, to) in &conversions {
                let _ = black_box(convert_with_context(1.0, from, to, &context));
            }
        })
    });
}

/// Benchmark cache performance under different load patterns
fn bench_cache_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_load_patterns");
    
    // Test different cache sizes
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("expressions", size), size, |b, &size| {
            let _ = clear_global_cache();
            
            // Generate expressions
            let expressions: Vec<_> = (0..size)
                .map(|i| parse_expression(&format!("{}*kg", i + 1)).unwrap())
                .collect();
            
            b.iter(|| {
                for expr in &expressions {
                    let _ = black_box(evaluate(expr));
                }
            });
        });
    }
    
    group.finish();
}

/// Benchmark real-world usage patterns
fn bench_real_world_patterns(c: &mut Criterion) {
    let context = AdvancedConversionContext::default();

    // Medical units pattern
    c.bench_function("medical_units_pattern", |b| {
        let medical_conversions = vec![
            ("mg/dL", "mmol/L"),
            ("g/dL", "g/L"),
            ("mL/min", "L/h"),
            ("mmHg", "kPa"),
            ("Cel", "[degF]"),
            ("kg/m2", "lb/ft2"),
        ];
        
        b.iter(|| {
            for (from, to) in &medical_conversions {
                let _ = black_box(convert_with_context(100.0, from, to, &context));
            }
        });
    });

    // Engineering units pattern
    c.bench_function("engineering_units_pattern", |b| {
        let engineering_conversions = vec![
            ("N.m", "J"),
            ("kg.m/s2", "N"),
            ("Pa", "psi"),
            ("W", "hp"),
            ("m/s", "km/h"),
            ("J/kg.K", "Btu/lb.R"),
        ];
        
        b.iter(|| {
            for (from, to) in &engineering_conversions {
                let _ = black_box(convert_with_context(1.0, from, to, &context));
            }
        });
    });
}

/// Display cache statistics after benchmarks
fn display_cache_stats() {
    if let Ok(stats) = get_cache_stats() {
        println!("\n=== Cache Performance Statistics ===");
        println!("Expression hit ratio: {:.2}%", stats.expression_hit_ratio() * 100.0);
        println!("Conversion hit ratio: {:.2}%", stats.conversion_hit_ratio() * 100.0);
        println!("Overall hit ratio: {:.2}%", stats.overall_hit_ratio() * 100.0);
        println!("Expression hits: {}, misses: {}", stats.expression_hits, stats.expression_misses);
        println!("Conversion hits: {}, misses: {}", stats.conversion_hits, stats.conversion_misses);
        println!("Dimension hits: {}, misses: {}", stats.dimension_hits, stats.dimension_misses);
    }
}

criterion_group!(
    benches,
    bench_parsing,
    bench_evaluation,
    bench_unit_lookup,
    bench_prefix_operations,
    bench_conversions,
    bench_cache_performance,
    bench_real_world_patterns,
);

criterion_main!(benches);

#[cfg(test)]
mod benchmark_validation {
    use super::*;

    #[test]
    fn validate_benchmark_data() {
        // Ensure all benchmark expressions are valid
        let expressions = vec![
            "kg", "mg/dL", "kg.m/s2", "kg.m2/s3", "mm[Hg]",
            "10*3.mol/L", "cal_IT/g", "[degF]", "Cel", "N.m",
        ];

        for expr in expressions {
            parse_expression(expr).expect(&format!("Invalid expression: {}", expr));
        }

        // Ensure conversions work
        let conversions = vec![
            ("kg", "g"), ("mg", "ug"), ("km", "m"), ("Cel", "K"),
        ];

        let context = AdvancedConversionContext::default();

        for (from, to) in conversions {
            convert_with_context(1.0, from, to, &context).expect(&format!("Invalid conversion: {} -> {}", from, to));
        }
    }

    #[test]
    fn validate_cache_functionality() {
        // Clear cache
        clear_global_cache().expect("Failed to clear cache");

        // Perform some operations
        let expr = parse_expression("kg").unwrap();
        let _ = evaluate(&expr);
        
        let context = AdvancedConversionContext::default();
        let _ = convert_with_context(1.0, "kg", "g", &context);

        // Check that cache has some hits
        let stats = get_cache_stats().expect("Failed to get cache stats");
        assert!(stats.expression_hits > 0 || stats.conversion_hits > 0);
    }
}