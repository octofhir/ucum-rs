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

### 🚀 Enhanced API (ADR-001 Implementation)
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
- **Parsing**: ~5.01 µs for multiple unit expressions
- **Evaluation**: ~718 ns for parsed expressions  
- **Validation**: ~3.11 µs for comprehensive validation
- **Analysis**: ~1.65 µs for detailed unit analysis
- **Arithmetic**: ~1.09 µs for multiplication/division

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
  arithmetic 
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
```

## Interactive Playground

An interactive web-based playground is available to explore the UCUM library's capabilities.

### Features

- **Validation**: Validate UCUM expressions
- **Unit Information**: Get detailed information about units
- **Conversion**: Convert values between compatible units
- **Arithmetic**: Perform arithmetic operations on units

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

MIT OR Apache-2.0
