# UCUM-RS Development Commands

# Default command - show available commands
default:
    @just --list

# Build all workspace crates
build:
    cargo build --all

# Build with optimizations
build-release:
    cargo build --release --all

# Build with specific features
build-cli:
    cargo build --features cli

build-wasm:
    cargo build --features wasm

build-fhir:
    cargo build --features fhir

# Run all tests
test:
    cargo test --all

# Run tests with output
test-verbose:
    cargo test --all -- --nocapture

# Run specific test suite
test-core:
    cargo test

# Run official UCUM conformance tests
test-official:
    cargo test official_tests

# Run benchmarks
bench:
    cargo bench

# Run benchmarks and save baseline
bench-save name:
    cargo bench -- --save-baseline {{name}}

# Compare benchmarks against baseline
bench-compare baseline:
    cargo bench -- --baseline {{baseline}}

# Open benchmark results in browser
bench-open:
    open target/criterion/report/index.html

# Run parser performance comparison (optimized is now default)
bench-parser:
    @echo "🚀 Running UCUM Parser Performance Comparison"
    @echo "============================================="
    @echo "📊 Building with optimized parser (now default)..."
    cargo build --release
    @echo ""
    @echo "🔥 Running main benchmarks..."
    cargo bench benchmarks
    @echo ""
    @echo "💾 Running memory benchmarks..."
    cargo bench memory_bench
    @echo ""
    @echo "🎯 Performance Summary:"
    @echo "- Check target/criterion/report/index.html for detailed results"
    @echo "- Compare 'default' vs 'optimized_direct' results in parsing_comparison group"
    @echo "- Memory allocation patterns tested in memory_bench"
    @echo ""
    @echo "✅ Benchmark run complete!"

# Test all functionality including optimized parser
test-all:
    cargo test --all-features

# Build release version (now includes optimized parser by default)
build-release-optimized:
    cargo build --release

# Format all code
fmt:
    cargo fmt --all

# Check formatting without changes
fmt-check:
    cargo fmt --all -- --check

# Run clippy linter
lint:
    cargo clippy --all -- -D warnings

# Generate documentation
doc:
    cargo doc --open --no-deps --all

# Build WASM package
wasm-build:
    wasm-pack build --target web --features wasm

# Install CLI tool
install-cli:
    cargo install --path . --features cli

# Show CLI help
cli-help:
    octofhir-ucum --help

# CLI: Validate a unit expression (e.g., just cli-validate "mg/dL")
cli-validate expr:
    octofhir-ucum validate "{{expr}}"

# CLI: Analyze a unit expression (e.g., just cli-analyze "kg.m/s2")
cli-analyze expr:
    octofhir-ucum analyze "{{expr}}"

# CLI: Get canonical form of a unit (e.g., just cli-canonical "kPa")
cli-canonical expr:
    octofhir-ucum canonical "{{expr}}"

# CLI: Convert between units (e.g., just cli-convert 100 kPa "mm[Hg]")
cli-convert value from to:
    octofhir-ucum convert {{value}} {{from}} {{to}}

# CLI: Check if units are comparable (e.g., just cli-comparable m ft)
cli-comparable unit1 unit2:
    octofhir-ucum comparable {{unit1}} {{unit2}}

# CLI: Search for units (e.g., just cli-search "pressure")
cli-search query:
    octofhir-ucum search "{{query}}"

# CLI: Search with limit (e.g., just cli-search-limit "meter" 20)
cli-search-limit query limit:
    octofhir-ucum search "{{query}}" --limit {{limit}}

# Playground development
playground-install:
    cd playground && pnpm install

playground-dev:
    cd playground && npm run dev

# Run all checks before commit
check: fmt lint test
    @echo "All checks passed!"

# Prepare library for publishing (format, lint, test, build)
publish-prep: fmt lint test build-release
    @echo "🚀 Preparing library for publishing..."
    @echo "✅ Code formatted with cargo fmt"
    @echo "✅ Linting passed with cargo clippy" 
    @echo "✅ All tests passed"
    @echo "✅ Release build completed"
    @echo ""
    @echo "📦 Ready to publish! Run 'cargo publish --dry-run' to verify package"

# Clean build artifacts
clean:
    cargo clean

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated