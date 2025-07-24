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
- **Extended Functionality:**
  - Unit expression optimization and simplification
  - Measurement context support for domain-specific preferences
  - Model introspection (version, units, properties)
  - Self-validation of UCUM implementation
  - Advanced conversion with precision control
  - Enhanced display names for unit codes

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

### Extended Functionality Commands

#### `optimize`

Optimize a unit expression for better readability.

```sh
octofhir-ucum optimize <EXPRESSION>
```

Examples:
```sh
# Optimize a complex expression
octofhir-ucum optimize "kg.m.s-2"
# Output: N (newton)

# Optimize power expression  
octofhir-ucum optimize "kg.m2.s-3"
# Output: W (watt)
```

#### `simplify`

Simplify a unit expression by combining like terms and reducing complexity.

```sh
octofhir-ucum simplify <EXPRESSION>
```

Examples:
```sh
# Simplify a basic expression
octofhir-ucum simplify "m.m/s"
# Output: m2/s

# Simplify complex expression
octofhir-ucum simplify "kg.m.s-2.s"
# Output: kg.m.s-1
```

#### `context`

Create and work with measurement contexts for domain-specific unit preferences.

```sh
octofhir-ucum context <DOMAIN> [OPTIONS]
```

Supported domains:
- `medical`: Medical and healthcare contexts
- `engineering`: Engineering and technical contexts  
- `physics`: Physics and scientific contexts
- `chemistry`: Chemistry and laboratory contexts
- `general`: General purpose contexts

Options:
- `--check-unit <UNIT>`: Check if unit is preferred/avoided in context
- `--suggest <UNIT>`: Get unit suggestions for the context

Examples:
```sh
# Show medical context preferences
octofhir-ucum context medical
# Output:
# Medical Context:
# Domain: Medical
# Precision: 3 significant figures, max 1% error
# Preferred units: mg, kg, L, mL, mmol, mol, Cel, etc.
# Avoided units: [stone_av], [lb_av], [gal_us], etc.

# Check if a unit is preferred in engineering context
octofhir-ucum context engineering --check-unit kPa
# Output: kPa is preferred in Engineering context: true

# Get suggestions for a unit in chemistry context
octofhir-ucum context chemistry --suggest g
# Output: Chemistry suggestions for 'g': mg, kg, mol (preferred alternatives)
```

#### `model`

Show UCUM model information.

```sh
octofhir-ucum model
```

Example:
```sh
# Display model version and statistics
octofhir-ucum model
# Output:
# UCUM Model Information:
# Version: 2.1
# Revision Date: 2017-11-21
# Total Units: 312
# Total Prefixes: 24
# Total Properties: 101
```

#### `self-validate`

Validate UCUM implementation for self-consistency.

```sh
octofhir-ucum self-validate
```

Example:
```sh
# Check implementation for issues
octofhir-ucum self-validate
# Output: UCUM implementation validation: PASSED with 0 issues
```

#### `properties`

List all available properties in the UCUM model.

```sh
octofhir-ucum properties [OPTIONS]
```

Options:
- `--limit <LIMIT>`: Limit number of properties shown

Example:
```sh
# List all properties
octofhir-ucum properties

# List first 10 properties
octofhir-ucum properties --limit 10
```

#### `validate-canonical`

Validate canonical unit forms.

```sh
octofhir-ucum validate-canonical --unit <UNIT> --canonical <CANONICAL>
```

Example:
```sh
# Check if kg is canonical form of g
octofhir-ucum validate-canonical --unit kg --canonical g
# Output: kg -> g: NOT canonical (false)

# Check if m.s-1 is canonical form of m/s
octofhir-ucum validate-canonical --unit "m/s" --canonical "m.s-1"
# Output: m/s -> m.s-1: canonical (true)
```

#### `display`

Get display name for unit codes.

```sh
octofhir-ucum display <UNIT_CODE>
```

Examples:
```sh
# Get display name for kg
octofhir-ucum display kg
# Output: kilogram

# Get display name for compound unit
octofhir-ucum display "m/s"
# Output: (meter) / (second)
```

#### `convert-advanced`

Advanced unit conversion with precision control.

```sh
octofhir-ucum convert-advanced [OPTIONS] --value <VALUE> --from <FROM_UNIT> --to <TO_UNIT>
```

Options:
- `--precision <PLACES>`: Fixed decimal places
- `--significant <DIGITS>`: Significant figures
- `--rounding <MODE>`: Rounding mode (nearest, up, down, truncate)
- `--temperature-scale <SCALE>`: Temperature scale (kelvin, celsius, fahrenheit)
- `--no-special-units`: Disable special unit handling

Examples:
```sh
# Convert with 3 decimal places
octofhir-ucum convert-advanced --value 1000 --from g --to kg --precision 3
# Output: 1000 g = 1.000 kg (3 decimal places, factor: 0.001)

# Convert temperature with Celsius scale
octofhir-ucum convert-advanced --value 100 --from Cel --to K --precision 1 --temperature-scale celsius
# Output: 100 Cel = 373.2 K (1 decimal places, offset: 273.15, used special units)

# Convert with significant figures
octofhir-ucum convert-advanced --value 2.5 --from "kg.m/s2" --to N --significant 4
# Output: 2.5 kg⋅m/s² = 2.500 N (4 significant figures, factor: 1)
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

### Using Model Introspection

```sh
# Get UCUM model information
octofhir-ucum model
# Output:
# UCUM Model Information:
# Version: 2.1
# Revision Date: 2017-11-21
# Total Units: 312
# Total Prefixes: 24
# Total Properties: 101

# Validate implementation
octofhir-ucum self-validate
# Output: UCUM implementation validation: PASSED with 0 issues

# Get enhanced display names
octofhir-ucum display kg
# Output: kilogram (handles prefixed units correctly)
```

### Using Advanced Conversion

```sh
# Convert with precise decimal control
octofhir-ucum convert-advanced --value 1000 --from g --to kg --precision 3
# Output: 1000 g = 1.000 kg (3 decimal places, factor: 0.001)

# Convert temperature with special unit handling
octofhir-ucum convert-advanced --value 0 --from Cel --to K --precision 2
# Output: 0 Cel = 273.15 K (2 decimal places, offset: 273.15, used special units)
```

## Advanced Usage

For advanced usage and detailed examples, please refer to the [UCUM-RS User Guide](https://github.com/octofhir/ucum-rs/blob/main/USER_GUIDE.md).

## License

MIT OR Apache-2.0

## See Also

- [UCUM-RS User Guide](https://github.com/octofhir/ucum-rs/blob/main/USER_GUIDE.md) - Comprehensive guide to using the UCUM-RS library
- [UCUM Specification](https://ucum.org/ucum.html) - Official UCUM specification
- [FHIR Quantity](https://www.hl7.org/fhir/datatypes.html#Quantity) - FHIR Quantity data type that uses UCUM
