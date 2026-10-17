# UCUM-RS Benchmarks

This directory contains performance benchmarks for the UCUM-RS library.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark group
cargo bench parsing
cargo bench evaluation
cargo bench conversion

# Run with specific baseline comparison
cargo bench -- --baseline master
```

## Benchmark Structure

- `parsing_by_complexity`, `parsing_categories`, `parser_features`: parsing of simple,
  prefixed, compound and annotated expressions, powers of ten
- `evaluation`: evaluation of parsed expressions, arbitrary and special units included
- `api`: `validate()` (valid and invalid expressions) and `analyse()`
- `conversion`: linear, customary, temperature, level-to-level, logarithmic and arbitrary
  unit conversions
- `edge_cases`, `real_world_usage`: empty and long expressions, end-to-end patterns

## Viewing Results

Benchmark results are saved in HTML format in `target/criterion/`.
Open `target/criterion/report/index.html` in a browser to view detailed performance graphs and statistics.

## Performance Targets

Current performance targets (on modern hardware):
- Simple parsing: < 1 µs
- Complex parsing: < 5 µs
- Evaluation: < 1 µs
- API operations: < 10 µs