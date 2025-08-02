# UCUM-RS Benchmarks

This directory contains performance benchmarks for the UCUM-RS library.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark group
cargo bench parsing
cargo bench evaluation
cargo bench api

# Run with specific baseline comparison
cargo bench -- --baseline master
```

## Benchmark Structure

The benchmarks are organized into four main groups:

### 1. Parsing (`parsing`)
Tests the performance of parsing various UCUM expressions:
- Simple units (m, kg, s, K, mol)
- Prefixed units (km, mg, µs, kPa, mL)
- Complex expressions (kg.m/s2, mg/dL, km/h, m2.kg/s3/A)

### 2. Evaluation (`evaluation`)
Tests the performance of evaluating parsed expressions:
- Simple units (kg)
- Prefixed units (mg)
- Compound units (kg.m/s2)
- Complex units (mg/dL)

### 3. API Operations (`api`)
Tests high-level API functions:
- `validate()` - Expression validation
- `analyse()` - Unit analysis

### 4. Real-World Usage (`validate_parse_evaluate`)
End-to-end benchmark simulating typical usage patterns.

## Viewing Results

Benchmark results are saved in HTML format in `target/criterion/`.
Open `target/criterion/report/index.html` in a browser to view detailed performance graphs and statistics.

## Performance Targets

Current performance targets (on modern hardware):
- Simple parsing: < 1 µs
- Complex parsing: < 5 µs
- Evaluation: < 1 µs
- API operations: < 10 µs