#![allow(clippy::result_large_err)] // UcumError is intentionally large, see src/lib.rs

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use octofhir_ucum::{analyse, convert, evaluate, parse_expression, validate};
use std::hint::black_box;

/// Parsing across expression complexities
fn bench_parsing_by_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_by_complexity");
    let test_cases = [
        ("simple_unit", "m"),
        ("prefixed_unit", "kg"),
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
        ("square_brackets", "[in_i]"),
        ("celsius", "Cel"),
    ];
    for (name, expr) in &test_cases {
        group.bench_with_input(BenchmarkId::new("parse", name), expr, |b, &expr| {
            b.iter(|| parse_expression(black_box(expr)))
        });
    }
    group.finish();
}

/// Parsing of unit categories
fn bench_parsing_categories(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_categories");
    let categories: [(&str, &[&str]); 4] = [
        ("base_units", &["m", "s", "g", "rad", "K", "C", "cd"]),
        (
            "prefixed",
            &["km", "mg", "us", "kPa", "mL", "cm", "mm", "ng"],
        ),
        (
            "medical",
            &["mg/dL", "mmol/L", "meq/L", "U/L", "mm[Hg]", "{beats}/min"],
        ),
        (
            "engineering",
            &["N", "Pa", "J", "W", "V", "Ohm", "Hz", "kPa"],
        ),
    ];
    for (category, units) in categories {
        for unit in units {
            group.bench_with_input(BenchmarkId::new(category, unit), unit, |b, &unit| {
                b.iter(|| parse_expression(black_box(unit)))
            });
        }
    }
    group.finish();
}

/// Evaluation of parsed expressions
fn bench_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluation");
    for (name, expr) in [
        ("simple", "kg"),
        ("prefixed", "mg"),
        ("compound", "kg.m/s2"),
        ("complex", "mg/dL"),
        ("arbitrary", "[IU]/mL"),
        ("special", "dB[SPL]"),
    ] {
        let parsed = parse_expression(expr).unwrap();
        group.bench_function(name, |b| b.iter(|| evaluate(black_box(&parsed))));
    }
    group.finish();
}

/// High-level API functions
fn bench_api_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("api");
    group.bench_function("validate", |b| b.iter(|| validate(black_box("kg.m/s2"))));
    group.bench_function("validate_invalid", |b| {
        b.iter(|| validate(black_box("kg.mx/s2")))
    });
    group.bench_function("analyse", |b| b.iter(|| analyse(black_box("kg.m/s2"))));
    group.finish();
}

/// Conversions: linear, with an offset and through a special function
fn bench_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("conversion");
    for (name, from, to) in [
        ("linear", "mg/dL", "g/L"),
        ("customary", "[lb_av]", "kg"),
        ("temperature", "Cel", "[degF]"),
        ("levels", "dB", "Np"),
        ("logarithmic", "[pH]", "mol/L"),
        ("arbitrary", "[IU]/mL", "[IU]/L"),
    ] {
        group.bench_function(name, |b| {
            b.iter(|| convert(black_box(7.0), black_box(from), black_box(to)))
        });
    }
    group.finish();
}

/// Parser features: powers of ten and annotations
fn bench_parser_features(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_features");
    for expr in &["10*3", "10^-2", "10*6.mol", "10^-12.kg"] {
        group.bench_with_input(BenchmarkId::new("ten_power", expr), expr, |b, &expr| {
            b.iter(|| parse_expression(black_box(expr)))
        });
    }
    for expr in &["m{length}", "kg{mass}", "s{time}", "K{temperature}"] {
        group.bench_with_input(BenchmarkId::new("annotations", expr), expr, |b, &expr| {
            b.iter(|| parse_expression(black_box(expr)))
        });
    }
    group.finish();
}

/// Edge cases
fn bench_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");
    group.bench_function("empty", |b| b.iter(|| parse_expression(black_box(""))));
    group.bench_function("single_char", |b| {
        b.iter(|| parse_expression(black_box("m")))
    });
    group.bench_function("nested_parens", |b| {
        b.iter(|| parse_expression(black_box("((kg.m)/s)/mol")))
    });
    let long_expr = "kg.m.s.A.K.mol.cd/kg.m.s.A.K.mol.cd";
    group.bench_function("long_expression", |b| {
        b.iter(|| parse_expression(black_box(long_expr)))
    });
    group.finish();
}

/// End-to-end usage patterns
fn bench_real_world_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world_usage");
    group.bench_function("validate_parse_evaluate", |b| {
        b.iter(|| {
            let expr = "mg/dL";
            let _ = validate(black_box(expr));
            let parsed = parse_expression(black_box(expr)).unwrap();
            let _ = evaluate(black_box(&parsed));
        })
    });
    group.bench_function("medical_dosing", |b| {
        b.iter(|| convert(black_box(5.0), black_box("mg/kg/d"), black_box("ug/kg/h")))
    });
    group.bench_function("engineering_calc", |b| {
        b.iter(|| analyse(black_box("kPa.m2")))
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_parsing_by_complexity,
    bench_parsing_categories,
    bench_evaluation,
    bench_api_operations,
    bench_conversion,
    bench_parser_features,
    bench_edge_cases,
    bench_real_world_usage
);
criterion_main!(benches);
