# UCUM Rust Library

A Rust implementation of UCUM (Unified Code for Units of Measure) for FHIRPath quantity operations, implementing the official UCUM.g4 grammar specification.

## Project Structure

This is a Rust workspace with two packages:

- **`ucum-core`**: The core UCUM library providing parsing, validation, and unit operations
- **`ucum-cli`**: A command-line interface for UCUM operations

## Features

### UCUM Core Library

- **Grammar Compliance**: Full implementation of the UCUM.g4 grammar specification
- **Term Structure**: Supports division (`/`), concatenation (`.`), and components
- **Components**: Handles parenthesized expressions, annotations, and simple units
- **Annotations**: Supports semantic annotations in curly braces `{}`
- **Square Brackets**: Handles special symbols in square brackets `[]`
- **Exponents**: Supports positive and negative exponents
- **Terminal Symbols**: Validates all allowed characters per grammar

### UCUM CLI

- **Validate**: Validate UCUM expressions
- **AST**: Get Abstract Syntax Tree for UCUM expressions
- **List**: List available units with optional filtering
- **Convert**: Convert between units (planned)

## Usage

### Using the Core Library

```rust
use ucum_core::{UcumParser, UcumRegistry};

// Create a parser
let parser = UcumParser::new();

// Parse UCUM expressions
match parser.parse("mg") {
    Ok(result) => println!("Parsed: {:?}", result),
    Err(e) => eprintln!("Error: {}", e),
}

// Work with registry
let registry = UcumRegistry::new().unwrap_or_default();
println!("Registry contains {} units", registry.len());
```

### Using the CLI

```bash
# Validate a UCUM expression
cargo run --bin octofhir-ucum -- validate "mg"

# Get AST for a UCUM expression (debug format)
cargo run --bin octofhir-ucum -- ast "kg/m2"

# Get AST for a UCUM expression (JSON format)
cargo run --bin octofhir-ucum -- ast "kg/m2" --format json

# List available units
cargo run --bin octofhir-ucum -- list

# Get help
cargo run --bin octofhir-ucum -- --help
```

### Validate Command Exit Codes

The `validate` command returns appropriate exit codes for use in scripts:

- **Exit code 0**: Expression is valid
- **Exit code 1**: Expression is invalid (with error details)

### AST Command Formats

The `ast` command supports two output formats:

- **debug** (default): Human-readable debug format showing the parsed structure
- **json**: JSON format for programmatic processing

The AST shows the complete parsed structure including:

- **Component**: Basic unit components
- **Division**: Division operations (`/`)
- **Concatenation**: Concatenation operations (`.`)
- **Annotatable**: Units with optional exponents
- **SimpleUnitSymbols**: Basic unit symbols

## Examples

Run the included examples:

```bash
# Basic usage example
cargo run --example basic_usage --package ucum-core

# FHIRPath integration example
cargo run --example fhirpath_integration --package ucum-core
```

## Building

```bash
# Build all packages
cargo build

# Build specific package
cargo build -p ucum-core
cargo build -p ucum-cli

# Run tests
cargo test
```

## Features

The `ucum-core` package supports the following features:

- `default`: Includes `builtin-data`
- `builtin-data`: Includes built-in UCUM data
- `network`: Enables network features (reqwest, tokio)
- `full`: Includes all features

## License

MIT License - see LICENSE file for details.

## Authors

OctoFHIR Team <funyloony@gmail.com>
