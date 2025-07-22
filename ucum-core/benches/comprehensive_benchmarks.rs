use criterion::{Criterion, black_box, criterion_group, criterion_main};
use octofhir_ucum_core::*;

fn benchmark_parsing(c: &mut Criterion) {
    let units = vec![
        "m", "kg", "s", "A", "K", "mol", "cd", "m/s", "kg.m/s2", "J", "W", "Pa", "Hz", "km/h",
        "mg/dL", "mmol/L", "Cel", "[in_i]",
    ];

    c.bench_function("parse_simple_units", |b| {
        b.iter(|| {
            for unit in &units {
                let _ = black_box(parse_expression(unit));
            }
        })
    });
}

fn benchmark_evaluation(c: &mut Criterion) {
    let expressions = vec![
        parse_expression("m").unwrap(),
        parse_expression("kg.m/s2").unwrap(),
        parse_expression("km/h").unwrap(),
    ];

    c.bench_function("evaluate_expressions", |b| {
        b.iter(|| {
            for expr in &expressions {
                let _ = black_box(evaluate(expr));
            }
        })
    });
}

fn benchmark_new_api(c: &mut Criterion) {
    c.bench_function("validate_units", |b| {
        b.iter(|| {
            let _ = black_box(validate("kg.m/s2"));
            let _ = black_box(validate("km/h"));
            let _ = black_box(validate("invalid_unit"));
        })
    });

    c.bench_function("analyse_units", |b| {
        b.iter(|| {
            let _ = black_box(analyse("kg.m/s2"));
            let _ = black_box(analyse("km/h"));
        })
    });

    c.bench_function("unit_arithmetic", |b| {
        b.iter(|| {
            let _ = black_box(unit_multiply("m", "s"));
            let _ = black_box(unit_divide("m", "s"));
        })
    });
}

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_evaluation,
    benchmark_new_api
);
criterion_main!(benches);
