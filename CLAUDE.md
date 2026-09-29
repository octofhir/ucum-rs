# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Building
```bash
# Build all workspace crates
cargo build --all

# Build with optimizations
cargo build --release --all

# Build with specific features
cargo build --features cli
cargo build --features fhir
cargo build -p octofhir-ucum-wasm

# Build no_std + alloc (core without default features)
cargo build --no-default-features --target thumbv7em-none-eabihf
```

### Testing
```bash
# Run all tests
cargo test --all

# Run tests with output
cargo test --all -- --nocapture

# Run specific test suite
cargo test
cargo test official_tests  # Official UCUM conformance tests

# Run benchmarks
cargo bench
```

### Linting and Formatting
```bash
# Format all code
cargo fmt --all

# Check formatting without changes
cargo fmt --all -- --check

# Run clippy linter
cargo clippy --all -- -D warnings

# Generate documentation
cargo doc --open --no-deps --all
```

### WASM Build
```bash
# Build WASM package
wasm-pack build wasm --target web --out-dir ../pkg --out-name octofhir_ucum
```

### CLI Usage
```bash
# Install CLI
cargo install --path . --features cli

# CLI commands
octofhir-ucum validate "mg/dL"
octofhir-ucum convert --value 100 --from kPa --to "mm[Hg]"
octofhir-ucum list-units
octofhir-ucum explain kg
octofhir-ucum parse "kg.m/s2"
```

### Playground Development
```bash
cd playground
pnpm install

# Use npm for development due to pnpm script execution issues
npm run dev  # Runs on http://localhost:6000
```

## Architecture Overview

### Core Architecture
The UCUM-RS library implements a zero-copy parsing architecture for performance:

1. **Parser** (`src/parser.rs`):
   - Uses `nom` for zero-copy parsing
   - Dual AST architecture: `UnitExpr<'a>` (borrows) and `OwnedUnitExpr` (owns)
   - Lazy Unicode normalization (only when µ detected)
   - Fast pattern validation with single-pass scanning

2. **Evaluator** (`src/evaluator.rs`):
   - Traverses AST to compute canonical form, dimensions, and conversion factors
   - Handles special units (temperature, logarithmic, arbitrary)
   - Uses HashMap-based prefix lookup for O(1) performance
   - Supports both zero-copy and owned evaluation paths

3. **Registry** (`src/registry.rs`):
   - Generated at compile-time from `ucum-essence.xml` via `build.rs`
   - Contains all UCUM units, prefixes, and their properties
   - Provides O(1) lookups for units and prefixes

4. **Special Units** (`src/special_units.rs`):
   - Extensible handler system for temperature, logarithmic, and arbitrary units
   - Context-aware conversions for units with offsets

### Key Design Patterns

1. **Zero-Copy Optimization**:
   - Parser creates AST that borrows from input strings
   - Evaluator works directly with borrowed data for performance
   - OwnedUnitExpr provided for API compatibility when ownership needed

2. **Compile-Time Code Generation**:
   - `build.rs` parses UCUM XML specification at compile time
   - Generates static registry data structures
   - Ensures no runtime XML parsing overhead

3. **Error Handling**:
   - Comprehensive error types with spans for precise error location
   - Suggestion engine for common mistakes
   - Context-aware error messages

4. **Performance Optimizations**:
   - Lazy Unicode normalization
   - HashMap-based lookups instead of linear search
   - Optional caching system (currently disabled for WASM compatibility)
   - Thread-local storage avoided for WASM support

### Multi-Crate Workspace Structure

- **octofhir-ucum**: Core crate, `no_std` + `alloc` without default features:
  - `std` (default): regex/fuzzy unit search, platform float math
  - `cli`: Command-line interface binary
  - `fhir`: FHIR Quantity data type integration
  - `serde`: Serialization support
- **octofhir-ucum-wasm** (`wasm/`): WebAssembly bindings, published to npm as `@octofhir/ucum-wasm` (not on crates.io)
- **ucum-fuzz**: Fuzzing infrastructure (not published)

### Performance Characteristics

Current benchmarks (v0.3.0):
- Validation: ~322,000 ops/sec
- Parsing: ~280,000 ops/sec (40% improvement with zero-copy)
- Evaluation: ~1,390,000 ops/sec
- Analysis: ~606,000 ops/sec

### Important Implementation Notes

1. **Temperature Conversions**: Special handling required for Celsius/Fahrenheit due to offsets
2. **Unicode Handling**: µ (micro) symbol normalized to 'u' during parsing
3. **Precision**: Uses `rust_decimal` for high-precision arithmetic
4. **no_std**: Core is `no_std` + `alloc`. Use `crate::prelude` for `alloc` types, `hashbrown` for maps, `spin::LazyLock` for statics, and `crate::math` for float functions (`f64::powf` etc. are not in `core`)