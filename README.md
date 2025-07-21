# UCUM-RS

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

| Feature                | Status   | Notes                                  |
|------------------------|----------|----------------------------------------|
| SI base/derived units  | ✅       | Full support                           |
| Customary units        | ✅       | Imperial, US customary, etc.           |
| Specialized units      | ✅       | Medical, laboratory, information units |
| Prefix handling        | ✅       | e.g., kPa, mL, µg                      |
| Expression parsing     | ✅       | Grammar-based, robust error messages   |
| Unit conversion        | ✅       | Handles factors, offsets, temperature  |
| CLI tool               | ✅       | `octofhir-ucum-cli` binary             |
| WASM support           | ✅       | npm package: `@octofhir/ucum-wasm`     |
| Interactive playground | ✅       | Svelte 5 web application               |
| FHIR integration demo  | 🚧       | Planned                                |
| Property-based tests   | ✅       | `proptest`                             |
| Fuzzing                | 🚧       | Planned                                |

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
- `playground/`         – Interactive web-based playground (Svelte 5)
- `spec/`               – UCUM specification assets

## License

MIT OR Apache-2.0
