# UCUM-RS Development Guidelines

Before implementing big features prepare ADR(https://github.com/joelparkerhenderson/architecture-decision-record) and only after that start writing code 

Specification placed under the spec folder. Before big changes validate that we align with spec


This document provides guidelines for developing and maintaining the UCUM-RS project, a Rust implementation of the Unified Code for Units of Measure (UCUM).

## Project Structure

The project is organized as a Rust workspace with the following components:

- **ucum-core** (`octofhir-ucum-core`): Core library for parsing, evaluation, and registry
- **ucum-cli** (`octofhir-ucum-cli`): Command-line interface
- **ucum-wasm** (`octofhir-ucum-wasm`): WebAssembly bindings for JavaScript/TypeScript
- **playground**: Interactive web-based playground (Svelte 5)

## Build and Configuration

### Prerequisites

- Rust 2024 edition (nightly toolchain recommended)
- wasm-pack (for WebAssembly builds)
- Node.js 20+ and pnpm (for playground)

### Building the Core Library

The core library uses a custom build process to generate Rust code from the UCUM specification:

```bash
# Build the core library
cd ucum-core
cargo build

# Or from the workspace root
cargo build -p octofhir-ucum-core
```

**Important**: The build process requires the `ucum-essence.xml` file, which contains the UCUM specification data. This file is processed by the `build.rs` script to generate Rust code at build time.

### Building the CLI

```bash
# Build the CLI
cd ucum-cli
cargo build

# Or from the workspace root
cargo build -p octofhir-ucum-cli

# Install the CLI locally
cargo install --path ucum-cli
```

### Building the WebAssembly Package

The WebAssembly package requires `wasm-pack`:

```bash
# Install wasm-pack if not already installed
cargo install wasm-pack

# Build the WebAssembly package
cd ucum-wasm
wasm-pack build --target web

# Build for specific targets
wasm-pack build --target web      # For direct use in browsers
wasm-pack build --target bundler  # For use with bundlers (webpack, rollup, etc.)
wasm-pack build --target nodejs   # For use in Node.js
```

### Building the Playground

The playground is a Svelte 5 application that uses the WebAssembly package:

```bash
# Build the WebAssembly package first
cd ucum-wasm
wasm-pack build --target web

# Build and run the playground
cd ../playground
pnpm install
pnpm dev  # Development server
pnpm build  # Production build
```

**Note**: The playground's build script automatically builds the WebAssembly package, so you can simply run `pnpm build` from the playground directory.

## Testing

### Running Tests

The project uses Rust's built-in testing framework:

```bash
# Run all tests in the workspace
cargo test --all

# Run tests for a specific package
cargo test -p octofhir-ucum-core
cargo test -p octofhir-ucum-cli

# Run a specific test
cargo test -p octofhir-ucum-core --test lookup
```

### Test Organization

Tests are organized as follows:

- **Unit tests**: Located in the `src` directory, alongside the code they test
- **Integration tests**: Located in the `tests` directory, testing the public API

The core library has extensive integration tests in the `tests` directory, covering:

- Expression parsing (`expr.rs`)
- Expression evaluation (`evaluator.rs`)
- Unit lookup (`lookup.rs`)
- Error handling (`error.rs`)
- Special units (`special_units.rs`)
- Arbitrary units (`arbitrary.rs`)
- Unicode handling (`micro.rs`)

### Adding New Tests

To add a new test, create a file in the appropriate `tests` directory:

```rust
// Example test file: ucum-core/tests/example.rs
use octofhir_ucum_core::{evaluate, parse_expression, Dimension};

#[test]
fn test_simple_conversion() {
    // Parse and evaluate meter
    let meter_expr = parse_expression("m").expect("Failed to parse 'm'");
    let meter_result = evaluate(&meter_expr).expect("Failed to evaluate 'm'");
    
    // Parse and evaluate centimeter
    let cm_expr = parse_expression("cm").expect("Failed to parse 'cm'");
    let cm_result = evaluate(&cm_expr).expect("Failed to evaluate 'cm'");
    
    // Check that they have the same dimension (length)
    assert_eq!(meter_result.dim, cm_result.dim);
    assert_eq!(meter_result.dim, Dimension([0, 1, 0, 0, 0, 0, 0]));
    
    // Check the conversion factor: 1 meter = 100 centimeters
    assert_eq!(meter_result.factor / cm_result.factor, 100.0);
}
```

Run the test with:

```bash
cargo test -p octofhir-ucum-core --test example
```

## Development Guidelines

### Code Style

The project follows standard Rust code style. Use the following tools to ensure consistency:

```bash
# Format code
cargo fmt --all

# Check for linting issues
cargo clippy --all -- -D warnings
```

### Documentation

Document public APIs using Rust doc comments. Generate and view documentation with:

```bash
cargo doc --open
```

### WebAssembly Development

When developing the WebAssembly bindings:

1. Make changes to the `ucum-wasm/src/lib.rs` file
2. Build with `wasm-pack build --target web`
3. Test in the playground with `cd ../playground && pnpm dev`

### Playground Development

The playground uses Svelte 5 and Vite. Key files:

- `src/App.svelte`: Main application component
- `src/lib/`: Utility functions and components
- `vite.config.ts`: Vite configuration

### Release Process

1. Update version in `Cargo.toml` workspace configuration
2. Update CHANGELOG.md
3. Commit and tag the release
4. Push to GitHub
5. Publish to crates.io:
   ```bash
   cargo publish -p octofhir-ucum-core
   cargo publish -p octofhir-ucum-cli
   ```
6. Publish to npm:
   ```bash
   cd ucum-wasm
   wasm-pack build --target web
   cd pkg
   npm publish --access public
   ```

## Troubleshooting

### Common Issues

1. **Build fails with missing `ucum-essence.xml`**:
   - Ensure the file is present in the `ucum-core` directory
   - The file is included in the repository and should be available

2. **WebAssembly build fails**:
   - Ensure `wasm-pack` is installed: `cargo install wasm-pack`
   - Check that you have the correct Rust toolchain: `rustup default nightly`

3. **Playground build fails**:
   - Ensure Node.js 20+ is installed
   - Ensure pnpm is installed: `npm install -g pnpm`
   - Run `pnpm install` in the playground directory

4. **Tests fail**:
   - Check that you have the latest code: `git pull`
   - Ensure the `ucum-essence.xml` file is up to date
   - Run `cargo clean && cargo test` to rebuild from scratch
