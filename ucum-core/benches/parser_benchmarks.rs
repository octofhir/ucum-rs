use criterion::{Criterion, black_box, criterion_group, criterion_main};
use octofhir_ucum_core::parse_expression;

fn bench_simple_units(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_simple_units");

    // Basic units
    group.bench_function("meter", |b| b.iter(|| parse_expression(black_box("m"))));

    group.bench_function("gram", |b| b.iter(|| parse_expression(black_box("g"))));

    group.bench_function("second", |b| b.iter(|| parse_expression(black_box("s"))));

    group.bench_function("kelvin", |b| b.iter(|| parse_expression(black_box("K"))));

    group.finish();
}

fn bench_prefixed_units(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_prefixed_units");

    // Common prefixed units
    group.bench_function("kilogram", |b| b.iter(|| parse_expression(black_box("kg"))));

    group.bench_function("centimeter", |b| {
        b.iter(|| parse_expression(black_box("cm")))
    });

    group.bench_function("millimeter", |b| {
        b.iter(|| parse_expression(black_box("mm")))
    });

    group.bench_function("kilopascal", |b| {
        b.iter(|| parse_expression(black_box("kPa")))
    });

    group.bench_function("milligram", |b| {
        b.iter(|| parse_expression(black_box("mg")))
    });

    group.finish();
}

fn bench_power_expressions(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_power_expressions");

    // Power expressions
    group.bench_function("square_meter", |b| {
        b.iter(|| parse_expression(black_box("m2")))
    });

    group.bench_function("cubic_meter", |b| {
        b.iter(|| parse_expression(black_box("m3")))
    });

    group.bench_function("meter_power_2", |b| {
        b.iter(|| parse_expression(black_box("m^2")))
    });

    group.bench_function("meter_power_3", |b| {
        b.iter(|| parse_expression(black_box("m^3")))
    });

    group.bench_function("negative_power", |b| {
        b.iter(|| parse_expression(black_box("m^-1")))
    });

    group.finish();
}

fn bench_complex_expressions(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_complex_expressions");

    // Product expressions
    group.bench_function("meter_per_second", |b| {
        b.iter(|| parse_expression(black_box("m/s")))
    });

    group.bench_function("meter_per_second_squared", |b| {
        b.iter(|| parse_expression(black_box("m/s2")))
    });

    group.bench_function("newton_meter", |b| {
        b.iter(|| parse_expression(black_box("N.m")))
    });

    group.bench_function("kilogram_meter_per_second", |b| {
        b.iter(|| parse_expression(black_box("kg.m/s")))
    });

    // Complex medical units
    group.bench_function("mg_per_dl", |b| {
        b.iter(|| parse_expression(black_box("mg/dL")))
    });

    group.bench_function("mmol_per_l", |b| {
        b.iter(|| parse_expression(black_box("mmol/L")))
    });

    group.bench_function("units_per_ml", |b| {
        b.iter(|| parse_expression(black_box("[IU]/mL")))
    });

    group.finish();
}

fn bench_special_units(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_special_units");

    // Special units with brackets
    group.bench_function("international_unit", |b| {
        b.iter(|| parse_expression(black_box("[IU]")))
    });

    group.bench_function("arbitrary_unit", |b| {
        b.iter(|| parse_expression(black_box("[arb'U]")))
    });

    group.bench_function("celsius", |b| b.iter(|| parse_expression(black_box("Cel"))));

    group.bench_function("fahrenheit", |b| {
        b.iter(|| parse_expression(black_box("[degF]")))
    });

    group.finish();
}

fn bench_parenthesized_expressions(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_parenthesized");

    group.bench_function("simple_parentheses", |b| {
        b.iter(|| parse_expression(black_box("(m)")))
    });

    group.bench_function("complex_parentheses", |b| {
        b.iter(|| parse_expression(black_box("(kg.m)/s2")))
    });

    group.bench_function("nested_parentheses", |b| {
        b.iter(|| parse_expression(black_box("((m/s)/s)")))
    });

    group.finish();
}

fn bench_numeric_expressions(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_numeric");

    group.bench_function("integer", |b| b.iter(|| parse_expression(black_box("10"))));

    group.bench_function("decimal", |b| {
        b.iter(|| parse_expression(black_box("10.5")))
    });

    group.bench_function("scientific_notation", |b| {
        b.iter(|| parse_expression(black_box("1e3")))
    });

    group.bench_function("numeric_with_unit", |b| {
        b.iter(|| parse_expression(black_box("10.m")))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_units,
    bench_prefixed_units,
    bench_power_expressions,
    bench_complex_expressions,
    bench_special_units,
    bench_parenthesized_expressions,
    bench_numeric_expressions
);
criterion_main!(benches);
