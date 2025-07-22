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

### From crates.io

```sh
cargo install octofhir-ucum-cli
```

### From source

```sh
# Clone the repository
git clone https://github.com/octofhir/ucum-rs.git
cd ucum-rs

# Install the CLI
cargo install --path ucum-cli
```

## Usage

The CLI provides several commands for working with UCUM expressions:

```sh
octofhir-ucum [OPTIONS] <COMMAND>
```

### Global Options

- `-h, --help`: Print help information
- `-V, --version`: Print version information
- `-v, --verbose`: Enable verbose output

### Commands

#### `validate`

Validate a UCUM expression.

```sh
octofhir-ucum validate <EXPRESSION>
```

Examples:
```sh
# Validate a simple expression
octofhir-ucum validate "mg/dL"

# Validate a complex expression
octofhir-ucum validate "kg.m2/s3"

# Validate an expression with annotations
octofhir-ucum validate "mm[Hg]"
```

#### `convert`

Convert a value from one unit to another.

```sh
octofhir-ucum convert --value <VALUE> --from <FROM_UNIT> --to <TO_UNIT>
```

Examples:
```sh
# Convert 100 kPa to mm[Hg]
octofhir-ucum convert --value 100 --from kPa --to mm[Hg]

# Convert 1 kg to g
octofhir-ucum convert --value 1 --from kg --to g

# Convert 25 Celsius to Fahrenheit
octofhir-ucum convert --value 25 --from Cel --to [degF]
```

#### `list-units`

List all supported units.

```sh
octofhir-ucum list-units [OPTIONS]
```

Options:
- `--filter <FILTER>`: Filter units by property (e.g., "mass", "length", "time")
- `--format <FORMAT>`: Output format (default: "table", options: "table", "json", "csv")

Examples:
```sh
# List all units
octofhir-ucum list-units

# List all mass units
octofhir-ucum list-units --filter mass

# List all units in JSON format
octofhir-ucum list-units --format json
```

#### `explain`

Explain a unit code.

```sh
octofhir-ucum explain <UNIT_CODE>
```

Examples:
```sh
# Explain a simple unit
octofhir-ucum explain kg

# Explain a unit with annotation
octofhir-ucum explain mm[Hg]

# Explain a complex unit
octofhir-ucum explain kg.m/s2
```

#### `parse`

Parse a UCUM expression and print the AST.

```sh
octofhir-ucum parse <EXPRESSION>
```

Examples:
```sh
# Parse a simple expression
octofhir-ucum parse "mg/dL"

# Parse a complex expression
octofhir-ucum parse "kg.m2/s3"
```

#### `evaluate`

Evaluate a UCUM expression and print the result.

```sh
octofhir-ucum evaluate <EXPRESSION>
```

Examples:
```sh
# Evaluate a simple expression
octofhir-ucum evaluate "mg/dL"

# Evaluate a complex expression
octofhir-ucum evaluate "kg.m2/s3"
```

#### `arithmetic`

Perform arithmetic operations on unit expressions.

```sh
octofhir-ucum arithmetic [OPTIONS] <LEFT_UNIT> <OPERATION> <RIGHT_UNIT>
```

Options:
- `--value <VALUE>`: Value to apply to the result (default: 1.0)

Operations:
- `mul`: Multiply units
- `div`: Divide units

Examples:
```sh
# Multiply kg by m/s2 (result: kg.m/s2 = N)
octofhir-ucum arithmetic kg mul "m/s2"

# Divide J by s (result: J/s = W)
octofhir-ucum arithmetic J div s

# Multiply kg by m/s2 with value 9.8 (result: 9.8 N)
octofhir-ucum arithmetic --value 9.8 kg mul "m/s2"
```

#### `completions`

Generate shell completions.

```sh
octofhir-ucum completions <SHELL> [OUTPUT_DIR]
```

Supported shells:
- `bash`
- `zsh`
- `fish`
- `powershell`
- `elvish`

Examples:
```sh
# Generate bash completions
octofhir-ucum completions bash

# Generate zsh completions to a specific directory
octofhir-ucum completions zsh ~/.zsh/completions
```

## Examples

### Validating Units

```sh
# Check if mg/dL is a valid UCUM expression
octofhir-ucum validate "mg/dL"
# Output: Valid UCUM expression: mg/dL

# Check if invalid_unit is a valid UCUM expression
octofhir-ucum validate "invalid_unit"
# Output: Invalid UCUM expression: invalid_unit
# Error: Unrecognized unit code: invalid_unit
```

### Converting Between Units

```sh
# Convert 100 kPa to mm[Hg]
octofhir-ucum convert --value 100 --from kPa --to mm[Hg]
# Output: 100 kPa = 750.0617 mm[Hg]

# Convert 1 kg to g
octofhir-ucum convert --value 1 --from kg --to g
# Output: 1 kg = 1000 g

# Convert 25 Celsius to Fahrenheit
octofhir-ucum convert --value 25 --from Cel --to [degF]
# Output: 25 Cel = 77 [degF]
```

### Explaining Units

```sh
# Explain the kg unit
octofhir-ucum explain kg
# Output:
# Code: kg
# Name: kilogram
# Property: mass
# Dimensions: [1, 0, 0, 0, 0, 0, 0]
# Factor: 1000
# Is Metric: true
# Is Special: false
# Is Arbitrary: false
```

### Performing Arithmetic

```sh
# Multiply kg by m/s2 (result: kg.m/s2 = N)
octofhir-ucum arithmetic kg mul "m/s2"
# Output:
# Expression: kg.m/s2
# Canonical: kg.m/s2
# Dimensions: [1, 1, -2, 0, 0, 0, 0]
# Factor: 1000
# Is Special: false
# Is Arbitrary: false
# Equivalent to: N (newton)
```

## Advanced Usage

For advanced usage and detailed examples, please refer to the [UCUM-RS User Guide](https://github.com/octofhir/ucum-rs/blob/main/USER_GUIDE.md).

## License

MIT OR Apache-2.0

## See Also

- [UCUM-RS User Guide](https://github.com/octofhir/ucum-rs/blob/main/USER_GUIDE.md) - Comprehensive guide to using the UCUM-RS library
- [UCUM Specification](https://ucum.org/ucum.html) - Official UCUM specification
- [FHIR Quantity](https://www.hl7.org/fhir/datatypes.html#Quantity) - FHIR Quantity data type that uses UCUM
