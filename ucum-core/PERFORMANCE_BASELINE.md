# UCUM-RS Performance Baseline

## Benchmark Results (Release Build)

Performance metrics established after ADR-001 implementation completion:

### Core Operations
- **parse_simple_units**: 5.01 µs ± 0.05 µs
  - Parsing 18 different unit expressions including basic units, compound units, and special units
  - Excellent performance for real-time applications

- **evaluate_expressions**: 718 ns ± 3 ns  
  - Evaluating parsed expressions to get factors and dimensions
  - Sub-microsecond performance, very fast

### New API Functions (ADR-001)
- **validate_units**: 3.11 µs ± 0.02 µs
  - Comprehensive unit validation including parsing and evaluation
  - Good performance for validation workflows

- **analyse_units**: 1.65 µs ± 0.03 µs
  - Detailed unit analysis with dimension vectors and factors
  - Fast enough for interactive applications

- **unit_arithmetic**: 1.09 µs ± 0.01 µs
  - Unit multiplication and division operations
  - Excellent performance for mathematical operations

## Performance Assessment

✅ **Excellent Overall Performance**
- All operations complete in microseconds
- Sub-microsecond evaluation performance
- Suitable for real-time and interactive applications
- No performance regression from ADR-001 enhancements

## System Information
- **Build**: Release mode with optimizations
- **Backend**: Plotters (Gnuplot not available)
- **Samples**: 100 measurements per benchmark
- **Outliers**: 6-13% (normal for microbenchmarks)

## Recommendations
- Current performance is production-ready
- No immediate optimization needed
- Monitor performance with future feature additions
- Consider caching for repeated operations in high-throughput scenarios
