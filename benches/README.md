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

## Reference Results

On an Apple M1 (Rust 1.99): parsing a simple unit ~62 ns, `mg/dL` ~170 ns, `validate()`
~0.7 µs (~38 µs for an invalid expression, which computes suggestions), `analyse()`
~0.8 µs, a linear conversion ~1.3 µs, a temperature conversion ~2.6 µs.
