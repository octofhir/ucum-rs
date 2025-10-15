# Contributing to UCUM-RS

We welcome contributions to the UCUM-RS project! This guide will help you get started with development and ensure your contributions align with the project's standards.

## Getting Started

### Prerequisites

- Rust 2024 edition (1.83+)
- `wasm-pack` for WebAssembly builds
- `pnpm` for playground development

### Setup

1. **Fork and clone the repository:**

   ```sh
   git clone https://github.com/YOUR_USERNAME/ucum-rs.git
   cd ucum-rs
   ```

2. **Build the project:**

   ```sh
   # Build all workspace crates
   cargo build --all

   # Build with specific features
   cargo build --features cli
   cargo build --features wasm
   cargo build --features fhir
   ```

3. **Run tests to verify setup:**

   ```sh
   cargo test --all
   ```

## Development Workflow

### Testing

```sh
# Run all tests (recommended before submitting PR)
cargo test --all

# Run tests with output for debugging
cargo test --all -- --nocapture

# Run specific test suites
cargo test official_tests  # Official UCUM conformance tests
cargo test test_micro_normalization  # UTF-8 handling tests

# Run benchmarks
cargo bench
```

### Code Quality

All code must pass formatting and linting checks before being merged:

```sh
# Format code (required before commit)
cargo fmt --all

# Check formatting without changes
cargo fmt --all -- --check

# Run linter with strict warnings
cargo clippy --all -- -D warnings

# Pre-publish validation (runs all quality checks)
just publish-prep  # or: cargo fmt && cargo clippy --all -- -D warnings && cargo test --all
```

### Documentation

```sh
# Generate and open documentation
cargo doc --open --no-deps --all

# Validate documentation examples
cargo test --doc
```

## Architecture Overview

The UCUM-RS library implements a high-performance zero-copy parsing architecture. Key components:

### Core Components

- **Parser** (`src/parser.rs`): Zero-copy parsing with `nom`, handles all UCUM edge cases
- **AST** (`src/ast.rs`): Dual AST architecture - `UnitExpr<'a>` (borrowed) and `OwnedUnitExpr` (owned)
- **Evaluator** (`src/evaluator.rs`): Computes canonical forms, dimensions, and conversion factors
- **Registry** (`src/registry.rs`): Compile-time generated from `ucum-essence.xml` via `build.rs`
- **Special Units** (`src/special_units.rs`): Extensible handlers for temperature, logarithmic units

### Design Principles

1. **Zero-Copy Optimization**: Parser creates AST that borrows from input strings
2. **Compile-Time Code Generation**: Registry generated from XML specification at build time
3. **WASM Compatibility**: No thread-local storage, `no_std` support where possible
4. **Comprehensive Error Handling**: Precise error locations with helpful suggestions

For detailed architecture documentation, see [CLAUDE.md](CLAUDE.md).

## Development Areas

### Core Parser (`src/parser.rs`)

When adding parser features:

1. Implement in `parser.rs` maintaining zero-copy optimizations
2. Add comprehensive tests covering edge cases
3. Ensure backward compatibility with existing API
4. Validate against official UCUM conformance tests

### AST and Evaluation (`src/ast.rs`, `src/evaluator.rs`)

- Follow zero-copy patterns where possible
- Use `UnitExpr<'a>` for borrowed data, `OwnedUnitExpr` for owned data
- Maintain dimensional analysis consistency

### Registry and Build System (`src/registry.rs`, `build.rs`)

- Registry is generated at compile-time from `ucum-essence.xml`
- Changes to build system must maintain WASM compatibility
- Avoid thread-local storage for cross-platform support

## Adding New Features

1. **Create issue** describing the feature and use case
2. **Write tests first** - we follow TDD principles
3. **Implement feature** maintaining backward compatibility
4. **Update documentation** including code examples
5. **Add CLI support** if user-facing (optional)
6. **Add WASM bindings** if relevant (optional)

### Performance Optimizations

If proposing performance improvements:

1. **Benchmark first** - establish baseline performance
2. **Profile bottlenecks** using `cargo bench` and `perf`
3. **Implement optimizations** with clear performance gains
4. **Validate correctness** - all tests must still pass
5. **Document performance gains** with before/after metrics

## Testing Guidelines

### Test Categories

1. **Unit tests** - Test individual functions and components
2. **Integration tests** - Test complete parsing and evaluation flows
3. **Official conformance tests** - UCUM specification compliance (98.6% pass rate)
4. **Property-based tests** - Using `proptest` for edge case discovery
5. **Fuzzing tests** - Located in `ucum-fuzz/` directory

### Writing Tests

```rust
#[test]
fn test_new_feature() {
    // Test successful case
    let result = parse_expression("your_expression").unwrap();
    assert_eq!(result, expected_ast);

    // Test error cases
    assert!(parse_expression("invalid_expression").is_err());

    // Test edge cases
    assert_eq!(parse_expression(""), Ok(UnitExpr::Numeric(1.0)));
}
```

## Code Style

- **Formatting**: Use `cargo fmt` (rustfmt) for consistent formatting
- **Linting**: Address all `clippy` warnings with `cargo clippy --all -- -D warnings`
- **Documentation**: Document all public APIs with examples
- **Error handling**: Use descriptive error messages with context
- **Performance**: Prefer zero-copy patterns, avoid unnecessary allocations

## Submitting Changes

### Pull Request Process

1. **Create feature branch** from `main`
2. **Write tests** covering your changes
3. **Run quality checks**: `just publish-prep` or equivalent commands
4. **Update documentation** if adding public APIs
5. **Submit pull request** with clear description of changes
6. **Address review feedback** promptly

### Pull Request Checklist

- [ ] All tests pass (`cargo test --all`)
- [ ] Code is formatted (`cargo fmt --all -- --check`)
- [ ] No clippy warnings (`cargo clippy --all -- -D warnings`)
- [ ] Documentation updated for public APIs
- [ ] [CHANGELOG.md](CHANGELOG.md) updated if applicable
- [ ] Backward compatibility maintained

## Project-Specific Guidelines

### Unicode Handling

- Always handle µ (micro sign) properly in parsers
- Use UTF-8 byte sequences (0xC2 0xB5) for micro sign detection
- Test with both ASCII 'u' and Unicode 'µ' variants

### WASM Compatibility

- Avoid thread-local storage (`thread_local!`)
- Test WASM builds: `wasm-pack build --target web --features wasm`
- Ensure `no_std` compatibility where possible

### Error Messages

- Provide precise error locations with spans
- Include suggestions for common mistakes
- Test error message clarity with real users

## Playground Development

The interactive playground provides a real-time testing environment:

```sh
cd playground
pnpm install

# Use npm for development due to pnpm script execution issues
npm run dev  # Runs on http://localhost:6000
```

## Getting Help

- **Documentation**: Check [CLAUDE.md](CLAUDE.md) for development commands
- **Issues**: Search existing issues before creating new ones
- **Discussions**: Use GitHub Discussions for questions
- **Code Review**: All changes require review before merging

## Release Process

Releases are managed by project maintainers. The process includes:

1. Version bump in `Cargo.toml`
2. Update [CHANGELOG.md](CHANGELOG.md) with release notes
3. Run `just publish-prep` to validate all quality checks
4. Create GitHub release with tag
5. Publish to crates.io and npm (for WASM package)

## Code of Conduct

- Be respectful and inclusive in all interactions
- Focus on constructive feedback and collaboration
- Help maintain a welcoming environment for all contributors
- Report any unacceptable behavior to project maintainers

## License

By contributing to UCUM-RS, you agree that your contributions will be licensed under the Apache-2.0 License.
