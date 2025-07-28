use criterion::{Criterion, black_box, criterion_group, criterion_main};
use octofhir_ucum_core::{evaluate_owned, parse_expression};

fn eval_expression(
    expr: &str,
) -> Result<octofhir_ucum_core::EvalResult, octofhir_ucum_core::UcumError> {
    let ast = parse_expression(expr)?;
    evaluate_owned(&ast)
}

fn bench_simple_unit_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluator_simple_units");

    // Basic units
    group.bench_function("meter", |b| b.iter(|| eval_expression(black_box("m"))));

    group.bench_function("gram", |b| b.iter(|| eval_expression(black_box("g"))));

    group.bench_function("second", |b| b.iter(|| eval_expression(black_box("s"))));

    group.bench_function("kelvin", |b| b.iter(|| eval_expression(black_box("K"))));

    group.bench_function("pascal", |b| b.iter(|| eval_expression(black_box("Pa"))));

    group.finish();
}

fn bench_prefixed_unit_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluator_prefixed_units");

    // Common prefixed units - these require prefix lookup and calculation
    group.bench_function("kilogram", |b| b.iter(|| eval_expression(black_box("kg"))));

    group.bench_function("centimeter", |b| {
        b.iter(|| eval_expression(black_box("cm")))
    });

    group.bench_function("millimeter", |b| {
        b.iter(|| eval_expression(black_box("mm")))
    });

    group.bench_function("kilopascal", |b| {
        b.iter(|| eval_expression(black_box("kPa")))
    });

    group.bench_function("milligram", |b| b.iter(|| eval_expression(black_box("mg"))));

    group.bench_function("microgram", |b| b.iter(|| eval_expression(black_box("ug"))));

    group.finish();
}

fn bench_power_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluator_power_expressions");

    // Power expressions - require dimension calculations
    group.bench_function("square_meter", |b| {
        b.iter(|| eval_expression(black_box("m2")))
    });

    group.bench_function("cubic_meter", |b| {
        b.iter(|| eval_expression(black_box("m3")))
    });

    group.bench_function("meter_power_2", |b| {
        b.iter(|| eval_expression(black_box("m^2")))
    });

    group.bench_function("meter_power_3", |b| {
        b.iter(|| eval_expression(black_box("m^3")))
    });

    group.bench_function("negative_power", |b| {
        b.iter(|| eval_expression(black_box("m^-1")))
    });

    group.bench_function("high_power", |b| {
        b.iter(|| eval_expression(black_box("m^10")))
    });

    group.finish();
}

fn bench_complex_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluator_complex_expressions");

    // Product and quotient expressions - require multiple unit lookups and calculations
    group.bench_function("meter_per_second", |b| {
        b.iter(|| eval_expression(black_box("m/s")))
    });

    group.bench_function("meter_per_second_squared", |b| {
        b.iter(|| eval_expression(black_box("m/s2")))
    });

    group.bench_function("newton_meter", |b| {
        b.iter(|| eval_expression(black_box("N.m")))
    });

    group.bench_function("kilogram_meter_per_second", |b| {
        b.iter(|| eval_expression(black_box("kg.m/s")))
    });

    group.bench_function("joule_per_kelvin", |b| {
        b.iter(|| eval_expression(black_box("J/K")))
    });

    group.finish();
}

fn bench_medical_units_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluator_medical_units");

    // Common medical units - often used in healthcare applications
    group.bench_function("mg_per_dl", |b| {
        b.iter(|| eval_expression(black_box("mg/dL")))
    });

    group.bench_function("mmol_per_l", |b| {
        b.iter(|| eval_expression(black_box("mmol/L")))
    });

    group.bench_function("units_per_ml", |b| {
        b.iter(|| eval_expression(black_box("[IU]/mL")))
    });

    group.bench_function("mg_per_kg_per_day", |b| {
        b.iter(|| eval_expression(black_box("mg/kg/d")))
    });

    group.bench_function("cells_per_ul", |b| {
        b.iter(|| eval_expression(black_box("10*3/uL")))
    });

    group.finish();
}

fn bench_special_units_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluator_special_units");

    // Special units with brackets and temperature units
    group.bench_function("international_unit", |b| {
        b.iter(|| eval_expression(black_box("[IU]")))
    });

    group.bench_function("arbitrary_unit", |b| {
        b.iter(|| eval_expression(black_box("[arb'U]")))
    });

    group.bench_function("celsius", |b| b.iter(|| eval_expression(black_box("Cel"))));

    group.bench_function("fahrenheit", |b| {
        b.iter(|| eval_expression(black_box("[degF]")))
    });

    group.bench_function("percent", |b| b.iter(|| eval_expression(black_box("%"))));

    group.finish();
}

fn bench_parenthesized_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluator_parenthesized");

    group.bench_function("simple_parentheses", |b| {
        b.iter(|| eval_expression(black_box("(m)")))
    });

    group.bench_function("complex_parentheses", |b| {
        b.iter(|| eval_expression(black_box("(kg.m)/s2")))
    });

    group.bench_function("nested_parentheses", |b| {
        b.iter(|| eval_expression(black_box("((m/s)/s)")))
    });

    group.bench_function("multiple_parentheses", |b| {
        b.iter(|| eval_expression(black_box("(kg.m2)/(s3.K)")))
    });

    group.finish();
}

fn bench_numeric_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluator_numeric");

    group.bench_function("integer", |b| b.iter(|| eval_expression(black_box("10"))));

    group.bench_function("decimal", |b| b.iter(|| eval_expression(black_box("10.5"))));

    group.bench_function("scientific_notation", |b| {
        b.iter(|| eval_expression(black_box("1e3")))
    });

    group.bench_function("numeric_with_unit", |b| {
        b.iter(|| eval_expression(black_box("10.m")))
    });

    group.bench_function("complex_numeric", |b| {
        b.iter(|| eval_expression(black_box("2.5.kg.m/s2")))
    });

    group.finish();
}

fn bench_parse_and_evaluate_combined(c: &mut Criterion) {
    let mut group = c.benchmark_group("combined_parse_and_evaluate");

    // Test the full pipeline: parse + evaluate
    let expressions = vec![
        "m",
        "kg",
        "m/s",
        "m/s2",
        "N.m",
        "mg/dL",
        "mmol/L",
        "[IU]/mL",
        "Cel",
        "kPa",
        "cm2",
        "m^3",
        "(kg.m)/s2",
    ];

    for expr in expressions {
        group.bench_function(
            &format!(
                "full_pipeline_{}",
                expr.replace("/", "_per_")
                    .replace(".", "_dot_")
                    .replace("^", "_pow_")
                    .replace("[", "")
                    .replace("]", "")
            ),
            |b| {
                b.iter(|| {
                    let ast = parse_expression(black_box(expr)).unwrap();
                    evaluate(black_box(&ast)).unwrap()
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_unit_evaluation,
    bench_prefixed_unit_evaluation,
    bench_power_evaluation,
    bench_complex_evaluation,
    bench_medical_units_evaluation,
    bench_special_units_evaluation,
    bench_parenthesized_evaluation,
    bench_numeric_evaluation,
    bench_parse_and_evaluate_combined
);
criterion_main!(benches);
