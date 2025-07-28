# UCUM-RS

[![CI](https://github.com/octofhir/ucum-rs/workflows/CI/badge.svg)](https://github.com/octofhir/ucum-rs/actions/workflows/ci.yml)
[![Crates.io Core](https://img.shields.io/crates/v/octofhir-ucum-core.svg)](https://crates.io/crates/octofhir-ucum-core)
[![Crates.io CLI](https://img.shields.io/crates/v/octofhir-ucum-cli.svg)](https://crates.io/crates/octofhir-ucum-cli)
[![Crates.io FHIR](https://img.shields.io/crates/v/octofhir-ucum-fhir.svg)](https://crates.io/crates/octofhir-ucum-fhir)
[![npm](https://img.shields.io/npm/v/@octofhir/ucum-wasm.svg)](https://www.npmjs.com/package/@octofhir/ucum-wasm)

Unified Code for Units of Measure (UCUM) implementation in Rust 2024 edition.

## Quick Start

```sh
# Add to your project
cargo add octofhir-ucum-core

# Or use the CLI
cargo install --path octofhir-ucum-cli

# Example: Convert 100 kPa to mm[Hg]
octofhir-ucum convert --value 100 --from kPa --to mm[Hg]
```

## Features

### 🚀 Enhanced API (ADR-001 + Phase 3 Implementation)
| Feature                | Status   | Notes                                  |
|------------------------|----------|----------------------------------------|
| **Comprehensive Validation** | ✅ | `validate()` with detailed error reporting |
| **Unit Analysis**      | ✅       | `analyse()` with dimensions, factors, properties |
| **Unit Arithmetic**    | ✅       | `unit_multiply()`, `unit_divide()` operations |
| **Advanced Search**    | ✅       | Text, property, fuzzy, and regex search |
| **Property Validation** | ✅      | `validate_in_property()` for physical quantities |
| **Unit Compatibility** | ✅       | `is_comparable()` for commensurability checking |
| **Canonical Forms**    | ✅       | `get_canonical_units()` for normalization |
| **Special Unit System** | ✅      | Extensible handlers for temperature, logarithmic units |
| **Precision Arithmetic** | ✅     | Optional `rust_decimal` support for high precision |
| **🆕 Model Introspection** | ✅   | `get_model()`, `validate_ucum()`, `get_properties()` |
| **🆕 Enhanced Display Names** | ✅ | `get_common_display()` with prefixed unit support |
| **🆕 Advanced Conversion** | ✅   | `convert_with_context()` with precision control |

### 🔧 Core Capabilities  
| Feature                | Status   | Notes                                  |
|------------------------|----------|----------------------------------------|
| SI base/derived units  | ✅       | Full support with 7-dimensional vectors |
| Customary units        | ✅       | Imperial, US customary, etc.           |
| Specialized units      | ✅       | Medical, laboratory, information units |
| Prefix handling        | ✅       | e.g., kPa, mL, µg with precision support |
| Expression parsing     | ✅       | Grammar-based, robust error messages   |
| Unit conversion        | ✅       | Handles factors, offsets, temperature  |
| Temperature support    | ✅       | Celsius, Fahrenheit, Rankine with offsets |

### 🛠️ Tools & Integration
| Feature                | Status   | Notes                                  |
|------------------------|----------|----------------------------------------|
| CLI tool               | ✅       | `octofhir-ucum-cli` binary             |
| WASM support           | ✅       | npm package: `@octofhir/ucum-wasm`     |
| Interactive playground | ✅       | Svelte 5 web application               |
| FHIR integration       | ✅       | FHIR Quantity data type support        |
| Property-based tests   | ✅       | `proptest`                             |
| Fuzzing                | ✅       | `cargo-fuzz` targets for parser/eval   |

### 📊 Test Conformance (98.6% Overall)
| Test Category          | Status   | Results                                |
|------------------------|----------|----------------------------------------|
| **Overall Conformance** | ✅      | **98.6%** (1120/1136 tests passing)   |
| Validation tests       | ✅       | **99.5%** (1048/1053)                 |
| Conversion tests       | ⚠️       | **83.1%** (49/59) - acceptable precision differences |
| Division tests         | ✅       | **100%** (3/3) - precision arithmetic fixed |
| Multiplication tests   | ✅       | **100%** (4/4)                         |
| Display name tests     | ✅       | **94.1%** (16/17)                     |

### ⚡ Performance
- **Validation**: ~322,000 ops/second (~3.1 µs per operation)
- **Parsing**: ~200,000 ops/second (~5.0 µs per operation)
- **Evaluation**: ~1,390,000 ops/second (~718 ns per operation)
- **Analysis**: ~606,000 ops/second (~1.65 µs per operation)

## WASM Package

The UCUM library is available as a WebAssembly package for use in JavaScript/TypeScript applications.

### Installation

```sh
# Using npm
npm install @octofhir/ucum-wasm

# Using yarn
yarn add @octofhir/ucum-wasm

# Using pnpm
pnpm add @octofhir/ucum-wasm
```

### Usage

```typescript
import { 
  start, 
  validate, 
  get_unit_info, 
  convert, 
  evaluate_expression, 
  arithmetic,
  // Phase 3 functions
  get_ucum_model,
  get_unit_display_name,
  convert_advanced_simple
} from '@octofhir/ucum-wasm';

// Initialize the WASM module
start();

// Validate a UCUM expression
const isValid = validate('mg/dL');  // true

// Get information about a unit
const unitInfo = get_unit_info('mg');
console.log(unitInfo.factor);  // 0.000001
console.log(unitInfo.dimensions);  // [1, 0, 0, 0, 0, 0, 0]

// Convert between units
const result = convert(100, 'kPa', 'mm[Hg]');  // 750.06...

// Evaluate a UCUM expression
const evalResult = evaluate_expression('mg/dL');
console.log(evalResult.factor);  // 0.00001

// Perform arithmetic operations
const arithResult = arithmetic('mg', 'mul', 'mL', 1);
console.log(arithResult.dimensions);  // [1, 3, 0, 0, 0, 0, 0]

// Phase 3: Model introspection
const model = get_ucum_model();
console.log(model.version);       // '2.1'
console.log(model.total_units);   // 312

// Phase 3: Enhanced display names
console.log(get_unit_display_name('kg'));    // 'kilogram'
console.log(get_unit_display_name('m/s'));   // '(meter) / (second)'

// Phase 3: Advanced conversion with precision
const advResult = convert_advanced_simple(1000, 'g', 'kg', 3);
console.log(advResult.value);         // 1.000
console.log(advResult.precision_info); // '3 decimal places'
```

## Phase 3 API Completeness

Phase 3 introduces comprehensive model introspection and advanced conversion capabilities to enhance the UCUM implementation.

### Model Introspection

```rust
use octofhir_ucum_core::{get_model, validate_ucum, get_properties, get_common_display};

// Get model information
let model = get_model();
println!("UCUM Version: {}", model.version);        // "2.1"
println!("Total Units: {}", model.units.len());     // 312
println!("Total Prefixes: {}", model.prefixes.len()); // 24

// Validate implementation self-consistency
let issues = validate_ucum();
if issues.is_empty() {
    println!("UCUM implementation is valid");
} else {
    println!("Issues found: {:?}", issues);
}

// Get all available properties
let properties = get_properties();
println!("Available properties: {}", properties.len()); // 101

// Enhanced display names (handles prefixed units)
println!("{}", get_common_display("kg"));    // "kilogram"
println!("{}", get_common_display("cm"));    // "centimeter"
println!("{}", get_common_display("m/s"));   // "(meter) / (second)"
```

### Advanced Conversion with Precision Control

```rust
use octofhir_ucum_core::{
    convert_with_context, 
    AdvancedConversionContext,
    DecimalPrecision,
    RoundingMode,
    TemperatureScale
};

// Create conversion context with precise control
let context = AdvancedConversionContext {
    precision: DecimalPrecision::Fixed(3),
    rounding: RoundingMode::Nearest,
    temperature_scale: TemperatureScale::Celsius,
    use_special_units: true,
};

// Convert with advanced precision
let result = convert_with_context(1000.0, "g", "kg", &context)?;
println!("Value: {}", result.value);           // 1.000
println!("Precision: {}", result.precision_info); // "3 decimal places"
println!("Used special units: {}", result.used_special_units); // false

// Temperature conversion with special handling
let temp_result = convert_with_context(100.0, "Cel", "K", &context)?;
println!("Value: {}", temp_result.value);      // 373.150
println!("Used special units: {}", temp_result.used_special_units); // true
```

### CLI Integration

All Phase 3 features are available through the CLI:

```sh
# Model introspection
octofhir-ucum model
octofhir-ucum self-validate
octofhir-ucum properties --limit 10

# Enhanced display names
octofhir-ucum display kg           # kilogram
octofhir-ucum display "m/s"        # (meter) / (second)

# Advanced conversion with precision
octofhir-ucum convert-advanced --value 1000 --from g --to kg --precision 3
octofhir-ucum convert-advanced --value 100 --from Cel --to K --precision 2
```

### WASM Integration

Phase 3 functions are fully exposed in the WASM package:

```javascript
// Model introspection
const model = get_ucum_model();
const validation = validate_ucum_implementation();
const properties = get_ucum_properties();

// Enhanced display names
const displayName = get_unit_display_name('kg');

// Advanced conversion
const result = convert_advanced_simple(1000, 'g', 'kg', 3);
const advancedResult = convert_advanced(100, 'Cel', 'K', {
  precision_type: 'fixed',
  precision_value: 2,
  rounding_mode: 'nearest',
  temperature_scale: 'celsius',
  use_special_units: true
});
```

## Interactive Playground

An interactive web-based playground is available to explore the UCUM library's capabilities.

### Features

- **Validation**: Validate UCUM expressions
- **Unit Information**: Get detailed information about units
- **Conversion**: Convert values between compatible units
- **Arithmetic**: Perform arithmetic operations on units
- **Phase 3 Capabilities**: Model introspection, enhanced display names, and advanced conversion with precision control

### Running Locally

```sh
# Navigate to the playground directory
cd playground

# Install dependencies
pnpm install

# Start the development server (use npm due to pnpm script execution issues)
npm run dev
```

The playground will be available at http://localhost:6000.

## FHIR Integration

The UCUM library provides integration with FHIR (Fast Healthcare Interoperability Resources) through the `octofhir-ucum-fhir` crate.

### Installation

```sh
# Add to your project
cargo add octofhir-ucum-fhir
```

### Features

- **FHIR Quantity**: FHIR Quantity data type implementation
- **Conversion**: Convert between FHIR Quantity and UCUM Quantity
- **Unit Conversion**: Convert FHIR Quantities between different units
- **Equivalence**: Check if two FHIR Quantities are equivalent
- **Error Handling**: Comprehensive error handling for invalid inputs

### Usage

```rust
use octofhir_ucum_fhir::{FhirQuantity, convert_quantity, are_equivalent};

// Create a FHIR Quantity with a UCUM code
let quantity = FhirQuantity::with_ucum_code(1000.0, "mg");

// Convert to a different unit
let converted = convert_quantity(&quantity, "g").unwrap();
assert_eq!(converted.value, 1.0);
assert_eq!(converted.code, Some("g".to_string()));

// Check if two quantities are equivalent
let quantity2 = FhirQuantity::with_ucum_code(1.0, "g");
assert!(are_equivalent(&quantity2, &converted).unwrap());
```

## Fuzzing

The UCUM library includes fuzzing infrastructure to identify potential bugs and edge cases using `cargo-fuzz`.

### Setup

```sh
# Install cargo-fuzz
cargo install cargo-fuzz
```

### Fuzzing Targets

- **Parser Fuzzer**: Tests the `parse_expression` function with arbitrary input strings
- **Evaluator Fuzzer**: Tests the `evaluate` function with valid UCUM expressions

### Running the Fuzzers

```sh
# Run the parser fuzzer
cargo fuzz run -p octofhir-ucum-fuzz fuzz_parser

# Run the evaluator fuzzer
cargo fuzz run -p octofhir-ucum-fuzz fuzz_evaluator
```

### Continuous Fuzzing

For continuous fuzzing, you can set up a CI job that runs the fuzzers for a fixed amount of time:

```sh
# Run the parser fuzzer for 5 minutes
cargo fuzz run -p octofhir-ucum-fuzz fuzz_parser -- -max_total_time=300
```

For more details, see the [ucum-fuzz README](ucum-fuzz/README.md).

## Official Test Validation

The UCUM library includes validation against the official UCUM test cases from the FHIR/Ucum-java repository to ensure compliance with the UCUM specification.

### Test Coverage

Our implementation achieves **91.4% conformance** to the official UCUM functional test suite:

- **Total Tests:** 1,068 official UCUM test cases
- **Passed:** 976 tests
- **Failed:** 92 tests
- **Success Rate:** 91.4%

### Running Official Tests

```sh
# Run all official validation tests
cargo test official_tests

# Run with detailed output to see individual test results
cargo test run_official_validation_tests -- --nocapture

# Run tests from the second official test file
cargo test run_official_validation_tests_2 -- --nocapture
```

## Contribution Guide

1. **Clone the repo:**

   ```sh
   git clone https://github.com/YOUR_ORG/ucum-rs.git
   cd ucum-rs
   ```

2. **Build:**

   ```sh
   cargo build --all
   ```

3. **Test:**

   ```sh
   cargo test --all
   ```

4. **Run CLI:**

   ```sh
   cargo run --package octofhir-ucum-cli -- convert --value 1 --from m --to cm
   ```

5. **Build WASM package:**

   ```sh
   cd ucum-wasm
   wasm-pack build --target web
   ```

6. **Run playground:**

   ```sh
   cd playground
   pnpm install
   pnpm dev
   ```

7. **Docs:**

   ```sh
   cargo doc --open
   ```

8. **Formatting & Linting:**

   ```sh
   cargo fmt --all
   cargo clippy --all -- -D warnings
   ```

## Project Structure

- `octofhir-ucum-core/` – Core library (parsing, evaluation, registry)
- `octofhir-ucum-cli/`  – Command-line interface
- `ucum-wasm/` – WebAssembly bindings for JavaScript/TypeScript (@octofhir/ucum-wasm)
- `ucum-fhir/` – FHIR integration (FHIR Quantity data type support)
- `ucum-fuzz/` – Fuzzing infrastructure (cargo-fuzz targets)
- `playground/`         – Interactive web-based playground (Svelte 5)
- `spec/`               – UCUM specification assets

## License

Apache-2.0
