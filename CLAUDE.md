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
cargo test --test official  # Official UCUM conformance tests

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
octofhir-ucum convert 100 kPa "mm[Hg]"
octofhir-ucum analyze "[pH]"
octofhir-ucum canonical N
octofhir-ucum comparable Cel "[degF]"
octofhir-ucum search pascal
octofhir-ucum -i validate PAL   # case insensitive codes
```

### Playground Development
```bash
cd playground
pnpm install

# Use npm for development due to pnpm script execution issues
npm run dev  # Runs on http://localhost:6000
```

## Specification Conformance (mandatory)

The library follows the UCUM specification, version 2.2 (`spec/`, `ucum-essence.xml`,
`tests/official/`). This is a requirement, not a goal: when code and specification disagree,
the code is wrong, and spec conformance takes priority over backward compatibility. A change
to semantics must cite the section (§) it implements. Documented choices where the sources
disagree are listed in the README ("Specification conformance").

## Architecture Overview

1. **Registry** (`build.rs` → `src/registry.rs`): generated from `ucum-essence.xml`. Every
   unit is resolved to the seven UCUM base units (m, s, g, rad, K, C, cd) with exact
   `Number` arithmetic (`build.rs` includes `src/number.rs`). Special units keep the factor
   and dimension of their corresponding proper unit and a `SpecialKind`; arbitrary units keep
   their arbitrary atoms. Case insensitive codes are indexed in `UNITS_CI`/`PREFIXES_CI`.
2. **Number** (`src/number.rs`): decimal mantissa of 28 digits with an `i32` exponent; exact
   decimal factors of any magnitude.
3. **Parser** (`src/parser.rs`): strict syntax of §1-10 (ASCII 33-126, no white space, one
   annotation, positive integers, `10*`/`10^` are atoms). Produces one owned AST, `UnitExpr`
   (`src/ast.rs`), which keeps annotations for display.
4. **Evaluator** (`src/evaluator.rs`): semantics (§16-26). `EvalResult` = factor, dimension,
   arbitrary atoms, optional `SpecialTerm` (special unit + scale α). Special units take part
   in no algebra but a numeric scale (§22). Conversion: exact affine path for proper units
   and temperatures, linear path between logarithmic/tangent/sqrt units, function path
   (`SpecialKind::forward`/`inverse`) otherwise. Commensurable = same dimension and same
   arbitrary atoms.
5. **Public API** (`src/lib.rs`), display names (`src/display.rs`), suggestions for errors
   (`src/suggestions.rs`), FHIR (`src/fhir.rs`), CLI (`src/bin/cli.rs`), WASM (`wasm/`).

### Multi-Crate Workspace Structure

- **octofhir-ucum**: Core crate, `no_std` + `alloc` without default features:
  - `std` (default): regex/fuzzy unit search, platform float math
  - `cli`: Command-line interface binary
  - `fhir`: FHIR Quantity data type integration
  - `serde`: Serialization support
- **octofhir-ucum-wasm** (`wasm/`): WebAssembly bindings, published to npm as `@octofhir/ucum-wasm` (not on crates.io)

### Important Implementation Notes

1. **Tests**: `tests/official.rs` runs the official UCUM tests through the public API with
   tolerances from the written precision; spec behaviour is tested by section in
   `tests/syntax.rs`, `tests/semantics.rs`, `tests/special_unit_conversion.rs` and
   `tests/arbitrary_units.rs`.
2. **no_std**: Core is `no_std` + `alloc`. Use `crate::prelude` for `alloc` types, `hashbrown` for maps, `spin::LazyLock` for statics, and `crate::math` for float functions (`f64::powf` etc. are not in `core`)
3. **serde for `Number`** lives in `src/types.rs`, not `src/number.rs`, because the build
   script includes `number.rs` and has no serde dependency.
