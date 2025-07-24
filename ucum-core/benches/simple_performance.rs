//! Simple performance test to validate Phase 4 improvements

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use octofhir_ucum_core::{
    parse_expression, evaluate, find_unit, find_unit_optimized, get_cache_stats, clear_global_cache,
};

/// Simple benchmark for expression parsing
fn bench_simple_parsing(c: &mut Criterion) {
    c.bench_function("parse_kg", |b| {
        b.iter(|| {
            let _ = black_box(parse_expression("kg"));
        })
    });
}

/// Simple benchmark for evaluation
fn bench_simple_evaluation(c: &mut Criterion) {
    let expr = parse_expression("kg").unwrap();
    
    c.bench_function("evaluate_kg", |b| {
        b.iter(|| {
            let _ = black_box(evaluate(&expr));
        })
    });
}

/// Simple benchmark for unit lookup comparison
fn bench_unit_lookup_comparison(c: &mut Criterion) {
    c.bench_function("unit_lookup_linear", |b| {
        b.iter(|| {
            let _ = black_box(find_unit("kg"));
        })
    });

    c.bench_function("unit_lookup_optimized", |b| {
        b.iter(|| {
            let _ = black_box(find_unit_optimized("kg"));
        })
    });
}

/// Display cache stats
fn display_cache_stats() {
    if let Ok(stats) = get_cache_stats() {
        println!("\n=== Cache Performance Statistics ===");
        println!("Expression hit ratio: {:.2}%", stats.expression_hit_ratio() * 100.0);
        println!("Conversion hit ratio: {:.2}%", stats.conversion_hit_ratio() * 100.0);
        println!("Overall hit ratio: {:.2}%", stats.overall_hit_ratio() * 100.0);
    }
}

criterion_group!(
    benches,
    bench_simple_parsing,
    bench_simple_evaluation,
    bench_unit_lookup_comparison,
);

criterion_main!(benches);

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Test parsing works
        let expr = parse_expression("kg").unwrap();
        
        // Test evaluation works
        let _ = evaluate(&expr).unwrap();
        
        // Test cache functionality
        let _ = clear_global_cache();
        let stats = get_cache_stats().unwrap();
        println!("Cache stats: {:?}", stats);
    }
}