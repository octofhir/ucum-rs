use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use octofhir_ucum::parse_expression;
use std::hint::black_box;

/// Benchmark memory allocation patterns for different expression types
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    // Test expressions that trigger different allocation patterns
    let test_cases = [
        ("small_inline", "m"),
        ("unicode_normalization", "µg"),
        ("standalone_annotation", "{annotation}"),
        ("complex_factors", "kg.m.s.A.K.mol.cd"),
        ("deep_nesting", "((((m))))"),
        ("many_divisions", "m/s/kg/A/K/mol/cd"),
        ("leading_division", "/min"),
        ("ten_power", "10*3.mol"),
        ("mixed_operations", "kg.m2/s3/A"),
        ("bracket_units", "[in_i]"),
        ("celsius_temp", "Cel"),
        ("annotation_inline", "m{length}"),
    ];

    for (name, expr) in &test_cases {
        group.bench_with_input(BenchmarkId::new("parse", name), expr, |b, &expr| {
            b.iter(|| {
                let result = parse_expression(black_box(expr));
                black_box(result)
            })
        });
    }

    group.finish();
}

/// Benchmark batch parsing (simulates real-world usage patterns)
fn bench_batch_parsing(c: &mut Criterion) {
    // Realistic mix of units encountered in practice
    let expressions = vec![
        // SI base units
        "m",
        "kg",
        "s",
        "A",
        "K",
        "mol",
        "cd",
        // Common prefixed units
        "km",
        "mg",
        "µs",
        "kPa",
        "mL",
        "mm",
        "cm",
        "ng",
        "µg",
        // Medical/healthcare units
        "mg/dL",
        "mmol/L",
        "mEq/L",
        "U/L",
        "mm[Hg]",
        "beats/min",
        "/min",
        // Engineering units
        "kg.m/s2",
        "J/mol/K",
        "Pa.s",
        "N",
        "W",
        "V",
        "Ohm",
        "Hz",
        // Complex expressions
        "m2",
        "m3",
        "s-1",
        "kg/m3",
        "m2.kg/s3/A",
        // Special formats
        "10*3.mol/L",
        "10^-6.kg",
        "cal_IT/g",
        "[degF]",
        "Cel",
        // Edge cases
        "/min",
        "(kg.m)/s2",
        "m{length}",
        "{annotation}",
    ];

    c.bench_function("batch_parsing", |b| {
        b.iter(|| {
            for expr in &expressions {
                let _ = black_box(parse_expression(expr));
            }
        })
    });

    // Test parsing the same expressions multiple times (cache effects)
    c.bench_function("repeated_parsing", |b| {
        b.iter(|| {
            for _ in 0..10 {
                for expr in &expressions {
                    let _ = black_box(parse_expression(expr));
                }
            }
        })
    });
}

/// Benchmark pathological cases that stress the parser
fn bench_pathological_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathological");

    // Very long symbol names
    let long_symbol = format!("[{}]", "a".repeat(100));
    group.bench_function("long_symbol", |b| {
        b.iter(|| parse_expression(black_box(&long_symbol)))
    });

    // Many factors with products
    let many_factors = (0..50)
        .map(|i| format!("u{i}"))
        .collect::<Vec<_>>()
        .join(".");
    group.bench_function("many_factors", |b| {
        b.iter(|| parse_expression(black_box(&many_factors)))
    });

    // Many factors with divisions
    let many_divisions = (0..20)
        .map(|i| format!("u{i}"))
        .collect::<Vec<_>>()
        .join("/");
    group.bench_function("many_divisions", |b| {
        b.iter(|| parse_expression(black_box(&many_divisions)))
    });

    // Deep nesting
    let deep_nesting = format!("{}{}{}", "(".repeat(20), "m", ")".repeat(20));
    group.bench_function("deep_nesting", |b| {
        b.iter(|| parse_expression(black_box(&deep_nesting)))
    });

    // Complex annotations
    let complex_annotation = format!(
        "m{{{}}}",
        "complex annotation text with spaces and symbols!@#$%".repeat(5)
    );
    group.bench_function("complex_annotation", |b| {
        b.iter(|| parse_expression(black_box(&complex_annotation)))
    });

    // Many unicode characters
    let many_unicode = "µµµµµµµµµµg";
    group.bench_function("many_unicode", |b| {
        b.iter(|| parse_expression(black_box(many_unicode)))
    });

    // Very large exponents
    let large_exponent = "m999999";
    group.bench_function("large_exponent", |b| {
        b.iter(|| parse_expression(black_box(large_exponent)))
    });

    group.finish();
}

/// Benchmark zero-copy vs owned allocations
fn bench_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocations");

    // Test cases that should use zero-copy parsing
    let zero_copy_cases = ["m", "kg", "s", "kPa", "mol"];
    for case in &zero_copy_cases {
        group.bench_with_input(BenchmarkId::new("zero_copy", case), case, |b, &case| {
            b.iter(|| parse_expression(black_box(case)))
        });
    }

    // Test cases that require allocations (Unicode normalization)
    let owned_cases = ["µg", "µL", "µmol", "µs", "µA"];
    for case in &owned_cases {
        group.bench_with_input(BenchmarkId::new("owned", case), case, |b, &case| {
            b.iter(|| parse_expression(black_box(case)))
        });
    }

    // Test cases with annotations (require allocation)
    let annotation_cases = ["m{length}", "kg{mass}", "s{time}"];
    for case in &annotation_cases {
        group.bench_with_input(BenchmarkId::new("annotations", case), case, |b, &case| {
            b.iter(|| parse_expression(black_box(case)))
        });
    }

    group.finish();
}

criterion_group!(
    memory_benches,
    bench_memory_patterns,
    bench_batch_parsing,
    bench_pathological_cases,
    bench_allocation_patterns
);
criterion_main!(memory_benches);
