# UCUM-RS

[![CI](https://github.com/octofhir/ucum-rs/workflows/CI/badge.svg)](https://github.com/octofhir/ucum-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/octofhir-ucum.svg)](https://crates.io/crates/octofhir-ucum)
[![npm](https://img.shields.io/npm/v/@octofhir/ucum-wasm.svg)](https://www.npmjs.com/package/@octofhir/ucum-wasm)
[![Docs.rs](https://docs.rs/octofhir-ucum/badge.svg)](https://docs.rs/octofhir-ucum)

High-performance Unified Code for Units of Measure (UCUM) implementation in Rust. Validate, parse, convert, and manipulate units of measure with comprehensive UCUM specification support.

## Quick Start

### Rust Library

```sh
# Add to your project
cargo add octofhir-ucum
```

```rust
use octofhir_ucum::{validate, convert, analyse, unit_multiply};

// Validate UCUM expressions
assert!(validate("mg/dL").is_ok());

// Convert between units
let result = convert(100.0, "kPa", "mm[Hg]")?;
// result ≈ 750.06

// Analyze units (dimensions, properties, canonical form)
let info = analyse("mg/dL")?;
println!("Canonical: {}", info.canonical_form);  // g.dL-1
println!("Dimensions: {:?}", info.dimensions);   // [1, 0, 0, 0, 0, 0, 0]

// Unit arithmetic
let result = unit_multiply("kg", "m/s2")?;
// result = "kg.m/s2" (Newton)
```

### Command-Line Interface

```sh
# Install CLI
cargo install octofhir-ucum

# Validate units
octofhir-ucum validate "mg/dL"

# Convert values
octofhir-ucum convert --value 100 --from kPa --to "mm[Hg]"

# Explore units
octofhir-ucum list-units --property length
octofhir-ucum explain kg
```

### JavaScript/TypeScript (WASM)

```sh
npm install @octofhir/ucum-wasm
```

```typescript
import { start, validate, convert, get_unit_info } from '@octofhir/ucum-wasm';

// Initialize WASM module
start();

// Validate and convert
const isValid = validate('mg/dL');  // true
const result = convert(100, 'kPa', 'mm[Hg]');  // 750.06...

// Get unit information
const info = get_unit_info('mg');
console.log(info.factor);       // 0.000001
console.log(info.dimensions);   // [1, 0, 0, 0, 0, 0, 0]
```

## Features

### Core Capabilities

- **Full UCUM Support**: SI base/derived units, customary units, specialized medical/laboratory units
- **Validation**: Comprehensive syntax validation with helpful error messages
- **Unit Conversion**: Accurate conversion with support for temperature offsets and logarithmic units
- **Unit Analysis**: Dimensional analysis, canonical forms, and commensurability checking
- **Unit Arithmetic**: Multiply and divide units to derive new units
- **Advanced Search**: Find units by text, property, dimensions, or fuzzy matching
- **High Performance**: Zero-copy parsing architecture with optimized lookups

### Integration Options

- **Rust Library**: Full-featured API with zero-copy parsing
- **CLI Tool**: Command-line interface for validation, conversion, and exploration
- **WASM Package**: Use in browsers and Node.js applications
- **FHIR Support**: Native integration with FHIR Quantity data types

### Conformance

- 98.6% conformance with official UCUM test suite (1120/1136 tests passing)
- Handles edge cases: temperature conversions, logarithmic units, arbitrary units
- Comprehensive error reporting with suggestions for common mistakes

## Usage Examples

### Unit Conversion

```rust
use octofhir_ucum::convert;

// Simple conversion
let kg_to_g = convert(1.0, "kg", "g")?;  // 1000.0

// Pressure conversion
let kpa_to_mmhg = convert(100.0, "kPa", "mm[Hg]")?;  // 750.06

// Temperature conversion (with offset handling)
let celsius_to_kelvin = convert(100.0, "Cel", "K")?;  // 373.15
```

### Unit Validation

```rust
use octofhir_ucum::validate;

// Valid units
assert!(validate("mg/dL").is_ok());
assert!(validate("kg.m/s2").is_ok());

// Invalid units - get helpful error messages
match validate("mg/invalid") {
    Err(e) => println!("Error: {}", e),  // "Unknown unit: invalid"
    Ok(_) => {}
}
```

### Unit Analysis

```rust
use octofhir_ucum::analyse;

let info = analyse("mg/dL")?;
println!("Canonical: {}", info.canonical_form);  // g.dL-1
println!("Factor: {}", info.factor);             // 0.00001
println!("Property: {}", info.property);         // mass concentration
println!("Dimensions: {:?}", info.dimensions);   // [1, 0, 0, 0, 0, 0, 0]
```

### Advanced Conversion with Precision

```rust
use octofhir_ucum::{convert_with_context, AdvancedConversionContext, DecimalPrecision, RoundingMode};

let context = AdvancedConversionContext {
    precision: DecimalPrecision::Fixed(3),
    rounding: RoundingMode::Nearest,
    ..Default::default()
};

let result = convert_with_context(1000.0, "g", "kg", &context)?;
println!("{}", result.value);  // 1.000
```

## FHIR Integration

Seamless integration with FHIR Quantity data types:

```rust
use octofhir_ucum_fhir::{FhirQuantity, convert_quantity, are_equivalent};

// Create FHIR Quantity
let quantity = FhirQuantity::with_ucum_code(1000.0, "mg");

// Convert units
let converted = convert_quantity(&quantity, "g")?;
assert_eq!(converted.value, 1.0);

// Check equivalence
let quantity2 = FhirQuantity::with_ucum_code(1.0, "g");
assert!(are_equivalent(&quantity2, &converted)?);
```

Enable FHIR support:

```sh
cargo add octofhir-ucum --features fhir
```

## Interactive Playground

Try the library in your browser with our interactive playground:

```sh
cd playground
pnpm install
npm run dev  # http://localhost:6000
```

The playground provides real-time validation, conversion, and unit exploration.

## Documentation

- [API Documentation](https://docs.rs/octofhir-ucum) - Complete API reference
- [User Guide](USER_GUIDE.md) - Comprehensive usage guide with examples
- [Tutorial](TUTORIAL.md) - Step-by-step introduction
- [Contributing Guide](CONTRIBUTING.md) - Development setup and guidelines
- [Changelog](CHANGELOG.md) - Release history

## Performance

The library is designed for high performance with zero-copy parsing:

- Simple parsing: ~7.9M ops/sec (~126 ns/op)
- Complex expressions: ~1.5M ops/sec (~640 ns/op)
- Unit evaluation: ~1.3M ops/sec (~718 ns/op)

See [benchmarks](benches/) for detailed performance metrics.

## Project Structure

```
ucum-rs/
├── src/
│   ├── lib.rs              # Core library
│   ├── parser.rs           # Zero-copy parser
│   ├── evaluator.rs        # Unit evaluation
│   ├── registry.rs         # Unit registry (generated)
│   ├── bin/cli.rs          # Command-line interface
│   ├── wasm.rs             # WebAssembly bindings
│   └── fhir.rs             # FHIR integration
├── playground/             # Interactive web playground
├── ucum-fuzz/              # Fuzzing infrastructure
└── spec/                   # UCUM specification assets
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Development setup and workflow
- Architecture overview
- Testing guidelines
- Code style requirements
- Pull request process

## License

Apache-2.0

## Resources

- [UCUM Specification](https://ucum.org/)
- [FHIR Quantity](https://www.hl7.org/fhir/datatypes.html#Quantity)
- [GitHub Repository](https://github.com/octofhir/ucum-rs)
- [Issue Tracker](https://github.com/octofhir/ucum-rs/issues)
