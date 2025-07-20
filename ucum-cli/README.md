# octofhir-ucum-cli

Command-line interface for UCUM (Unified Code for Units of Measure) utilities in Rust.

## Features

- Validate UCUM expressions
- Convert values between units
- Print parsed ASTs
- List all supported units
- Explain unit codes
- Generate shell completions
- Perform arithmetic on unit expressions

## Installation

Install from source:

```sh
cargo install --path .
```

## Usage

```sh
# Validate an expression
octofhir-ucum validate "100 kPa"

# Convert 100 kPa to mm[Hg]
octofhir-ucum convert --value 100 --from kPa --to mm[Hg]

# List all units
octofhir-ucum list-units

# Explain a unit code
octofhir-ucum explain mm[Hg]
```

For all commands and options, run:

```sh
octofhir-ucum --help
```

## License

MIT OR Apache-2.0
