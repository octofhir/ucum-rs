# octofhir-ucum-core

UCUM (Unified Code for Units of Measure) core library for FHIRPath quantity operations, written in Rust (2024 edition).

## Features

- Parse, validate, and convert UCUM unit expressions
- SI, customary, specialized, and information units
- Prefix and factor handling
- Robust error messages
- Suitable for both embedded and server environments (`no_std` optional)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
octofhir-ucum-core = "*"
```

### Example

```rust
use octofhir_ucum_core::{evaluate, parse_expression, find_unit};

let expr = "100 kPa";
let parsed = parse_expression(expr)?;
let result = evaluate(&parsed)?;
println!("Canonical value: {} {}", result.value, result.unit);

// Lookup a unit
if let Some(unit) = find_unit("mm[Hg]") {
    println!("Unit: {} - {}", unit.code, unit.name);
}
```

## API

- `parse_expression(expr: &str)` – Parse a UCUM expression
- `evaluate(expr: &UnitExpr)` – Evaluate and canonicalize a parsed expression
- `find_unit(code: &str)` – Lookup a unit by code
- `find_prefix(sym: &str)` – Lookup a prefix by symbol

See [docs.rs](https://docs.rs/octofhir-ucum-core) for full API documentation.

## License

MIT OR Apache-2.0
