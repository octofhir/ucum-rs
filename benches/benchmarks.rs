use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use octofhir_ucum::{analyse, evaluate_owned, parse_expression, validate};
use std::hint::black_box;

/// Benchmark parsing performance across different expression complexities
fn bench_parsing_by_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_by_complexity");

    let test_cases = [
        ("simple_unit", "m"),
        ("prefixed_unit", "kg"),
        ("unicode_micro", "µs"),
        ("basic_product", "kg.m"),
        ("simple_quotient", "m/s"),
        ("complex_expression", "kg.m/s2"),
        ("medical_units", "mg/dL"),
        ("multiple_powers", "m2.kg/s3/A"),
        ("nested_quotient", "J/mol/K"),
        ("leading_division", "/min"),
        ("annotation", "m{length}"),
        ("ten_power", "10*3.mol"),
        ("parentheses", "(kg.m)/s2"),
        ("special_chars", "[in_i]"),
        ("celsius", "Cel"),
    ];

    for (name, expr) in &test_cases {
        group.bench_with_input(BenchmarkId::new("parse", name), expr, |b, &expr| {
            b.iter(|| parse_expression(black_box(expr)))
        });
    }

    group.finish();
}

/// Benchmark parsing performance for various unit categories
fn bench_parsing_categories(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_categories");

    // SI base units - fundamental units
    let base_units = ["m", "kg", "s", "A", "K", "mol", "cd"];
    for unit in &base_units {
        group.bench_with_input(BenchmarkId::new("base_units", unit), unit, |b, &unit| {
            b.iter(|| parse_expression(black_box(unit)))
        });
    }

    // Prefixed units - common metric prefixes
    let prefixed_units = ["km", "mg", "µs", "kPa", "mL", "cm", "mm", "ng"];
    for unit in &prefixed_units {
        group.bench_with_input(BenchmarkId::new("prefixed", unit), unit, |b, &unit| {
            b.iter(|| parse_expression(black_box(unit)))
        });
    }

    // Medical/FHIR units - healthcare contexts
    let medical_units = ["mg/dL", "mmol/L", "mEq/L", "U/L", "mm[Hg]", "beats/min"];
    for unit in &medical_units {
        group.bench_with_input(BenchmarkId::new("medical", unit), unit, |b, &unit| {
            b.iter(|| parse_expression(black_box(unit)))
        });
    }

    // Engineering units - technical applications
    let engineering_units = ["N", "Pa", "J", "W", "V", "Ohm", "Hz", "kPa"];
    for unit in &engineering_units {
        group.bench_with_input(BenchmarkId::new("engineering", unit), unit, |b, &unit| {
            b.iter(|| parse_expression(black_box(unit)))
        });
    }

    group.finish();
}

/// Benchmark evaluation performance
fn bench_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluation");

    let expressions = vec![
        ("simple", "kg"),
        ("prefixed", "mg"),
        ("compound", "kg.m/s2"),
        ("complex", "mg/dL"),
    ];

    for (name, expr) in expressions {
        let parsed = parse_expression(expr).unwrap();
        group.bench_function(name, |b| b.iter(|| evaluate_owned(black_box(&parsed))));
    }

    group.finish();
}

/// Benchmark high-level API functions
fn bench_api_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("api");

    // Validation
    group.bench_function("validate", |b| b.iter(|| validate(black_box("kg.m/s2"))));

    // Analysis
    group.bench_function("analyse", |b| b.iter(|| analyse(black_box("kg.m/s2"))));

    group.finish();
}

/// Benchmark parser-specific optimizations
fn bench_parser_features(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_features");

    // Unicode handling (µ normalization)
    let unicode_cases = ["µg", "µL", "µmol", "µs"];
    for unit in &unicode_cases {
        group.bench_with_input(BenchmarkId::new("unicode", unit), unit, |b, &unit| {
            b.iter(|| parse_expression(black_box(unit)))
        });
    }

    // Ten power expressions
    let ten_power_cases = ["10*3", "10^-2", "10*6.mol", "10^-12.kg"];
    for expr in &ten_power_cases {
        group.bench_with_input(BenchmarkId::new("ten_power", expr), expr, |b, &expr| {
            b.iter(|| parse_expression(black_box(expr)))
        });
    }

    // Annotation handling
    let annotation_cases = ["m{length}", "kg{mass}", "s{time}", "K{temperature}"];
    for expr in &annotation_cases {
        group.bench_with_input(BenchmarkId::new("annotations", expr), expr, |b, &expr| {
            b.iter(|| parse_expression(black_box(expr)))
        });
    }

    group.finish();
}

/// Benchmark edge cases and error conditions
fn bench_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");

    // Empty and minimal expressions
    group.bench_function("empty", |b| b.iter(|| parse_expression(black_box(""))));
    group.bench_function("single_char", |b| {
        b.iter(|| parse_expression(black_box("m")))
    });

    // Leading operators
    group.bench_function("leading_division", |b| {
        b.iter(|| parse_expression(black_box("/min")))
    });

    // Nested parentheses
    group.bench_function("nested_parens", |b| {
        b.iter(|| parse_expression(black_box("((kg.m)/s)/mol")))
    });

    // Very long expressions
    let long_expr = "kg.m.s.A.K.mol.cd/kg.m.s.A.K.mol.cd";
    group.bench_function("long_expression", |b| {
        b.iter(|| parse_expression(black_box(long_expr)))
    });

    group.finish();
}

/// End-to-end benchmark simulating real usage patterns
fn bench_real_world_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world_usage");

    // Complete validation workflow
    group.bench_function("validate_parse_evaluate", |b| {
        b.iter(|| {
            let expr = "mg/dL";
            let _ = validate(black_box(expr));
            let parsed = parse_expression(black_box(expr)).unwrap();
            let _ = evaluate_owned(black_box(&parsed));
        })
    });

    // Medical dosing calculation pattern
    group.bench_function("medical_dosing", |b| {
        b.iter(|| {
            let dose_unit = "mg/kg";
            let time_unit = "/d";
            let _ = validate(black_box(dose_unit));
            let _ = validate(black_box(time_unit));
            let _ = parse_expression(black_box(dose_unit));
            let _ = parse_expression(black_box(time_unit));
        })
    });

    // Engineering calculation pattern
    group.bench_function("engineering_calc", |b| {
        b.iter(|| {
            let pressure = "kPa";
            let area = "m2";
            let force_expr = "kPa.m2"; // Pressure × Area = Force
            let _ = validate(black_box(pressure));
            let _ = validate(black_box(area));
            let _ = analyse(black_box(force_expr));
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_parsing_by_complexity,
    bench_parsing_categories,
    bench_evaluation,
    bench_api_operations,
    bench_parser_features,
    bench_edge_cases,
    bench_real_world_usage
);
criterion_main!(benches);
