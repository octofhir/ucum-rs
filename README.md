# UCUM-RS

Unified Code for Units of Measure (UCUM) implementation in Rust 2024 edition.

## Quick Start

```sh
# Add to your project
cargo add ucum-core

# Or use the CLI
cargo install --path ucum-cli

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
| CLI tool               | ✅       | `ucum-cli` binary                      |
| FHIR integration demo  | 🚧       | Planned                                |
| WASM support           | 🚧       | Feature-gated, planned                 |
| Property-based tests   | ✅       | `proptest`                             |
| Fuzzing                | 🚧       | Planned                                |

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
   cargo run --package ucum-cli -- convert --value 1 --from m --to cm
   ```

5. **Docs:**

   ```sh
   cargo doc --open
   ```

6. **Formatting & Linting:**

   ```sh
   cargo fmt --all
   cargo clippy --all -- -D warnings
   ```

## Project Structure

- `ucum-core/` – Core library (parsing, evaluation, registry)
- `ucum-cli/`  – Command-line interface
- `spec/`      – UCUM specification assets
- `docs/`      – Book-style documentation (mdBook)

## License

MIT OR Apache-2.0
